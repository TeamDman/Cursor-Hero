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
use cursor_hero_ui_inspector_types::prelude::ThreadboundUISnapshotMessage;
use cursor_hero_ui_inspector_types::prelude::UIData;

pub fn do_properties_panel(
    ui: &mut Ui,
    inspector: &mut InspectorUi,
    ui_data: &mut UIData,
    preview: Option<(egui::TextureId, (f32, f32))>,
    threadbound_events: &mut EventWriter<ThreadboundUISnapshotMessage>,
) {
    // Ensure something is selected
    let Some(selected_drill_id) = ui_data.selected.clone() else {
        return;
    };

    // Ensure the thing selected is in the tree
    let Some(selected_info) = ui_data.tree.lookup_drill_id(selected_drill_id.clone()) else {
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
    ui.horizontal(|ui| {
        let drill_id = selected_info.drill_id.to_string();
        if ui.button("copy").clicked() {
            ui.output_mut(|out| {
                out.copied_text.clone_from(&drill_id);
            });
            info!("Copied drill_id {} to clipboard", drill_id);
        }
        ui.label("drill_id");
        inspector.ui_for_reflect_readonly(&drill_id, ui);
    });

    if let Some(mark) = &ui_data.mark {
        ui.horizontal(|ui| {
            let relative_drill_id = selected_info.drill_id.relative_to(mark).to_string();
            if ui.button("copy").clicked() {
                ui.output_mut(|out| {
                    out.copied_text.clone_from(&relative_drill_id);
                });
                info!(
                    "Copied drill_id relative to mark {} to clipboard",
                    relative_drill_id
                );
            }
            ui.label("drill_id relative to mark");
            inspector.ui_for_reflect_readonly(&relative_drill_id, ui);
        });
    }

    // Name
    ui.horizontal(|ui| {
        let name = selected_info.name.to_string();
        if ui.button("copy").clicked() {
            ui.output_mut(|out| {
                out.copied_text.clone_from(&name);
            });
            info!("Copied name {} to clipboard", name);
        }
        ui.label("name");
        inspector.ui_for_reflect_readonly(&name, ui);
    });

    // Runtime ID
    ui.horizontal(|ui| {
        let runtime_id = selected_info.runtime_id.to_string();
        if ui.button("copy").clicked() {
            ui.output_mut(|out| {
                out.copied_text.clone_from(&runtime_id);
            });
            info!("Copied runtime_id {} to clipboard", runtime_id);
        }
        ui.label("runtime_id");
        inspector.ui_for_reflect_readonly(&runtime_id, ui);
    });

    // Automation ID
    ui.horizontal(|ui| {
        let automation_id = selected_info.automation_id.to_string();
        if ui.button("copy").clicked() {
            ui.output_mut(|out| {
                out.copied_text.clone_from(&automation_id);
            });
            info!("Copied automation_id {} to clipboard", automation_id);
        }
        ui.label("automation_id");
        inspector.ui_for_reflect_readonly(&automation_id, ui);
    });

    // Control Type
    ui.horizontal(|ui| {
        let control_type = selected_info.control_type.clone();
        if ui.button("copy").clicked() {
            ui.output_mut(|out| {
                out.copied_text.clone_from(&format!("{control_type:?}"));
            });
            info!("Copied control_type {:?} to clipboard", control_type);
        }
        ui.label("control_type");
        inspector.ui_for_reflect_readonly(&control_type, ui);
    });

    // Localized control Type
    ui.horizontal(|ui| {
        let localized_control_type = selected_info.localized_control_type.clone();
        if ui.button("copy").clicked() {
            ui.output_mut(|out| {
                out.copied_text.clone_from(&localized_control_type);
            });
            info!(
                "Copied localized_control_type {:?} to clipboard",
                localized_control_type
            );
        }
        ui.label("localized_control_type");
        inspector.ui_for_reflect_readonly(&localized_control_type, ui);
    });

    // Class name
    ui.horizontal(|ui| {
        let class_name = selected_info.class_name.clone();
        if ui.button("copy").clicked() {
            ui.output_mut(|out| {
                out.copied_text.clone_from(&class_name);
            });
            info!("Copied class_name {:?} to clipboard", class_name);
        }
        ui.label("class_name");
        inspector.ui_for_reflect_readonly(&class_name, ui);
    });

    // Bounds size
    ui.horizontal(|ui| {
        let bounds_size = selected_info.bounding_rect.size().to_string();
        if ui.button("copy").clicked() {
            ui.output_mut(|out| {
                out.copied_text.clone_from(&bounds_size);
            });
            info!("Copied bounds size {} to clipboard", bounds_size);
        }
        ui.label("bounds size");
        inspector.ui_for_reflect_readonly(&bounds_size, ui);
    });

    ui.horizontal(|ui| {

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
            .tree
            .lookup_drill_id(compare_drill_id)
            .unwrap_or(&ui_data.tree);

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

        ui.label("bounds relative to mark");
        inspector.ui_for_reflect_readonly(&bounds_relative_str, ui);
    });

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

    // Properties
    // ui.separator();
    // inspector.ui_for_reflect_readonly(selected_info, ui);
}
