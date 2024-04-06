use std::time::Duration;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::EguiContexts;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use cursor_hero_ui_automation::prelude::*;
use cursor_hero_ui_hover_types::prelude::GameHoverIndicator;
use cursor_hero_ui_hover_types::prelude::HostHoverIndicator;
use cursor_hero_ui_hover_types::prelude::HoverInfo;
use cursor_hero_ui_hover_types::prelude::InspectorHoverIndicator;
use cursor_hero_ui_inspector_types::prelude::FetchingState;
use cursor_hero_ui_inspector_types::prelude::UIData;
use cursor_hero_worker::prelude::anyhow::Context;
use cursor_hero_worker::prelude::anyhow::Error;
use cursor_hero_worker::prelude::anyhow::Result;
use cursor_hero_worker::prelude::Sender;
use cursor_hero_worker::prelude::WorkerConfig;
use cursor_hero_worker::prelude::WorkerMessage;
use cursor_hero_worker::prelude::WorkerPlugin;
use itertools::Itertools;
use uiautomation::UIAutomation;

pub struct UiInspectorPlugin;

impl Plugin for UiInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorkerPlugin {
            config: WorkerConfig::<ThreadboundUISnapshotMessage, GameboundUISnapshotMessage, ()> {
                name: "ui_hover".to_string(),
                is_ui_automation_thread: true,
                handle_threadbound_message: handle_threadbound_message,
                handle_threadbound_message_error_handler: handle_threadbound_message_error_handler,
                ..default()
            },
        });
        let condition = input_toggle_active(false, KeyCode::Grave);
        app.add_systems(
            Update,
            trigger_tree_update_for_hovered.run_if(condition.clone()),
        );
        app.add_systems(Update, fetch_requested.run_if(condition.clone()));
        app.add_systems(Update, receive.run_if(condition.clone()));
        app.add_systems(Update, gui.run_if(condition));
        // app.add_systems(Update, gui.after(EguiSet::InitContexts));
    }
}

#[derive(Debug, Reflect, Clone, Event)]
enum ThreadboundUISnapshotMessage {
    CapturePartialTreeAt {
        pos: IVec2,
    },
    ChildrenFetchRequest {
        drill_id: DrillId,
        runtime_id: RuntimeId,
    },
}
impl WorkerMessage for ThreadboundUISnapshotMessage {}

#[derive(Debug, Reflect, Clone, Event)]
enum GameboundUISnapshotMessage {
    PartialTree {
        ui_tree: ElementInfo,
        start: ElementInfo,
    },
    ChildrenFetchResponse {
        drill_id: DrillId,
        runtime_id: RuntimeId,
        children: Vec<ElementInfo>,
    },
    Error,
}
impl WorkerMessage for GameboundUISnapshotMessage {}

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
        ThreadboundUISnapshotMessage::CapturePartialTreeAt { pos } => {
            debug!("taking snapshot");
            let start = find_element_at(*pos)?;
            let gathered = gather_incomplete_ui_tree_starting_deep(start)?;
            if let Err(e) = reply_tx.send(GameboundUISnapshotMessage::PartialTree {
                ui_tree: gathered.ui_tree,
                start: gathered.start_info,
            }) {
                error!("Failed to send snapshot: {:?}", e);
            }
        }
        ThreadboundUISnapshotMessage::ChildrenFetchRequest {
            drill_id,
            runtime_id,
        } => {
            debug!("fetching children for {:?}", drill_id);
            let automation = UIAutomation::new().context("creating automation")?;
            let walker = automation.create_tree_walker().context("creating walker")?;
            let root = automation.get_root_element().context("getting root")?;
            let found = root.drill(&walker, drill_id.clone()).context("drilling")?;
            let mut children = found
                .gather_children(&walker, &StopBehaviour::EndOfSiblings)
                .into_iter()
                .enumerate()
                .filter_map(|(i, child)| {
                    gather_single_element_info(&child)
                        .ok()
                        .map(|mut child_info| {
                            child_info.drill_id = DrillId::Child(vec![i].into_iter().collect());
                            child_info
                        })
                })
                .collect_vec();

            update_drill_ids(Some(&mut children), &drill_id);
            if let Err(e) = reply_tx.send(GameboundUISnapshotMessage::ChildrenFetchResponse {
                drill_id: drill_id.clone(),
                runtime_id: runtime_id.clone(),
                children,
            }) {
                error!("Failed to send ChildrenFetchResponse: {:?}", e);
            }
        }
    }
    Ok(())
}

