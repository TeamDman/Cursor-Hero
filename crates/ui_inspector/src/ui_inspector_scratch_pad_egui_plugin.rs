use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::collapsing_header::CollapsingState;
use bevy_egui::EguiContexts;
use cursor_hero_ui_automation::prelude::DrillId;
use cursor_hero_ui_inspector_types::prelude::InspectorScratchPadEvent;
use cursor_hero_ui_inspector_types::prelude::ScratchPadMode;
use cursor_hero_ui_inspector_types::prelude::UIData;
pub struct UiInspectorScratchPadEguiPlugin;

impl Plugin for UiInspectorScratchPadEguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            gui.run_if(|ui_data: Res<UIData>| {
                ui_data.windows.global_toggle && ui_data.windows.scratch_pad
            }),
        );
    }
}

fn gui(
    mut contexts: EguiContexts,
    mut ui_data: ResMut<UIData>,
    mut inspector_events: EventWriter<InspectorScratchPadEvent>,
) {
    // Get context
    let ctx = contexts.ctx_mut();

    // Do window
    let window_id = egui::Id::new("UIAutomation Inspector Scratch Pad");
    egui::Window::new("Scratch Pad")
        .id(window_id)
        // .open(open)
        // .default_pos((5.0, 5.0))
        // .default_width(1200.0)
        // .default_height(1000.0)
        .default_open(ui_data.windows.scratch_pad_header_open)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let window_drill_id = ui_data
                    .selected
                    .as_ref()
                    .and_then(|x| {
                        x.as_child()
                            .map(|inner| inner.iter().take(1).cloned().collect())
                    })
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
                    if ui.button("push").clicked()
                        && let Some(ref selected_drill_id) = ui_data.selected
                        && let Some(selected_info) =
                            ui_data.tree.lookup_drill_id(selected_drill_id.clone())
                    {
                        inspector_events.send(InspectorScratchPadEvent::ScratchPadAppendInfo {
                            info: selected_info.clone(),
                        });
                        info!("Sent push event");
                    }

                    // Scratch pad - push button
                    if ui.button("push known").clicked() {
                        inspector_events.send(InspectorScratchPadEvent::ScratchPadAppendAllKnown);
                        info!("Sent push known event");
                    }
                    // Scratch pad - push button
                    if ui.button("push unknown").clicked() {
                        inspector_events.send(InspectorScratchPadEvent::ScratchPadAppendAllUnknown);
                        info!("Sent push unknown event");
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
            });
        });

    // Track window collapsed state
    ui_data.windows.scratch_pad_header_open =
        CollapsingState::load(ctx, window_id.with("collapsing"))
            .map(|x| x.is_open())
            .unwrap_or(ui_data.windows.scratch_pad_header_open);
}
