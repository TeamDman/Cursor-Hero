use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::EguiContexts;
use bevy_egui::EguiUserTextures;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use cursor_hero_bevy::prelude::TopLeftI;
use cursor_hero_bevy::prelude::TranslateIVec2;
use cursor_hero_screen::get_image::get_image;
use cursor_hero_screen::get_image::ScreensToImageParam;
use cursor_hero_ui_automation::prelude::*;
use cursor_hero_ui_hover_types::prelude::GameHoverIndicator;
use cursor_hero_ui_hover_types::prelude::HostHoverIndicator;
use cursor_hero_ui_hover_types::prelude::HoverInfo;
use cursor_hero_ui_hover_types::prelude::InspectorHoverIndicator;
use cursor_hero_ui_inspector_types::prelude::FetchingState;
use cursor_hero_ui_inspector_types::prelude::InspectorEvent;
use cursor_hero_ui_inspector_types::prelude::PreviewImage;
use cursor_hero_ui_inspector_types::prelude::UIData;
use cursor_hero_worker::prelude::anyhow::Context;
use cursor_hero_worker::prelude::anyhow::Error;
use cursor_hero_worker::prelude::anyhow::Result;
use cursor_hero_worker::prelude::Sender;
use cursor_hero_worker::prelude::WorkerConfig;
use cursor_hero_worker::prelude::WorkerMessage;
use cursor_hero_worker::prelude::WorkerPlugin;
use image::DynamicImage;
use image::ImageBuffer;
use image::Rgba;
use image::RgbaImage;
use itertools::Itertools;
use std::time::Duration;
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
        app.add_systems(Update, handle_gamebound_messages.run_if(condition.clone()));
        app.add_systems(Update, gui.run_if(condition.clone()));
        app.add_systems(Update, handle_inspector_events.run_if(condition.clone()));
        app.add_systems(Update, update_preview_image.run_if(condition));
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
    mut ui_data: ResMut<UIData>,
    mut cooldown: Local<Option<Timer>>,
    time: Res<Time>,
    mut events: EventWriter<ThreadboundUISnapshotMessage>,
    game_hover_query: Query<&GameHoverIndicator>,
    host_hover_query: Query<&HostHoverIndicator>,
) {
    // Do nothing if paused
    if ui_data.paused {
        return;
    }

    // Get position of cursor
    let pos = match (game_hover_query.get_single(), host_hover_query.get_single()) {
        (Ok(GameHoverIndicator { cursor_pos, .. }), _) => *cursor_pos,
        (_, Ok(HostHoverIndicator { cursor_pos, .. })) => *cursor_pos,
        _ => return,
    };

    // Update selected based on the deepest matching cached element
    ui_data.selected = ui_data
        .ui_tree
        .get_descendents()
        .into_iter()
        .filter(|info| info.bounding_rect.contains(pos))
        .min_by_key(|info| info.bounding_rect.size().length_squared())
        .map(|info| info.drill_id.clone());

    // Do nothing if already waiting for a response
    if ui_data.in_flight {
        return;
    }

    // Do nothing if on cooldown
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

    // Send snapshot request
    events.send(ThreadboundUISnapshotMessage::CapturePartialTreeAt { pos });
    ui_data.in_flight = true;
}

