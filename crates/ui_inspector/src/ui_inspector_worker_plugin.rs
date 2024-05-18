use bevy::prelude::*;
use bevy_egui::EguiContexts;
use cursor_hero_cursor_types::cursor_click_types::Way;
use cursor_hero_ui_automation::prelude::*;
use cursor_hero_ui_inspector_types::prelude::FetchingState;
use cursor_hero_ui_inspector_types::prelude::GameboundUISnapshotMessage;
use cursor_hero_ui_inspector_types::prelude::ThreadboundUISnapshotMessage;
use cursor_hero_ui_inspector_types::prelude::UIData;
use cursor_hero_winutils::win_mouse::get_host_cursor_position;
use cursor_hero_winutils::win_mouse::set_host_cursor_position;
use cursor_hero_worker::prelude::anyhow::Context;
use cursor_hero_worker::prelude::anyhow::Error;
use cursor_hero_worker::prelude::anyhow::Result;
use cursor_hero_worker::prelude::Sender;
use cursor_hero_worker::prelude::WorkerConfig;
use cursor_hero_worker::prelude::WorkerPlugin;
use uiautomation::inputs::Mouse;
use uiautomation::UIAutomation;
pub struct UiInspectorWorkerPlugin;

impl Plugin for UiInspectorWorkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorkerPlugin {
            config: WorkerConfig::<
                ThreadboundUISnapshotMessage,
                GameboundUISnapshotMessage,
                (),
                _,
                _,
                _,
            > {
                name: "ui_hover".to_string(),
                is_ui_automation_thread: true,
                handle_threadbound_message,
                handle_threadbound_message_error_handler,
                ..default()
            },
        });
        app.add_systems(
            Update,
            handle_gamebound_messages.run_if(|ui_data: Res<UIData>| ui_data.windows.global_toggle),
        );
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
    debug!("Handling {msg:?}");
    match msg {
        ThreadboundUISnapshotMessage::TreeUpdate { pos } => {
            // Find element at position
            let start = find_element_at(*pos)?;

            // Gather tree
            let gathered = gather_info_tree_ancestry_filtered(start)?;

            // Send reply
            if let Err(e) = reply_tx.send(GameboundUISnapshotMessage::SetUITree {
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
        ThreadboundUISnapshotMessage::TreePatch {
            parent_drill_id,
            parent_runtime_id,
        } => {
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
            debug!("Gathering tree");
            let tree = gather_info_tree(parent)?;

            // Update data
            debug!("Sending patch");
            if let Err(e) = reply_tx.send(GameboundUISnapshotMessage::PatchUITree { patch: tree }) {
                error!("Failed to send patch: {:?}", e);
            }
        }
        ThreadboundUISnapshotMessage::ClickPos { pos, way } => {
            let mouse = Mouse::new().auto_move(false);
            // Mark current position, teleport the mouse, click, then teleport back
            let restore_point = get_host_cursor_position()?;
            set_host_cursor_position(*pos)?;
            match way {
                Way::Left => mouse.click(pos.to_ui_point())?,
                Way::Right => mouse.right_click(pos.to_ui_point())?,
                Way::Middle => {
                    warn!("Middle click not supported")
                }
            }
            // sleep(Duration::from_millis(500));
            set_host_cursor_position(restore_point)?;
        }
        ThreadboundUISnapshotMessage::Click { drill_id, way } => {
            let automation = UIAutomation::new().context("creating automation")?;
            let walker = automation.create_tree_walker().context("creating walker")?;
            let root = automation.get_root_element().context("getting root")?;
            let element = root.drill(&walker, drill_id.clone()).context("drilling")?;

            let click_point = match element.get_clickable_point() {
                Ok(Some(point)) => point,
                Err(_) | Ok(None) => match element.get_bounding_rectangle() {
                    Ok(rect) => {
                        debug!("No clickable point found, using bounding rectangle");
                        let bevy_rect = rect.to_bevy_irect();
                        let center = bevy_rect.center();
                        center.to_ui_point()
                    }
                    _ => {
                        warn!("No clickable point or bounding rectangle found for {element}");
                        return Ok(());
                    }
                },
            };

            let mouse = Mouse::new().auto_move(false);
            info!("Clicking {drill_id} at {:?}", click_point);
            // Mark current position, teleport the mouse, click, then teleport back
            let restore_point = get_host_cursor_position()?;
            set_host_cursor_position(click_point.to_bevy_ivec2())?;
            match way {
                Way::Left => mouse.click(click_point)?,
                Way::Right => mouse.right_click(click_point)?,
                Way::Middle => {
                    warn!("Middle click not supported")
                }
            }
            // sleep(Duration::from_millis(500));
            set_host_cursor_position(restore_point)?;
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
            GameboundUISnapshotMessage::SetUITree { ui_tree, start } => {
                ui_data.in_flight = false;
                ui_data.tree = ui_tree.clone();
                ui_data.start = start.clone();
                ui_data.selected = Some(start.drill_id.clone());
                ui_data.default_expanded = ui_tree
                    .get_descendents()
                    .iter()
                    .chain(std::iter::once(&ui_tree))
                    .filter(|x| x.children.is_some())
                    .map(|x| x.drill_id.clone())
                    .collect();
                ui_data.tree_is_fresh = true;
                debug!("Received snapshot");
            }
            GameboundUISnapshotMessage::PatchUITree { patch } => {
                ui_data.in_flight = false;
                debug!("Applying tree patch");
                let Some(elem) = ui_data.tree.lookup_drill_id_mut(patch.drill_id.clone()) else {
                    warn!("Patch drill_id not found: {}", patch.drill_id);
                    continue;
                };
                *elem = patch.clone();
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
