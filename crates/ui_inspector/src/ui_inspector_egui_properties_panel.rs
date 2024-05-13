use bevy::ecs::event::EventWriter;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::Ui;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use cursor_hero_bevy::prelude::BottomRightI;
use cursor_hero_bevy::prelude::TopLeftI;
use cursor_hero_bevy::prelude::TranslateIVec2;
use cursor_hero_ui_automation::prelude::DrillId;
use cursor_hero_ui_inspector_types::prelude::InspectorScratchPadEvent;
use cursor_hero_ui_inspector_types::prelude::ScratchPadMode;
use cursor_hero_ui_inspector_types::prelude::ThreadboundUISnapshotMessage;
use cursor_hero_ui_inspector_types::prelude::UIData;

pub fn do_properties_panel(
    ui: &mut Ui,
    inspector: &mut InspectorUi,
    ui_data: &mut UIData,
    preview: Option<(egui::TextureId, (f32, f32))>,
    threadbound_events: &mut EventWriter<ThreadboundUISnapshotMessage>,
    inspector_events: &mut EventWriter<InspectorScratchPadEvent>,
) {
    // Ensure something is selected
    let Some(selected_drill_id) = ui_data.selected.clone() else {
        return;
    };

    // Ensure the thing selected is in the tree
    let Some(selected_info) = ui_data.ui_tree.lookup_drill_id(selected_drill_id.clone()) else {
        return;
    };

    // Properties header
    let mut mark_clicked = false;
    ui.vertical_centered(|ui| {
        ui.heading("Properties");
    });
    ui.horizontal(|ui| {
        if ui.button("copy tree from here").clicked() {
            let msg = ThreadboundUISnapshotMessage::TreeClipboard {
                parent_drill_id: selected_info.drill_id.clone(),
                parent_runtime_id: selected_info.runtime_id.clone(),
            };
            debug!("Sending {msg:?}");
            threadbound_events.send(msg);
        }

        if ui.button("populate tree from here").clicked() {
            let msg = ThreadboundUISnapshotMessage::TreePatch {
                parent_drill_id: selected_info.drill_id.clone(),
                parent_runtime_id: selected_info.runtime_id.clone(),
            };
            debug!("Sending {msg:?}");
            threadbound_events.send(msg);
            // ui_data.in_flight = true;
        }

        let change_mark_button_text = match (&ui_data.mark, &selected_info.drill_id) {
            (Some(ref mark), drill_id) if mark == drill_id => "clear mark",
            _ => "set mark",
        };
        if ui.button(change_mark_button_text).clicked() {
            mark_clicked = true;
        }
    });

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
        let relative_drill_id = selected_info.drill_id.relative_to(mark).to_string();
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
        let old = ui_data.mark.take();
        ui_data.mark = if ui_data.mark.is_none() {
            Some(selected_info.drill_id.clone())
        } else {
            None
        };
        debug!("Updated mark from {:?} to {:?}", old, ui_data.mark);
    }

    // Preview image if possible
    if let Some((texture_id, size)) = preview {
        ui.vertical_centered(|ui| {
            ui.heading("Preview");
        });
        ui.image(SizedTexture::new(texture_id, size));
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
    ui.label("(changing mode will clear scratch pad)");
    ui.horizontal(|ui| {
        let mut changed = false;
        for mode in ScratchPadMode::variants() {
            let text = mode.to_string();
            changed |= ui
                .radio_value(&mut ui_data.scratch_pad_mode, mode, text)
                .changed();
        }
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
            inspector_events.send(InspectorScratchPadEvent::ScratchPadAppendInfo {
                info: selected_info.clone(),
            });
            info!("Sent push event");
        }

        // Scratch pad - push button
        if ui.button("push known").clicked() {
            inspector_events.send(InspectorScratchPadEvent::ScratchPadAppendAllKnown);
            info!("Sent push all event");
        }
        // Scratch pad - push button
        if ui.button("push all").clicked() {
            inspector_events.send(InspectorScratchPadEvent::ScratchPadAppendAll);
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

    // Properties
    ui.separator();
    inspector.ui_for_reflect_readonly(selected_info, ui);
}
