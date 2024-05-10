use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::collapsing_header::CollapsingState;
use bevy_egui::EguiContexts;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use cursor_hero_ui_hover_types::prelude::HoverInfo;
use cursor_hero_ui_hover_types::prelude::InspectorHoverIndicator;
use cursor_hero_ui_inspector_types::prelude::InspectorEvent;
use cursor_hero_ui_inspector_types::prelude::ThreadboundUISnapshotMessage;
use cursor_hero_ui_inspector_types::prelude::UIData;

use crate::ui_inspector_egui_properties_panel::do_properties_panel;
use crate::ui_inspector_egui_tree_panel::do_tree_panel;

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
    let window_id = egui::Id::new("UIAutomation Inspector");
    egui::Window::new("UIAutomation Inspector")
        .id(window_id)
        .default_pos((5.0, 5.0))
        .default_width(1200.0)
        .default_height(1000.0)
        .default_open(ui_data.open)
        .show(ctx, |ui| {
            egui::SidePanel::left(window_id.with("tree"))
                .resizable(true)
                .width_range(100.0..=4000.0)
                .default_width(600.0)
                .show_inside(ui, |ui| {
                    // LEFT PANEL
                    do_tree_panel(ui, &window_id, &mut inspector, &mut ui_data);
                });

            egui::TopBottomPanel::bottom(window_id.with("invisible panel to make things work"))
                .show_separator_line(false)
                .show_inside(ui, |_ui| {});

            egui::CentralPanel::default().show_inside(ui, |ui| {
                // RIGHT PANEL
                egui::ScrollArea::vertical().show(ui, |ui| {
                    do_properties_panel(
                        ui,
                        &mut inspector,
                        &mut ui_data,
                        preview,
                        &mut threadbound_events,
                        &mut inspector_events,
                    );
                });
            });
        });

    // Track window collapsed state
    if CollapsingState::load(ctx, window_id.with("collapsing"))
        .map(|x| x.is_open())
        .unwrap_or(ui_data.open)
        != ui_data.open
    {
        ui_data.open = !ui_data.open;
    }

    // Display paused status
    let id = egui::Id::new("Paused");
    egui::Window::new("Paused")
        .id(id)
        .title_bar(false)
        .default_pos((ctx.screen_rect().max.x - 200.0, 5.0))
        .show(ctx, |ui| {
            ui.checkbox(&mut ui_data.paused, "Paused");
        });
    ui_data.fresh = false;

    // Update info for inspector hover indicator
    hover_info.inspector_hover_indicator = ui_data
        .hovered
        .as_ref()
        .map(|elem| InspectorHoverIndicator { info: elem.clone() });
}