fn fetch_requested(
    mut data: ResMut<UIData>,
    mut events: EventWriter<ThreadboundUISnapshotMessage>,
) {
    for (key, state) in data.fetching.iter_mut() {
        if let FetchingState::FetchRequest = state {
            *state = FetchingState::FetchDispatched;
            events.send(ThreadboundUISnapshotMessage::ChildrenFetchRequest {
                drill_id: key.0.clone(),
                runtime_id: key.1.clone(),
            });
        }
    }
}

fn trigger_tree_update_for_hovered(
    mut data: ResMut<UIData>,
    mut cooldown: Local<Option<Timer>>,
    time: Res<Time>,
    mut events: EventWriter<ThreadboundUISnapshotMessage>,
    game_hover_query: Query<&GameHoverIndicator>,
    host_hover_query: Query<&HostHoverIndicator>,
) {
    // Check cooldown
    let default_duration = Duration::from_secs_f32(0.5);
    let Some(cooldown) = cooldown.as_mut() else {
        cooldown.replace(Timer::new(default_duration, TimerMode::Repeating));
        return;
    };
    if cooldown.tick(time.delta()).just_finished() {
        cooldown.reset();
    } else {
        return;
    }

    // Check other conditions
    if data.paused {
        return;
    }
    if data.in_flight {
        return;
    }
    let pos = match (game_hover_query.get_single(), host_hover_query.get_single()) {
        (Ok(GameHoverIndicator { info, .. }), _) => info.bounding_rect.center(),
        (_, Ok(HostHoverIndicator { info, .. })) => info.bounding_rect.center(),
        _ => return,
    };

    // Send snapshot request
    events.send(ThreadboundUISnapshotMessage::CapturePartialTreeAt { pos });
    data.in_flight = true;
}

fn receive(mut snapshot: EventReader<GameboundUISnapshotMessage>, mut ui_data: ResMut<UIData>) {
    for msg in snapshot.read() {
        match msg {
            GameboundUISnapshotMessage::Error => {
                ui_data.in_flight = false;
            }
            GameboundUISnapshotMessage::PartialTree {
                ui_tree,
                start,
            } => {
                ui_data.in_flight = false;
                ui_data.ui_tree = ui_tree.clone();
                ui_data.start = start.clone();
                ui_data.selected = Some(start.drill_id.clone());
                ui_data.expanded = ui_tree
                    .get_descendents()
                    .iter()
                    .chain([ui_tree].iter())
                    .filter(|x| x.children.is_some())
                    .map(|x| x.drill_id.clone())
                    .collect();
                ui_data.fresh = true;
                debug!("Received snapshot");
            }
            GameboundUISnapshotMessage::ChildrenFetchResponse {
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
        }
    }
}

fn gui(
    mut contexts: EguiContexts,
    mut ui_data: ResMut<UIData>,
    type_registry: Res<AppTypeRegistry>,
    mut hover_info: ResMut<HoverInfo>,
) {
    let ctx = contexts.ctx_mut();

    let mut cx = bevy_inspector_egui::reflect_inspector::Context {
        world: None,
        queue: None,
    };

    let type_registry = type_registry.0.clone();
    let type_registry = type_registry.read();
    let mut inspector = InspectorUi::for_bevy(&type_registry, &mut cx);

    let id = egui::Id::new("Inspector");
    egui::Window::new("Inspector")
        .title_bar(false)
        .id(id)
        .default_pos((5.0, 5.0))
        .default_width(1200.0)
        .default_height(1000.0)
        .show(ctx, |ui| {
            egui::SidePanel::left(id.with("tree"))
                .resizable(true)
                .width_range(100.0..=4000.0)
                .default_width(600.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("UI Tree");
                    });
                    egui::ScrollArea::both().show(ui, |ui| {
                        let id = id.with(ui_data.ui_tree.runtim○e_id.clone());
                        let mut elem = ui_data.ui_tree.clone();
                        
                        // resets each frame before being set when drawing expandos
                        ui_data.hovered = None;

                        ui_for_element_info(id, ui, &mut ui_data, &mut elem, &mut inspector);
                        ui_data.ui_tree = elem;
                        ui.allocate_space(ui.available_size());
                    });
                });

            egui::TopBottomPanel::bottom(id.with("invisible bottom panel"))
                .show_separator_line(false)
                .show_inside(ui, |_| ());

            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Properties");
                });
                let id = ui_data.selected.clone();
                let Some(id) = id else {
                    return;
                };
                let found = ui_data.ui_tree.lookup_drill_id_mut(id);
                // debug!("found {:?}", found);
                let Some(x) = found else {
                    return;
                };
                inspector.ui_for_reflect_readonly(x, ui);
                ui.separator();
                ui.label("drill_id");
                let drill_id = x.drill_id.to_string();
                inspector.ui_for_reflect_readonly(&drill_id, ui);
                if ui.button("copy").clicked() {
                    ui.output_mut(|out| {
                        out.copied_text = drill_id.clone();
                    });
                    info!("Copied drill_id {} to clipboard", drill_id);
                }
                ui.label("runtime_id");
                let runtime_id = x.runtime_id.to_string();
                inspector.ui_for_reflect_readonly(&runtime_id, ui);
                if ui.button("copy").clicked() {
                    ui.output_mut(|out| {
                        out.copied_text = runtime_id.clone();
                    });
                    info!("Copied runtime_id {} to clipboard", runtime_id);
                }
                // inspector.ui_for_reflect_readonly(&data, ui);
            });
        });

    let id = egui::Id::new("Paused");
    egui::Window::new("Paused")
        .id(id)
        .title_bar(false)
        .default_pos((ctx.screen_rect().max.x - 200.0, 5.0))
        .show(ctx, |ui| {
            ui.checkbox(&mut ui_data.paused, "Paused");
        });
    ui_data.fresh = false;

    hover_info.inspector_element = ui_data.hovered.as_ref().map(|elem| InspectorHoverIndicator {
        info: elem.clone(),
    });
}

