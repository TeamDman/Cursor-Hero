#![feature(let_chains, trivial_bounds)]
use std::collections::VecDeque;

use bevy::input::common_conditions::input_toggle_active;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use bevy_egui::egui;
use bevy_egui::egui::Align2;
use bevy_egui::EguiContexts;
use bevy_egui::EguiSet;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::reflect_inspector::Context;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use cursor_hero_memory::primary_window_memory_plugin::PrimaryWindowMemoryPlugin;
use cursor_hero_ui_automation::prelude::*;
use cursor_hero_winutils::win_mouse::get_cursor_position;
use cursor_hero_worker::prelude::Message;
use cursor_hero_worker::prelude::Sender;
use cursor_hero_worker::prelude::WorkerConfig;
use cursor_hero_worker::prelude::WorkerPlugin;
use itertools::Itertools;
use uiautomation::UIAutomation;
fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(LogPlugin {
                level: bevy::log::Level::DEBUG,
                filter: "
info,
wgpu_core=warn,
wgpu_hal=warn,
ui_hover_example=trace,
cursor_hero_worker=debug,
"
                .replace('\n', "")
                .trim()
                .into(),
            })
            .build(),
    );
    app.add_plugins(WorkerPlugin {
        config: WorkerConfig::<ThreadboundUISnapshotMessage, GameboundUISnapshotMessage> {
            name: "ui_hover".to_string(),
            is_ui_automation_thread: true,
            handle_threadbound_message: handle_threadbound_message,
            ..default()
        },
    });
    app.add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
    );
    app.add_plugins(PrimaryWindowMemoryPlugin);
    app.insert_resource(ClearColor(Color::rgb(0.992, 0.714, 0.69)));
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, periodic_snapshot);
    app.add_systems(Update, fetch_requested);
    app.add_systems(Update, receive);
    app.add_systems(Update, gui.after(EguiSet::InitContexts));
    app.init_resource::<UIData>();
    app.register_type::<UIData>();
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Reflect, Clone, Event)]
enum ThreadboundUISnapshotMessage {
    CaptureHovered,
    ChildrenFetchRequest {
        drill_id: DrillId,
        runtime_id: RuntimeId,
    },
}
impl Message for ThreadboundUISnapshotMessage {}

#[derive(Debug, Reflect, Clone, Event)]
enum GameboundUISnapshotMessage {
    Hovered {
        ui_tree: ElementInfo,
        start: ElementInfo,
        hovered: ElementInfo,
    },
    ChildrenFetchResponse {
        drill_id: DrillId,
        runtime_id: RuntimeId,
        children: Vec<ElementInfo>,
    },
}
impl Message for GameboundUISnapshotMessage {}

#[derive(Debug, Reflect)]
enum FetchingState {
    FetchRequest,
    FetchDispatched,
    Fetched(Vec<ElementInfo>),
}

#[derive(Resource, Debug, Reflect, Default)]
#[reflect(Resource)]
struct UIData {
    pub start: ElementInfo,
    pub hovered: ElementInfo,
    pub ui_tree: ElementInfo,
    pub selected: Option<DrillId>,
    pub expanded: Vec<DrillId>,
    pub fresh: bool,
    pub in_flight: bool,
    pub paused: bool,
    // Include runtime id in case tree changes and we quickly fetch something with the same drill_id before the first request comes back
    pub fetching: HashMap<(DrillId, RuntimeId), FetchingState>,
}

fn handle_threadbound_message(
    msg: &ThreadboundUISnapshotMessage,
    reply_tx: &Sender<GameboundUISnapshotMessage>,
) -> Result<(), Box<dyn std::error::Error>> {
    match msg {
        ThreadboundUISnapshotMessage::CaptureHovered => {
            debug!("taking snapshot");
            let cursor_pos = get_cursor_position()?;
            let hovered = find_element_at(cursor_pos)?;
            let hovered_info = gather_single_element_info(&hovered)?;
            let gathered = gather_incomplete_ui_tree_starting_deep(hovered)?;
            if let Err(e) = reply_tx.send(GameboundUISnapshotMessage::Hovered {
                ui_tree: gathered.ui_tree,
                start: gathered.start_info,
                hovered: hovered_info,
            }) {
                error!("Failed to send snapshot: {:?}", e);
            }
        }
        ThreadboundUISnapshotMessage::ChildrenFetchRequest {
            drill_id,
            runtime_id,
        } => {
            debug!("fetching children for {:?}", drill_id);
            let automation = UIAutomation::new()?;
            let walker = automation.create_tree_walker()?;
            let root = automation.get_root_element()?;
            let found = root.drill(&walker, drill_id.clone())?;
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

fn periodic_snapshot(
    mut data: ResMut<UIData>,
    mut cooldown: Local<Option<Timer>>,
    time: Res<Time>,
    mut events: EventWriter<ThreadboundUISnapshotMessage>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };
    if window.cursor_position().is_some() {
        return;
    }
    let cooldown_over = if let Some(cooldown) = cooldown.as_mut() {
        if cooldown.tick(time.delta()).just_finished() {
            cooldown.reset();
            true
        } else {
            false
        }
    } else {
        cooldown.replace(Timer::from_seconds(0.5, TimerMode::Repeating));
        true
    };
    if !cooldown_over {
        return;
    }

    if data.paused {
        return;
    }

    if data.in_flight {
        warn!("Too fast!");
        return;
    }

    events.send(ThreadboundUISnapshotMessage::CaptureHovered);
    data.in_flight = true;
}

fn receive(mut snapshot: EventReader<GameboundUISnapshotMessage>, mut ui_data: ResMut<UIData>) {
    for msg in snapshot.read() {
        match msg {
            GameboundUISnapshotMessage::Hovered {
                ui_tree,
                start,
                hovered,
            } => {
                ui_data.in_flight = false;
                ui_data.ui_tree = ui_tree.clone();
                ui_data.start = start.clone();
                ui_data.hovered = hovered.clone();
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
) {
    let ctx = contexts.ctx_mut();

    let mut cx = Context {
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
                        let id = id.with(ui_data.ui_tree.runtime_id.clone());
                        let mut elem = ui_data.ui_tree.clone();
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
                if let Some(id) = id
                    && let Some(x) = ui_data.ui_tree.lookup_drill_id_mut(id)
                {
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
                }
                // inspector.ui_for_reflect_readonly(&data, ui);
            });
        });

    let id = egui::Id::new("Paused");
    egui::Window::new("Paused")
        .id(id)
        .title_bar(false)
        .anchor(Align2::RIGHT_TOP, (5.0, 5.0))
        .show(ctx, |ui| {
            ui.checkbox(&mut ui_data.paused, "Paused");
        });
    ui_data.fresh = false;
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
    if expando.is_open() && element_info.children.is_none() {
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
    expando
        .show_header(ui, |ui| {
            let mut selected = data.selected == Some(element_info.drill_id.clone());
            if selected && data.fresh {
                ui.scroll_to_cursor(Some(egui::Align::Center));
            }
            if ui
                .toggle_value(
                    &mut selected,
                    format!(
                        "{:?} | {}",
                        element_info.name, element_info.localized_control_type
                    ),
                )
                .changed()
            {
                data.selected = if selected {
                    Some(element_info.drill_id.clone())
                } else {
                    None
                };
            };
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
            }
        });
}
