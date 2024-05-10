use crate::ui_inspector_children_fetcher_plugin::UiInspectorChildrenFetcherPlugin;
use crate::ui_inspector_egui_plugin::UiInspectorEguiPlugin;
use crate::ui_inspector_events_plugin::UiInspectorEventsPlugin;
use crate::ui_inspector_hover_indicator_click_plugin::UiInspectorHoverIndicatorClickPlugin;
use crate::ui_inspector_preview_image_plugin::UiInspectorPreviewImagePlugin;
use crate::ui_inspector_toggle_plugin::UiInspectorTogglePlugin;
use crate::ui_inspector_tree_update_plugin::UiInspectorTreeUpdatePlugin;
use crate::ui_inspector_worker_plugin::UiInspectorWorkerPlugin;
use bevy::prelude::*;

pub struct UiInspectorPlugin;

impl Plugin for UiInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiInspectorPreviewImagePlugin);
        app.add_plugins(UiInspectorWorkerPlugin);
        app.add_plugins(UiInspectorChildrenFetcherPlugin);
        app.add_plugins(UiInspectorTreeUpdatePlugin);
        app.add_plugins(UiInspectorEventsPlugin);
        app.add_plugins(UiInspectorHoverIndicatorClickPlugin);
        app.add_plugins(UiInspectorEguiPlugin);
        app.add_plugins(UiInspectorTogglePlugin);
    }
}
