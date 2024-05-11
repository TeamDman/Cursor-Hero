use bevy_egui::egui;
use bevy_egui::egui::Color32;
use bevy_egui::egui::Id;
use bevy_egui::egui::ScrollArea;
use bevy_egui::egui::Ui;
use bevy_inspector_egui::reflect_inspector::InspectorUi;
use cursor_hero_ui_automation::prelude::*;
use cursor_hero_ui_inspector_types::prelude::FetchingState;
use cursor_hero_ui_inspector_types::prelude::UIData;

pub fn do_tree_panel(
    ui: &mut Ui,
    window_id: &Id,
    inspector: &mut InspectorUi,
    ui_data: &mut UIData,
) {
    // Header
    ui.vertical_centered(|ui| {
        ui.heading("UI Tree");
    });

    // Tree
    ScrollArea::both().show(ui, |ui| {
        let id = window_id.with(ui_data.ui_tree.runtime_id.clone());
        let mut elem = ui_data.ui_tree.clone();

        // resets each frame before being set when drawing expandos
        ui_data.hovered = None;

        ui_for_element_info(id, ui, ui_data, &mut elem, inspector);
        ui_data.ui_tree = elem;
        ui.allocate_space(ui.available_size());
    });
}

#[allow(clippy::too_many_arguments)]
fn ui_for_element_info(
    id: egui::Id,
    ui: &mut egui::Ui,
    data: &mut UIData,
    element_info: &mut ElementInfo,
    inspector: &mut InspectorUi,
) {
    // Create expando using default from data
    let default_open = data.default_expanded.contains(&element_info.drill_id);
    let mut expando = egui::collapsing_header::CollapsingState::load_with_default_open(
        ui.ctx(),
        id,
        default_open,
    );
    let expando_is_open = expando.is_open();

    // Logic for when new data has arrived
    if data.fresh {
        // Force expanded
        expando.set_open(default_open);

        // Reset pending requests
        data.fetching.clear();
    }

    // Show
    expando
        .show_header(ui, |ui| {
            do_header(ui, data, element_info);
        })
        .body(|ui| {
            do_body(ui, id, expando_is_open, data, element_info, inspector);
        });
}

fn do_header(ui: &mut Ui, data: &mut UIData, element_info: &mut ElementInfo) {
    // Get selected state
    let mut selected = data.selected == Some(element_info.drill_id.clone());

    // Scroll to selected when fresh
    if selected && data.fresh {
        ui.scroll_to_cursor(Some(egui::Align::Center));
    }

    // Get label
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

    // Update highlight colour if known
    if previous.is_none() && false {
        todo!();
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
}

fn do_body(
    ui: &mut Ui,
    id: Id,
    expando_is_open: bool,
    data: &mut UIData,
    element_info: &mut ElementInfo,
    inspector: &mut InspectorUi,
) {
    if let Some(ref mut children) = element_info.children {
        for child in children.iter_mut() {
            ui_for_element_info(id.with(child.drill_id.clone()), ui, data, child, inspector);
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
}
