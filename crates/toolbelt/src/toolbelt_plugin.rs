use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_2d::PhysicsSet;
use leafwing_input_manager::prelude::*;

use crate::hover_frame::insert_hover_frame;
use crate::hover_frame::remove_hover_frame;
use crate::tool_activation::tool_activation;
use crate::tool_color::tool_color;
use crate::tool_distribution::tool_distribution;
use crate::tool_help_activation::tool_help_activation;
use crate::tool_help_cleanup::tool_help_cleanup;
use crate::tool_help_lifecycle::tool_help_lifecycle;
use crate::tool_visibility::tool_visibility;
use cursor_hero_toolbelt_types::types::*;

use crate::wheel_audio::wheel_audio;
use crate::wheel_opening::wheel_opening;
pub struct ToolbeltPlugin;

impl Plugin for ToolbeltPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ToolbeltAction>::default());
        app.add_systems(Update, insert_hover_frame);
        app.add_systems(Update, remove_hover_frame);
        app.add_systems(Update, tool_color);
        app.add_systems(Update, tool_activation);
        app.add_systems(Update, tool_help_cleanup);
        app.add_systems(
            Update,
            (
                wheel_opening,
                wheel_audio,
                tool_visibility,
                tool_help_activation,
                tool_help_lifecycle,
            )
                .chain(),
        );
        app.add_systems(
            PostUpdate,
            tool_distribution
                .after(PhysicsSet::Sync)
                .after(TransformSystem::TransformPropagate),
        );
    }
}
