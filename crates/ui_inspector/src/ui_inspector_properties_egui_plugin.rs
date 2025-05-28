use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::collapsing_header::CollapsingState;
use bevy_egui::EguiContexts;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use cursor_hero_ui_inspector_types::prelude::ThreadboundUISnapshotMessage;
use cursor_hero_ui_inspector_types::prelude::UIData;

use crate::ui_inspector_egui_properties_panel::do_properties_panel;

pub struct UiInspectorPropertiesEguiPlugin;

impl Plugin for UiInspectorPropertiesEguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            gui.run_if(|ui_data: Res<UIData>| {
                ui_data.windows.global_toggle && ui_data.windows.properties.open
            }),
        );
    }
}

fn gui(
    mut contexts: EguiContexts,
    mut ui_data: ResMut<UIData>,
    type_registry: Res<AppTypeRegistry>,
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
    let window_id = egui::Id::new("UI Properties");
    egui::Window::new("UI Properties")
        .id(window_id)
        .default_open(ui_data.windows.properties.header_open)
        .show(ctx, |ui| {
            do_properties_panel(
                ui,
                &mut inspector,
                &mut ui_data,
                preview,
                &mut threadbound_events,
            );
        });

    // Track window collapsed state
    ui_data.windows.properties.header_open = CollapsingState::load(ctx, window_id.with("collapsing"))
        .map(|x| x.is_open())
        .unwrap_or(ui_data.windows.properties.header_open);
}
