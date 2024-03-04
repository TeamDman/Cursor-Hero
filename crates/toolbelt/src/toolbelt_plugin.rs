use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_2d::PhysicsSet;
use leafwing_input_manager::prelude::*;

use crate::hover_frame::insert_hover_frame;
use crate::hover_frame::remove_hover_frame;
use crate::tool_activation::tool_activation;
use crate::tool_color::tool_color;
use crate::toolbelt_circle_layout_plugin::ToolbeltCircleLayoutPlugin;
use crate::toolbelt_layout_plugin::ToolbeltLayoutPlugin;
use crate::tool_help_activation::tool_help_activation;
use crate::tool_help_cleanup::tool_help_cleanup;
use crate::tool_help_lifecycle::tool_help_lifecycle;
use crate::tool_visibility::tool_visibility;
use crate::toolbelt_opening_plugin::ToolbeltOpeningPlugin;
use cursor_hero_toolbelt_types::toolbelt_types::*;

use crate::wheel_audio::wheel_audio;
use crate::toolbelt_circle_layout_plugin::update_render_data;
pub struct ToolbeltPlugin;

impl Plugin for ToolbeltPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ToolbeltLayoutPlugin);
        app.add_plugins(ToolbeltOpeningPlugin);
        app.add_plugins(ToolbeltCircleLayoutPlugin);
        app.add_plugins(InputManagerPlugin::<ToolbeltAction>::default());
        app.add_systems(Update, insert_hover_frame);
        app.add_systems(Update, remove_hover_frame);
        app.add_systems(Update, tool_color);
        app.add_systems(Update, tool_activation);
        app.add_systems(Update, tool_help_cleanup);
        app.add_systems(
            Update,
            (
                wheel_audio,
                tool_visibility,
                tool_help_activation,
                tool_help_lifecycle,
            )
                .chain(),
        );
    }
}
