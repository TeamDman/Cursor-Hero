use crate::ui_inspector_children_fetcher_plugin::UiInspectorChildrenFetcherPlugin;
use crate::ui_inspector_egui_plugin::UiInspectorEguiPlugin;
use crate::ui_inspector_hover_indicator_click_plugin::UiInspectorHoverIndicatorClickPlugin;
use crate::ui_inspector_preview_image_plugin::UiInspectorPreviewImagePlugin;
use crate::ui_inspector_scratch_pad_egui_plugin::UiInspectorScratchPadEguiPlugin;
use crate::ui_inspector_scratch_pad_events_plugin::UiInspectorScratchPadEventsPlugin;
use crate::ui_inspector_toggle_plugin::UiInspectorTogglePlugin;
use crate::ui_inspector_tree_update_plugin::UiInspectorTreeUpdatePlugin;
use crate::ui_inspector_worker_plugin::UiInspectorWorkerPlugin;
use bevy::input::common_conditions::input_toggle_active;
use cursor_hero_input::active_input_state_plugin::InputMethod;

use bevy::prelude::*;
use bevy_inspector_egui::quick::StateInspectorPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use cursor_hero_ui_inspector_types::prelude::UIData;

pub struct UiInspectorPlugin;

impl Plugin for UiInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiInspectorPreviewImagePlugin);
        app.add_plugins(UiInspectorWorkerPlugin);
        app.add_plugins(UiInspectorChildrenFetcherPlugin);
        app.add_plugins(UiInspectorTreeUpdatePlugin);
        app.add_plugins(UiInspectorScratchPadEventsPlugin);
        app.add_plugins(UiInspectorHoverIndicatorClickPlugin);
        app.add_plugins(UiInspectorEguiPlugin);
        app.add_plugins(UiInspectorTogglePlugin);
        app.add_plugins(UiInspectorScratchPadEguiPlugin);

        // must be after the default plugins
        app.add_plugins(
            WorldInspectorPlugin::default().run_if(|ui_data: Res<UIData>| {
                ui_data.opened.global_toggle && ui_data.opened.world
            }),
        );
        app.add_plugins(
            StateInspectorPlugin::<InputMethod>::default().run_if(|ui_data: Res<UIData>| {
                ui_data.opened.global_toggle && ui_data.opened.state
            }),
        );
    }
}