#[allow(clippy::too_many_arguments)]
fn ui_for_element_info(
    id: egui::Id,
    ui: &mut egui::Ui,
    data: &mut UIData,
    element_info: &mut ElementInfo,
    _inspector: &mut InspectorUi,
) {
    let default_open = data.expanded.contains(&element_info.drill_id);
    let mut expando = egui::collapsing_header::CollapsingState::load_with_default_open(
        ui.ctx(),
        id,
        default_open,
    );
    if data.fresh {
        expando.set_open(default_open);
        data.fetching.clear();
    }
    let expando_is_open = expando.is_open();
    expando
        .show_header(ui, |ui| {
            let mut selected = data.selected == Some(element_info.drill_id.clone());
            if selected && data.fresh {
                ui.scroll_to_cursor(Some(egui::Align::Center));
            }
            let label = if element_info.automation_id.is_empty() {
                format!(
                    "{:?} | {} | {}",
                    element_info.name, element_info.localized_control_type, element_info.drill_id,
                )
            } else {
                format!(
                    "{:?} | {} | {} | {}",
                    element_info.name,
                    element_info.localized_control_type,
                    element_info.automation_id,
                    element_info.drill_id,
                )
            };
            let toggle = ui.toggle_value(&mut selected, label);
            if toggle.changed() {
                data.selected = if selected {
                    Some(element_info.drill_id.clone())
                } else {
                    None
                };
            };
            if toggle.hovered() {
                data.hovered = Some(element_info.clone());
            }
        })
        .body(|ui| {
            if let Some(ref mut children) = element_info.children {
                for child in children.iter_mut() {
                    ui_for_element_info(
                        id.with(child.runtime_id.clone()),
                        ui,
                        data,
                        child,
                        _inspector,
                    );
                }
            } else if expando_is_open {
                let key = (
                    element_info.drill_id.clone(),
                    element_info.runtime_id.clone(),
                );
                let found = data.fetching.get_mut(&key);
                if !found.is_some() {
                    data.fetching.insert(key, FetchingState::FetchRequest);
                } else if let Some(FetchingState::Fetched(ref mut children)) = found {
                    element_info.children = Some(std::mem::take(children));
                    data.fetching.remove(&key);
                } else {
                    ui.label("fetching...");
                }
            }
        });
}
