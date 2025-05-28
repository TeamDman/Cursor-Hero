use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::collapsing_header::CollapsingState;
use bevy_egui::EguiContexts;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use cursor_hero_ui_hover_types::prelude::HoverInfo;
use cursor_hero_ui_hover_types::prelude::InspectorHoverIndicator;
use cursor_hero_ui_inspector_types::prelude::UIData;
use crate::ui_inspector_egui_tree_panel::do_tree_panel;

pub struct UiInspectorTreeEguiPlugin;

impl Plugin for UiInspectorTreeEguiPlugin {
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
    type_registry: Res<AppTypeRegistry>,
    mut hover_info: ResMut<HoverInfo>,
) {
    // Get context
    let ctx = contexts.ctx_mut();
    let mut cx = bevy_inspector_egui::reflect_inspector::Context {
        world: None,
        queue: None,
    };
    let type_registry = type_registry.0.clone();
    let type_registry = type_registry.read();
    let mut inspector = InspectorUi::for_bevy(&type_registry, &mut cx);

    // Do window
    let window_id = egui::Id::new("UI Tree");
    egui::Window::new("UI Tree")
        .id(window_id)
        .default_open(ui_data.windows.tree.header_open)
        .show(ctx, |ui| {
            do_tree_panel(ui, &window_id, &mut inspector, &mut ui_data);
        });

    // Track window collapsed state
    ui_data.windows.tree.header_open = CollapsingState::load(ctx, window_id.with("collapsing"))
        .map(|x| x.is_open())
        .unwrap_or(ui_data.windows.tree.header_open);

    // Reset fresh state after drawing tree
    ui_data.tree_is_fresh = false;

    // Update info for inspector hover indicator
    hover_info.inspector_hover_indicator = ui_data
        .hovered
        .as_ref()
        .map(|elem| InspectorHoverIndicator { info: elem.clone() });
}
