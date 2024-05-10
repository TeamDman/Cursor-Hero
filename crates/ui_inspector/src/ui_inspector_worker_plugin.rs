use bevy::prelude::*;
use bevy_egui::EguiContexts;
use cursor_hero_cursor_types::cursor_click_types::Way;
use cursor_hero_ui_automation::prelude::*;
use cursor_hero_ui_inspector_types::prelude::FetchingState;
use cursor_hero_ui_inspector_types::prelude::GameboundUISnapshotMessage;
use cursor_hero_ui_inspector_types::prelude::ThreadboundUISnapshotMessage;
use cursor_hero_ui_inspector_types::prelude::UIData;
use cursor_hero_worker::prelude::anyhow::Context;
use cursor_hero_worker::prelude::anyhow::Error;
use cursor_hero_worker::prelude::anyhow::Result;
use cursor_hero_worker::prelude::Sender;
use cursor_hero_worker::prelude::WorkerConfig;
use cursor_hero_worker::prelude::WorkerPlugin;
use uiautomation::UIAutomation;
pub struct UiInspectorWorkerPlugin;

impl Plugin for UiInspectorWorkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorkerPlugin {
            config: WorkerConfig::<ThreadboundUISnapshotMessage, GameboundUISnapshotMessage, ()> {
                name: "ui_hover".to_string(),
                is_ui_automation_thread: true,
                handle_threadbound_message,
                handle_threadbound_message_error_handler,
                ..default()
            },
        });
        let visible_condition = |ui_data: Res<UIData>| ui_data.visible;

        app.add_systems(Update, handle_gamebound_messages.run_if(visible_condition));
    }
}
fn handle_threadbound_message_error_handler(
    _msg: &ThreadboundUISnapshotMessage,
    reply_tx: &Sender<GameboundUISnapshotMessage>,
    _state: &mut (),
    _error: &Error,
) -> Result<()> {
    reply_tx.send(GameboundUISnapshotMessage::Error)?;
    Ok(())
}

fn handle_threadbound_message(
    msg: &ThreadboundUISnapshotMessage,
    reply_tx: &Sender<GameboundUISnapshotMessage>,
    _state: &mut (),
) -> Result<()> {
    match msg {
        ThreadboundUISnapshotMessage::UIDataUpdate { pos } => {
            debug!("taking snapshot");
            // Find element at position
            let start = find_element_at(*pos)?;

            // Gather tree
            let gathered = gather_info_tree_ancestry_filtered(start)?;

            // Send reply
            if let Err(e) = reply_tx.send(GameboundUISnapshotMessage::UpdateUIData {
                ui_tree: gathered.ui_tree,
                start: gathered.start_info,
            }) {
                error!("Failed to send snapshot: {:?}", e);
            }
        }
        ThreadboundUISnapshotMessage::GatherChildren {
            parent_drill_id,
            parent_runtime_id,
        } => {
            debug!("fetching children for {:?}", parent_drill_id);
            // Get parent
            let automation = UIAutomation::new().context("creating automation")?;
            let walker = automation.create_tree_walker().context("creating walker")?;
            let root = automation.get_root_element().context("getting root")?;
            let parent = root
                .drill(&walker, parent_drill_id.clone())
                .context("drilling")?;

            // Get children
            let children = gather_info_children(&parent, parent_drill_id, &walker)?;

            // Send reply
            if let Err(e) = reply_tx.send(GameboundUISnapshotMessage::GatherChildrenResponse {
                drill_id: parent_drill_id.clone(),
                runtime_id: parent_runtime_id.clone(),
                children,
            }) {
                error!("Failed to send ChildrenFetchResponse: {:?}", e);
            }
        }
        ThreadboundUISnapshotMessage::TreeClipboard {
            parent_drill_id,
            parent_runtime_id,
        } => {
            debug!("fetching tree for {:?}", parent_drill_id);
            // Get parent
            let automation = UIAutomation::new().context("creating automation")?;
            let walker = automation.create_tree_walker().context("creating walker")?;
            let root = automation.get_root_element().context("getting root")?;
            let parent = root
                .drill(&walker, parent_drill_id.clone())
                .context("drilling")?;

            // Validate parent
            let id = RuntimeId(parent.get_runtime_id()?);
            if id != *parent_runtime_id {
                error!(
                    "Parent runtime_id mismatch: expected {:?}, got {:?}",
                    parent_runtime_id, id
                );
                return Ok(());
            }

            // Get tree
            let tree = gather_info_tree(parent)?;

            // Send reply
            if let Err(e) =
                reply_tx.send(GameboundUISnapshotMessage::TreeClipboardResponse { ui_tree: tree })
            {
                error!("Failed to send snapshot: {:?}", e);
            }
        }
        ThreadboundUISnapshotMessage::Click { drill_id, way } => {
            let automation = UIAutomation::new().context("creating automation")?;
            let walker = automation.create_tree_walker().context("creating walker")?;
            let root = automation.get_root_element().context("getting root")?;
            let element = root.drill(&walker, drill_id.clone()).context("drilling")?;
            match way {
                Way::Left => element.click().context("left click")?,
                Way::Right => element.right_click().context("right click")?,
                Way::Middle => warn!("Middle click not supported"),
            }
        }
    }
    Ok(())
}

fn handle_gamebound_messages(
    mut snapshot: EventReader<GameboundUISnapshotMessage>,
    mut ui_data: ResMut<UIData>,
    mut contexts: EguiContexts,
) {
    for msg in snapshot.read() {
        match msg {
            GameboundUISnapshotMessage::Error => {
                ui_data.in_flight = false;
            }
            GameboundUISnapshotMessage::UpdateUIData { ui_tree, start } => {
                ui_data.in_flight = false;
                ui_data.ui_tree = ui_tree.clone();
                ui_data.start = start.clone();
                ui_data.selected = Some(start.drill_id.clone());
                ui_data.expanded = ui_tree
                    .get_descendents()
                    .iter()
                    .chain(std::iter::once(&ui_tree))
                    .filter(|x| x.children.is_some())
                    .map(|x| x.drill_id.clone())
                    .collect();
                ui_data.fresh = true;
                debug!("Received snapshot");
            }
            GameboundUISnapshotMessage::GatherChildrenResponse {
                drill_id,
                runtime_id,
                children,
            } => {
                let key = (drill_id.clone(), runtime_id.clone());
                if let Some(FetchingState::FetchDispatched) = ui_data.fetching.get(&key) {
                    ui_data
                        .fetching
                        .insert(key, FetchingState::Fetched(children.clone()));
                }
            }
            GameboundUISnapshotMessage::TreeClipboardResponse { ui_tree } => {
                contexts.ctx_mut().output_mut(|out| {
                    out.copied_text = format!("{:#?}", ui_tree);
                });
                debug!("Received snapshot");
            }
        }
    }
}
