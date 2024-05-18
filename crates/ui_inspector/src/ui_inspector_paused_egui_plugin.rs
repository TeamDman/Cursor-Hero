use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::collapsing_header::CollapsingState;
use bevy_egui::EguiContexts;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use cursor_hero_ui_hover_types::prelude::HoverInfo;
use cursor_hero_ui_hover_types::prelude::InspectorHoverIndicator;
use cursor_hero_ui_inspector_types::prelude::ThreadboundUISnapshotMessage;
use cursor_hero_ui_inspector_types::prelude::UIData;

use crate::ui_inspector_egui_properties_panel::do_properties_panel;
use crate::ui_inspector_egui_tree_panel::do_tree_panel;

pub struct UiInspectorPausedEguiPlugin;

impl Plugin for UiInspectorPausedEguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            gui.run_if(|ui_data: Res<UIData>| {
                ui_data.windows.global_toggle && ui_data.windows.tree
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
    let mut cx = bevy_inspector_egui::reflect_inspector::Context {
        world: None,
        queue: None,
    };

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
