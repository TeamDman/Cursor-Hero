use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::EguiContexts;
use cursor_hero_ui_inspector_types::prelude::UIData;

pub struct UiInspectorPausedEguiPlugin;

impl Plugin for UiInspectorPausedEguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            gui.run_if(|ui_data: Res<UIData>| {
                ui_data.windows.global_toggle && ui_data.windows.tree.open
            }),
        );
    }
}

fn gui(
    mut contexts: EguiContexts,
    mut ui_data: ResMut<UIData>,
) {
    // Get context
    let ctx = contexts.ctx_mut();
    
    // Display paused status
    let id = egui::Id::new("Paused");
    egui::Window::new("Paused")
        .id(id)
        .title_bar(false)
        .default_pos((ctx.screen_rect().max.x - 200.0, 5.0))
        .show(ctx, |ui| {
            ui.checkbox(&mut ui_data.paused, "Paused");
        });
    ui_data.tree_is_fresh = false;
}
