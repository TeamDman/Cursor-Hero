use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui;
use bevy_egui::EguiContext;
use bevy_egui::EguiContexts;
use bevy_egui::EguiUserTextures;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use cursor_hero_bevy::prelude::Area;
use cursor_hero_bevy::prelude::TopLeftI;
use cursor_hero_bevy::prelude::TranslateIVec2;
use cursor_hero_cursor_types::prelude::ClickEvent;
use cursor_hero_cursor_types::prelude::Way;
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
                handle_threadbound_message,
                handle_threadbound_message_error_handler,
                ..default()
            },
        });
        let condition = input_toggle_active(false, KeyCode::Grave);
        app.add_systems(
            Update,
            trigger_tree_update_for_hovered.run_if(condition.clone()),
        );
        app.add_systems(
            Update,
            trigger_gather_children_request.run_if(condition.clone()),
        );
        app.add_systems(Update, handle_gamebound_messages.run_if(condition.clone()));
        app.add_systems(Update, gui.run_if(condition.clone()));
        app.add_systems(Update, handle_inspector_events.run_if(condition.clone()));
        app.add_systems(Update, update_preview_image.run_if(condition.clone()));
        app.add_systems(Update, hovered_click_listener.run_if(condition.clone()));
    }
}

#[derive(Debug, Reflect, Clone, Event)]
enum ThreadboundUISnapshotMessage {
    UIDataUpdate {
        pos: IVec2,
    },
    GatherChildren {
        parent_drill_id: DrillId,
        parent_runtime_id: RuntimeId,
    },
    TreeClipboard {
        parent_drill_id: DrillId,
        parent_runtime_id: RuntimeId,
    },
}
impl WorkerMessage for ThreadboundUISnapshotMessage {}

