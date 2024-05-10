use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::collapsing_header::CollapsingState;
use bevy_egui::egui::Color32;
use bevy_egui::EguiContexts;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use cursor_hero_bevy::prelude::BottomRightI;
use cursor_hero_bevy::prelude::TopLeftI;
use cursor_hero_bevy::prelude::TranslateIVec2;
use cursor_hero_ui_automation::prelude::*;
use cursor_hero_ui_hover_types::prelude::HoverInfo;
use cursor_hero_ui_hover_types::prelude::InspectorHoverIndicator;
use cursor_hero_ui_inspector_types::prelude::FetchingState;
use cursor_hero_ui_inspector_types::prelude::InspectorEvent;
use cursor_hero_ui_inspector_types::prelude::ScratchPadMode;
use cursor_hero_ui_inspector_types::prelude::ThreadboundUISnapshotMessage;
use cursor_hero_ui_inspector_types::prelude::UIData;

pub struct UiInspectorEguiPlugin;

impl Plugin for UiInspectorEguiPlugin {
    fn build(&self, app: &mut App) {
        let visible_condition = |ui_data: Res<UIData>| ui_data.visible;
        app.add_systems(Update, gui.run_if(visible_condition));
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

                    if let Some(mark) = &ui_data.mark {
                        ui.label("drill_id relative to mark");
                        let relative_drill_id =
                            selected_info.drill_id.relative_to(&mark).to_string();
                        inspector.ui_for_reflect_readonly(&relative_drill_id, ui);
                        if ui.button("copy").clicked() {
                            ui.output_mut(|out| {
                                out.copied_text.clone_from(&relative_drill_id);
                            });
                            info!(
                                "Copied drill_id relative to mark {} to clipboard",
                                relative_drill_id
                            );
                        }
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
                        "Rect::new({:.1},{:.1},{:.1},{:.1})",
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

                    let window_drill_id = selected_drill_id
                        .as_child()
                        .map(|inner| inner.iter().take(1).cloned().collect())
                        .unwrap_or(DrillId::Root);

                    // Scratch pad - mode switch
                    ui.label("changing mode clears scratch pad");
                    ui.horizontal(|ui| {
                        let changed = ui
                            .radio_value(
                                &mut ui_data.scratch_pad_mode,
                                ScratchPadMode::Drill,
                                "Drill",
                            )
                            .changed()
                            || ui
                                .radio_value(
                                    &mut ui_data.scratch_pad_mode,
                                    ScratchPadMode::Bounds,
                                    "Bounds",
                                )
                                .changed()
                            || ui
                                .radio_value(
                                    &mut ui_data.scratch_pad_mode,
                                    ScratchPadMode::Color,
                                    "Color",
                                )
                                .changed();
                        if changed {
                            ui_data.scratch_pad.clear();
                        }
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

                        // Mark - set to window
                        if ui.button("mark window").clicked() {
                            ui_data.mark = Some(window_drill_id);
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
                        id.with(child.drill_id.clone()),
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
                    data.fetching.insert(key, FetchingState::RequestingFetch);
                } else if let Some(FetchingState::Fetched(ref mut children)) = found {
                    element_info.children = Some(std::mem::take(children));
                    data.fetching.remove(&key);
                } else {
                    ui.label("fetching...");
                }
            }
        });
}


