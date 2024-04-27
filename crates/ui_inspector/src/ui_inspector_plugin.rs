use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui;
use bevy_egui::egui::collapsing_header::CollapsingState;
use bevy_egui::egui::Color32;
use bevy_egui::EguiContext;
use bevy_egui::EguiContexts;
use bevy_egui::EguiUserTextures;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use cursor_hero_bevy::prelude::Area;
use cursor_hero_bevy::prelude::BottomRightI;
use cursor_hero_bevy::prelude::TopLeftI;
use cursor_hero_bevy::prelude::TranslateIVec2;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorElementKind;
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
use cursor_hero_ui_inspector_types::prelude::ScratchPadMode;
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

        let visible_condition = |ui_data: Res<UIData>| ui_data.visible;
        app.add_systems(
            Update,
            (|mut ui_data: ResMut<UIData>| {
                ui_data.visible ^= true;
            })
            .run_if(input_just_pressed(KeyCode::Grave)),
        );
        app.add_systems(
            Update,
            trigger_tree_update_for_hovered.run_if(visible_condition),
        );
        app.add_systems(
            Update,
            trigger_gather_children_request.run_if(visible_condition),
        );
        app.add_systems(Update, handle_gamebound_messages.run_if(visible_condition));
        app.add_systems(Update, gui.run_if(visible_condition));
        app.add_systems(Update, handle_inspector_events.run_if(visible_condition));
        app.add_systems(Update, update_preview_image.run_if(visible_condition));
        app.add_systems(Update, hovered_click_listener.run_if(visible_condition));
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
        // get selected info
        let Some(selected_drill_id) = ui_data.selected.clone() else {
            return;
        };

        // get window
        let window = selected_drill_id
            .as_child()
            .map(|inner| inner.iter().take(1).cloned().collect())
            .and_then(|window_drill_id| ui_data.ui_tree.lookup_drill_id(window_drill_id))
            .unwrap_or(&ui_data.ui_tree);

        let push_infos = match event {
            InspectorEvent::PushSelectedToScratchPad => {
                let Some(selected_info) =
                    ui_data.ui_tree.lookup_drill_id(selected_drill_id.clone())
                else {
                    return;
                };

                vec![selected_info]
            }
            InspectorEvent::PushKnownToScratchPad => {
                if window.name == "Calculator" {
                    window
                        .get_descendents()
                        .into_iter()
                        .filter(|info| {
                            CalculatorElementKind::from_identifier(info.as_identifier().as_str())
                                .is_some()
                        })
                        .collect()
                } else {
                    // Unknown window, just do selected
                    let Some(selected_info) =
                        ui_data.ui_tree.lookup_drill_id(selected_drill_id.clone())
                    else {
                        return;
                    };

                    vec![selected_info]
                }
            }
        };
        let mut content = String::new();
        for push_info in push_infos {
            // get identifier
            let mut identifier = push_info.as_identifier();

            content.push_str(match ui_data.scratch_pad_mode {
                ScratchPadMode::Drill => {
                    // get drill id
                    let drill_id = push_info
                        .drill_id
                        .as_child()
                        .map(|d| d.iter().skip(1).map(|x| x.to_string()).join(", "))
                        .unwrap_or_default();

                    // build content
                    let content = format!(
                        "let {0} = root.drill(&walker, vec![{1}]).context(\"{0}\")?.try_into()?;\n",
                        identifier, drill_id
                    );

                    content
                }
                ScratchPadMode::Bounds => {
                    // Use mark if present, window otherwise
                    let compare_drill_id = ui_data
                        .mark
                        .clone()
                        .or_else(|| {
                            selected_drill_id
                                .as_child()
                                .map(|inner| inner.iter().take(1).cloned().collect())
                        })
                        .unwrap_or(DrillId::Root);

                    // Look up the comparison element
                    let compare = ui_data
                        .ui_tree
                        .lookup_drill_id(compare_drill_id)
                        .unwrap_or(&ui_data.ui_tree);

                    // Get the bounds of the selected element relative to the comparison element
                    let bounds_relative = push_info
                        .bounding_rect
                        .translated(&-compare.bounding_rect.top_left());

                    // Use known identifiers based on window

                    if window.name == "Calculator"
                        && let Some(calc_elem) = CalculatorElementKind::from_identifier(&identifier)
                    {
                        identifier = format!("CalculatorElementKind::{}", calc_elem.get_name());
                    }

                    // Format as string
                    format!(
                        "{} => Rect::new({:.1},{:.1},{:.1},{:.1}),\n",
                        identifier,
                        bounds_relative.top_left().x as f32,
                        -bounds_relative.top_left().y as f32,
                        bounds_relative.bottom_right().x as f32,
                        -bounds_relative.bottom_right().y as f32
                    )
                }
            }.as_str());
        }
        
        // append to scratch pad
        // make new rows show at the top by adding to the front
        ui_data.scratch_pad.insert_str(0, content.as_str());
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
    debounce.clone_from(&ui_data.selected);
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
            .translated(&-world_capture_region.top_left());
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
    mut inspector_events: EventWriter<InspectorEvent>,
) {
    if !ui_data.visible {
        return;
    }

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
        .default_open(ui_data.open)
        .show(ctx, |ui| {
            egui::SidePanel::left(id.with("tree"))
                .resizable(true)
                .width_range(100.0..=4000.0)
                .default_width(600.0)
                .show_inside(ui, |ui| {
                    // LEFT PANEL

                    // Header
                    ui.vertical_centered(|ui| {
                        ui.heading("UI Tree");
                    });

                    // Tree
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
                // RIGHT PANEL
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Ensure something is selected
                    let Some(selected_drill_id) = ui_data.selected.clone() else {
                        return;
                    };

                    // Ensure the thing selected is in the tree
                    let Some(selected_info) =
                        ui_data.ui_tree.lookup_drill_id(selected_drill_id.clone())
                    else {
                        return;
                    };

                    // Properties header
                    let mut mark_clicked = false;
                    ui.vertical_centered(|ui| {
                        ui.heading("Properties");
                        if ui.button("copy tree from here").clicked() {
                            threadbound_events.send(ThreadboundUISnapshotMessage::TreeClipboard {
                                parent_drill_id: selected_info.drill_id.clone(),
                                parent_runtime_id: selected_info.runtime_id.clone(),
                            });
                        }
                        let change_mark_button_text = match (&ui_data.mark, &selected_info.drill_id)
                        {
                            (Some(ref mark), drill_id) if mark == drill_id => "clear mark",
                            _ => "set mark",
                        };
                        if ui.button(change_mark_button_text).clicked() {
                            mark_clicked = true;
                        }
                    });

                    // Properties
                    inspector.ui_for_reflect_readonly(selected_info, ui);

                    // Derived properties
                    ui.separator();

                    // Drill ID
                    ui.label("drill_id");
                    let drill_id = selected_info.drill_id.to_string();
                    inspector.ui_for_reflect_readonly(&drill_id, ui);
                    if ui.button("copy").clicked() {
                        ui.output_mut(|out| {
                            out.copied_text.clone_from(&drill_id);
                        });
                        info!("Copied drill_id {} to clipboard", drill_id);
                    }

                    // Runtime ID
                    ui.label("runtime_id");
                    let runtime_id = selected_info.runtime_id.to_string();
                    inspector.ui_for_reflect_readonly(&runtime_id, ui);
                    if ui.button("copy").clicked() {
                        ui.output_mut(|out| {
                            out.copied_text.clone_from(&runtime_id);
                        });
                        info!("Copied runtime_id {} to clipboard", runtime_id);
                    }

                    // Bounds size
                    ui.label("bounds size");
                    let bounds_size = selected_info.bounding_rect.size().to_string();
                    inspector.ui_for_reflect_readonly(&bounds_size, ui);
                    if ui.button("copy").clicked() {
                        ui.output_mut(|out| {
                            out.copied_text.clone_from(&bounds_size);
                        });
                        info!("Copied bounds size {} to clipboard", bounds_size);
                    }

                    ui.label("bounds relative to mark");

                    // Use mark if present, window otherwise
                    let compare_drill_id = ui_data
                        .mark
                        .clone()
                        .or_else(|| {
                            selected_drill_id
                                .as_child()
                                .map(|inner| inner.iter().take(1).cloned().collect())
                        })
                        .unwrap_or(DrillId::Root);

                    // Look up the comparison element
                    let compare = ui_data
                        .ui_tree
                        .lookup_drill_id(compare_drill_id)
                        .unwrap_or(&ui_data.ui_tree);

                    // Get the bounds of the selected element relative to the comparison element
                    let bounds_relative = selected_info
                        .bounding_rect
                        .translated(&-compare.bounding_rect.top_left());

                    // Format as string
                    let bounds_relative_str = format!(
                        "({:.1},{:.1},{:.1},{:.1})",
                        bounds_relative.top_left().x as f32,
                        -bounds_relative.top_left().y as f32,
                        bounds_relative.bottom_right().x as f32,
                        -bounds_relative.bottom_right().y as f32
                    );

                    // Render the string
                    inspector.ui_for_reflect_readonly(&bounds_relative_str, ui);

                    // Render copy button
                    if ui.button("copy").clicked() {
                        ui.output_mut(|out| {
                            out.copied_text.clone_from(&bounds_relative_str);
                        });
                        info!(
                            "Copied bounds relative {} to clipboard",
                            bounds_relative_str
                        );
                    }

                    // We can borrow ui_data as mut now since this is
                    // the last time selected_info is borrowed
                    if mark_clicked {
                        ui_data.mark = if ui_data.mark.is_none() {
                            Some(selected_info.drill_id.clone())
                        } else {
                            None
                        };
                    }

                    // Preview image if possible
                    if let Some((texture_id, size)) = preview {
                        ui.vertical_centered(|ui| {
                            ui.heading("Preview");
                        });
                        ui.image(egui::load::SizedTexture::new(texture_id, size));
                    }

                    // Scratch Pad
                    ui.vertical_centered(|ui| {
                        ui.heading("Scratch Pad");
                    });

                    // Scratch pad - mode switch
                    ui.horizontal(|ui| {
                        ui.radio_value(
                            &mut ui_data.scratch_pad_mode,
                            ScratchPadMode::Drill,
                            "Drill",
                        );
                        ui.radio_value(
                            &mut ui_data.scratch_pad_mode,
                            ScratchPadMode::Bounds,
                            "Bounds",
                        );
                    });

                    ui.horizontal(|ui| {
                        // Scratch pad - clear button
                        if ui.button("clear").clicked() {
                            ui_data.scratch_pad.clear();
                        }

                        // space between buttons
                        ui.add_space(10.0);

                        // Scratch pad - copy button
                        if ui.button("copy").clicked() {
                            ui.output_mut(|out| {
                                out.copied_text.clone_from(&ui_data.scratch_pad);
                            });
                            info!("Copied scratch pad to clipboard");
                        }

                        // space between buttons
                        ui.add_space(10.0);

                        // Scratch pad - push button
                        if ui.button("push").clicked() {
                            inspector_events.send(InspectorEvent::PushSelectedToScratchPad);
                            info!("Sent push event");
                        }

                        // Scratch pad - push button
                        if ui.button("push all").clicked() {
                            inspector_events.send(InspectorEvent::PushKnownToScratchPad);
                            info!("Sent push all event");
                        }
                    });

                    // Scratch pad - text area
                    egui::TextEdit::multiline(&mut ui_data.scratch_pad)
                        .desired_width(ui.available_width())
                        .show(ui);
                });
            });
        });

    if CollapsingState::load(ctx, id.with("collapsing"))
        .map(|x| x.is_open())
        .unwrap_or(ui_data.open)
        != ui_data.open
    {
        ui_data.open = !ui_data.open;
    }

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

            // Update highlight colours if marked
            let mut previous = None;
            if data.mark == Some(element_info.drill_id.clone()) {
                previous = Some((
                    ui.style().visuals.selection.bg_fill,
                    ui.style().visuals.widgets.hovered.weak_bg_fill,
                ));

                let visuals = &mut ui.style_mut().visuals;
                visuals.selection.bg_fill = Color32::from_rgb(61, 42, 102);
                visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(41, 22, 82);
            }

            // Draw the toggle
            let mut toggle = ui.toggle_value(&mut selected, label);

            // Restore previous colours
            if let Some(previous) = previous {
                let visuals = &mut ui.style_mut().visuals;
                visuals.selection.bg_fill = previous.0;
                visuals.widgets.hovered.weak_bg_fill = previous.1;
            }

            // Always apply highlight
            if data.mark == Some(element_info.drill_id.clone()) {
                toggle = toggle.highlight();
            }

            // Update selected
            if toggle.changed() {
                data.selected = if selected {
                    Some(element_info.drill_id.clone())
                } else {
                    None
                };
            };

            // Update hovered
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
            // If the click event targets a hover indicator
            if game_hover_query.get(*target_id).is_ok() || host_hover_query.get(*target_id).is_ok()
            {
                // Toggle the paused state
                ui_data.paused ^= true;
                info!("Hover indicator clicked, paused set to {}", ui_data.paused);
            }
        } else if way == &Way::Right {
            inspector_events.send(InspectorEvent::PushSelectedToScratchPad);
        }
    }
}