#[derive(Debug, Reflect, Clone, Event)]
enum GameboundUISnapshotMessage {
    UpdateUIData {
        ui_tree: ElementInfo,
        start: ElementInfo,
    },
    GatherChildrenResponse {
        drill_id: DrillId,
        runtime_id: RuntimeId,
        children: Vec<ElementInfo>,
    },
    TreeClipboardResponse {
        ui_tree: ElementInfo,
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
            let id = parent.get_runtime_id()?;
            if id != parent_runtime_id.0 {
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
    }
    Ok(())
}

fn trigger_gather_children_request(
    mut data: ResMut<UIData>,
    mut events: EventWriter<ThreadboundUISnapshotMessage>,
) {
    for (key, state) in data.fetching.iter_mut() {
        if let FetchingState::FetchRequest = state {
            *state = FetchingState::FetchDispatched;
            events.send(ThreadboundUISnapshotMessage::GatherChildren {
                parent_drill_id: key.0.clone(),
                parent_runtime_id: key.1.clone(),
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
    egui_context_query: Query<&EguiContext, With<PrimaryWindow>>,
) {
    // Do nothing if paused
    if ui_data.paused {
        return;
    }

    // Do nothing when hovering over egui
    if egui_context_query
        .get_single()
        .map(|ctx| ctx.clone().get_mut().is_pointer_over_area())
        .unwrap_or(false)
    {
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
        .filter(|info| !info.is_stupid_size())
        .min_by_key(|info| info.bounding_rect.size().area())
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
    events.send(ThreadboundUISnapshotMessage::UIDataUpdate { pos });
    ui_data.in_flight = true;
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
                info.name.replace(' ', "_").to_lowercase(),
                info.class_name.to_lowercase()
            )
        }
        let identifier = as_rust_identifier(info);

        // get drill id
        let drill_id = match info.drill_id {
            DrillId::Child(ref inner) => inner.iter().skip(1).map(|x| x.to_string()).join(", "),
            _ => "".to_string(),
        };

        // build content
        let content = format!(
            "let {0} = root.drill(&walker, vec![{1}]).context(\"{0}\")?.try_into()?;\n",
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
    *debounce = ui_data.selected.clone();
    let image = (|| {
        // Determine what to preview
        let (info, parent_info) = match ui_data.selected.clone() {
            Some(DrillId::Child(ref inner)) => {
                // Get parent ID by dropping last element
                let mut parent_drill_id = inner.clone();
                parent_drill_id.pop_back();
                let parent_drill_id = match parent_drill_id.len() {
                    0 => DrillId::Root,
                    _ => DrillId::Child(parent_drill_id),
                };

                // Look up info for parent
                let Some(parent_info) = ui_data.ui_tree.lookup_drill_id(parent_drill_id.clone())
                else {
                    warn!("Failed to find parent info for {:?}", parent_drill_id);
                    return None;
                };

                // Look up info
                let Some(last) = inner.back() else {
                    warn!("Failed to find last element in {:?}", inner);
                    return None;
                };
                let Some(info) = parent_info.lookup_drill_id([last].into_iter().cloned().collect())
                else {
                    warn!("Failed to find info for {:?}", inner);
                    return None;
                };
                (info, parent_info)
            }
            Some(DrillId::Root) => {
                let info = &ui_data.ui_tree;
                let parent_info = &ui_data.ui_tree;
                (info, parent_info)
            }
            Some(DrillId::Unknown) => {
                warn!("Selected drill_id is unknown");
                return None;
            }
            None => return None,
        };

        // Calculate regions
        let world_capture_region = match parent_info.drill_id {
            DrillId::Root => info.children.as_ref().map_or_else(
                || info.bounding_rect,
                |children| {
                    children
                        .iter()
                        .fold(info.bounding_rect, |acc, x| acc.union(x.bounding_rect))
                },
            ),
            DrillId::Child(_) => {
                if parent_info.bounding_rect.is_empty() {
                    info.bounding_rect
                } else {
                    parent_info.bounding_rect
                }
            }
            DrillId::Unknown => {
                warn!("Parent drill_id is unknown");
                return None;
            }
        };
        let texture_highlight_region = info
            .bounding_rect
            .translate(&-world_capture_region.top_left());
        let size = world_capture_region.size().abs().as_uvec2();

        // Check assumptions about reasonable image sizes given my personal monitor setup
        // This fn is running on the main thread so big operations will lag the UI
        if size.area() > IVec2::new(2100 * 3, 1100).area() {
            warn!(
                "Image size is very large: {:?} ({} sq px), skipping",
                size,
                size.area()
            );
            return None;
        } else if size.area() == 0 {
            warn!("Image size is zero, skipping");
            return None;
        }

        // Get the texture of the element
        let Ok(image) = get_image(world_capture_region, &screen_access) else {
            warn!("Failed to get image for region {:?}", world_capture_region);
            return None;
        };
        if image.size() != size {
            warn!(
                "Image size mismatch: expected {:?}, got {:?}",
                size,
                image.size()
            );
        }

        // Convert to an image buffer for manipulation
        let Some(mut image) =
            ImageBuffer::from_raw(size.x, size.y, image.data) as Option<RgbaImage>
        else {
            warn!("Failed to convert image to buffer");
            return None;
        };

        // Apply the highlight
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            if texture_highlight_region.contains(IVec2::new(x as i32, y as i32)) {
                *pixel = Rgba([
                    pixel.0[0].saturating_add(50),
                    pixel.0[1].saturating_add(50),
                    pixel.0[2],
                    pixel.0[3],
                ]);
            }
        }

        // Convert back to Bevy image
        let image = Image::from_dynamic(DynamicImage::ImageRgba8(image), true);
        Some((image, size))
    })();
    if let Some((image, size)) = image {
        // Remove the old handle
        if let Some(ref preview) = ui_data.selected_preview {
            egui_user_textures.remove_image(&preview.handle);
        }
        // Register the handle with egui
        let handle = asset_server.add(image);
        egui_user_textures.add_image(handle.clone());
        ui_data.selected_preview = Some(PreviewImage { handle, size });
    } else {
        ui_data.selected_preview = None;
    }
}

fn gui(
    mut contexts: EguiContexts,
    mut ui_data: ResMut<UIData>,
    type_registry: Res<AppTypeRegistry>,
    mut hover_info: ResMut<HoverInfo>,
    mut threadbound_events: EventWriter<ThreadboundUISnapshotMessage>,
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

    let id = egui::Id::new("UIAutomation Inspector");
    egui::Window::new("UIAutomation Inspector")
        .id(id)
        .default_pos((5.0, 5.0))
        .default_width(1200.0)
        .default_height(1000.0)
        .default_open(false)
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
                let Some(selected_drill_id) = ui_data.selected.clone() else {
                    return;
                };
                let Some(selected_info) = ui_data
                    .ui_tree
                    .lookup_drill_id_mut(selected_drill_id.clone())
                else {
                    return;
                };
                ui.vertical_centered(|ui| {
                    ui.heading("Properties");
                    if ui.button("copy tree from here").clicked() {
                        threadbound_events.send(ThreadboundUISnapshotMessage::TreeClipboard {
                            parent_drill_id: selected_info.drill_id.clone(),
                            parent_runtime_id: selected_info.runtime_id.clone(),
                        });
                    }
                });
                inspector.ui_for_reflect_readonly(selected_info, ui);
                ui.separator();
                ui.label("drill_id");
                let drill_id = selected_info.drill_id.to_string();
                inspector.ui_for_reflect_readonly(&drill_id, ui);
                if ui.button("copy").clicked() {
                    ui.output_mut(|out| {
                        out.copied_text = drill_id.clone();
                    });
                    info!("Copied drill_id {} to clipboard", drill_id);
                }
                ui.label("runtime_id");
                let runtime_id = selected_info.runtime_id.to_string();
                inspector.ui_for_reflect_readonly(&runtime_id, ui);
                if ui.button("copy").clicked() {
                    ui.output_mut(|out| {
                        out.copied_text = runtime_id.clone();
                    });
                    info!("Copied runtime_id {} to clipboard", runtime_id);
                }

                if let Some((texture_id, size)) = preview {
                    ui.vertical_centered(|ui| {
                        ui.heading("Preview");
                    });
                    ui.image(egui::load::SizedTexture::new(texture_id, size));
                }

                ui.vertical_centered(|ui| {
                    ui.heading("Scratch Pad");
                });
                if ui.button("clear").clicked() {
                    ui_data.scratch_pad.clear();
                }
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::TextEdit::multiline(&mut ui_data.scratch_pad)
                        .desired_width(ui.available_width())
                        .show(ui);
                });
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
                if found.is_none() {
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

fn hovered_click_listener(
    mut click_events: EventReader<ClickEvent>,
    game_hover_query: Query<&GameHoverIndicator>,
    host_hover_query: Query<&HostHoverIndicator>,
    mut ui_data: ResMut<UIData>,
    mut inspector_events: EventWriter<InspectorEvent>,
) {
    for event in click_events.read() {
        let ClickEvent::Clicked {
            target_id,
            cursor_id: _,
            way,
        } = event
        else {
            continue;
        };
        if way == &Way::Left {
            if game_hover_query.get(*target_id).is_ok() || host_hover_query.get(*target_id).is_ok()
            {
                ui_data.paused ^= true;
                info!("Hover indicator clicked, paused set to {}", ui_data.paused);
            }
        } else if way == &Way::Right {
            inspector_events.send(InspectorEvent::PushScratchPad);
        }
    }
}