fn handle_gamebound_messages(
    mut snapshot: EventReader<GameboundUISnapshotMessage>,
    mut ui_data: ResMut<UIData>,
) {
    for msg in snapshot.read() {
        match msg {
            GameboundUISnapshotMessage::Error => {
                ui_data.in_flight = false;
            }
            GameboundUISnapshotMessage::PartialTree { ui_tree, start } => {
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

fn handle_inspector_events(
    mut inspector_events: EventReader<InspectorEvent>,
    mut ui_data: ResMut<UIData>,
) {
    for event in inspector_events.read() {
        let InspectorEvent::PushScratchPad = event;

        // get selected info
        let Some(selected_drill_id) = ui_data.selected.clone() else {
            return;
        };
        let Some(info) = ui_data.ui_tree.lookup_drill_id_mut(selected_drill_id) else {
            return;
        };

        // get identifier
        fn as_rust_identifier(info: &ElementInfo) -> String {
            format!(
                "{}_{}",
                info.name.replace(" ", "_").to_lowercase(),
                info.class_name.to_lowercase()
            )
        }
        let identifier = as_rust_identifier(info);

        // get drill id
        let drill_id = match info.drill_id {
            DrillId::Child(ref inner) => inner.iter().map(|x| x.to_string()).join(", "),
            _ => "".to_string(),
        };

        // build content
        let content = format!(
            "let {} = root.drill(&walker, vec![{}]?.try_into()?;\n",
            identifier, drill_id
        );

        // append to scratch pad
        ui_data.scratch_pad.push_str(content.as_str());
    }
}

fn update_preview_image(
    screen_access: ScreensToImageParam,
    asset_server: Res<AssetServer>,
    mut ui_data: ResMut<UIData>,
    mut debounce: Local<Option<DrillId>>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
) {
    // Avoid duplicate work
    if *debounce == ui_data.selected {
        return;
    }

    // Get the drill_id of the parent of selected
    let Some(DrillId::Child(ref inner)) = ui_data.selected else {
        ui_data.selected_preview = None;
        return;
    };
    let mut parent_drill_id = inner.clone();
    parent_drill_id.pop_back();

    // Find the parent in the cache
    let Some(selected_parent) = ui_data
        .ui_tree
        .lookup_drill_id(DrillId::Child(parent_drill_id))
    else {
        return;
    };

    // Get the selected element
    let Some(last) = inner.back() else {
        return;
    };
    let Some(selected) = selected_parent.lookup_drill_id([last].into_iter().cloned().collect())
    else {
        return;
    };

    // Get the texture of the element
    let texture_region = selected_parent.bounding_rect;
    let Ok(image) = get_image(texture_region, &screen_access) else {
        return;
    };

    // Get the size of the texture
    let size = image.size();

    // Convert to an image buffer for manipulation
    let Some(mut image) = ImageBuffer::from_raw(size.x, size.y, image.data) as Option<RgbaImage> else {
        return;
    };

    // Calculate region to highlight
    let parent_region = selected_parent.bounding_rect;
    let highlight_region = selected.bounding_rect.translate(&-parent_region.top_left());

    // Apply the highlight
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if highlight_region.contains(IVec2::new(x as i32, y as i32)) {
            *pixel = Rgba([pixel.0[0].saturating_add(50), pixel.0[1].saturating_add(50), pixel.0[2], pixel.0[3]]);
        }
    }

    // Convert back to Bevy image
    let image = Image::from_dynamic(DynamicImage::ImageRgba8(image), true);

    // Register the handle with egui
    let handle = asset_server.add(image);
    egui_user_textures.add_image(handle.clone());
    ui_data.selected_preview = Some(PreviewImage { handle, size });

    // Track work
    *debounce = ui_data.selected.clone();
}

fn gui(
    mut contexts: EguiContexts,
    mut ui_data: ResMut<UIData>,
    type_registry: Res<AppTypeRegistry>,
    mut hover_info: ResMut<HoverInfo>,
) {
    // Get preview image
    let preview = if let Some(ref preview) = ui_data.selected_preview
        && let Some(texture_id) = contexts.image_id(&preview.handle)
    {
        let size = preview.size.as_vec2().normalize() * 400.0;
        Some((texture_id, (size.x, size.y)))
    } else {
        None
    };

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
                        let id = id.with(ui_data.ui_tree.runtime_id.clone());
                        let mut elem = ui_data.ui_tree.clone();

                        // resets each frame before being set when drawing expandos
                        ui_data.hovered = None;

                        ui_for_element_info(id, ui, &mut ui_data, &mut elem, &mut inspector);
                        ui_data.ui_tree = elem;
                        ui.allocate_space(ui.available_size());
                    });
                });

            egui::TopBottomPanel::bottom(id.with("invisible panel to make things work"))
                .show_separator_line(false)
                .show_inside(ui, |_ui| {});

            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Properties");
                });
                let Some(selected_drill_id) = ui_data.selected.clone() else {
                    return;
                };
                let found = ui_data
                    .ui_tree
                    .lookup_drill_id_mut(selected_drill_id.clone());
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

                ui.vertical_centered(|ui| {
                    ui.heading("Preview");
                });

                if let Some((texture_id, size)) = preview {
                    ui.image(egui::load::SizedTexture::new(texture_id, size));
                }

                ui.vertical_centered(|ui| {
                    ui.heading("Scratch Pad");
                });
                if ui.button("clear").clicked() {
                    ui_data.scratch_pad.clear();
                }
                egui::TextEdit::multiline(&mut ui_data.scratch_pad)
                    .desired_width(ui.available_width())
                    .show(ui);
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

    hover_info.inspector_hover_indicator = ui_data
        .hovered
        .as_ref()
        .map(|elem| InspectorHoverIndicator { info: elem.clone() });
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
