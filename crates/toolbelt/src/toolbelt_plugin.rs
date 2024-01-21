use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_2d::PhysicsSet;
use leafwing_input_manager::prelude::*;

use crate::hover_detection::hover_detection;
use crate::hover_frame::insert_hover_frame;
use crate::hover_frame::remove_hover_frame;
use crate::insert_toolbelt::insert_toolbelt;
use crate::pointer_reach::pointer_reach;
use crate::tool_activation::tool_activation;
use crate::tool_color::tool_color;
use crate::tool_distribution::tool_distribution;
use crate::tool_help_activation::tool_help_activation;
use crate::tool_help_cleanup::tool_help_cleanup;
use crate::tool_help_lifecycle::tool_help_lifecycle;
use crate::tool_visibility::tool_visibility;
use crate::types::*;
use crate::wheel_audio::wheel_audio;
use crate::wheel_opening::wheel_opening;
pub struct ToolbeltPlugin;

impl Plugin for ToolbeltPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Toolbelt>()
            .register_type::<Wheel>()
            .register_type::<Tool>()
            .register_type::<ActiveTool>()
            .register_type::<Hovered>()
            .add_event::<ToolbeltEvent>()
            .add_event::<ToolHoveredEvent>()
            .add_event::<ToolActivationEvent>()
            .add_plugins(InputManagerPlugin::<ToolbeltAction>::default())
            .add_systems(
                Update,
                (
                    hover_detection,
                    insert_hover_frame,
                    insert_toolbelt,
                    remove_hover_frame,
                    tool_color,
                    tool_activation,
                    tool_help_cleanup,
                    (
                        wheel_opening,
                        wheel_audio,
                        pointer_reach, // prevent sprint plugin from clobbering wheel pointer reach update
                        tool_visibility,
                        tool_help_activation,
                        tool_help_lifecycle,
                    )
                        .chain(),
                ),
            )
            .add_systems(
                PostUpdate,
                tool_distribution
                    .after(PhysicsSet::Sync)
                    .after(TransformSystem::TransformPropagate),
            );
    }
}
