use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_2d::PhysicsSet;
use leafwing_input_manager::prelude::*;

use crate::hover_tag::hover_tag;
use crate::insert_toolbelt::insert_toolbelt;
use crate::pointer_reach::pointer_reach;
use crate::tool_color::tool_color;
use crate::tool_distribution::tool_distribution;
use crate::tool_frame::insert_hover_frame;
use crate::tool_frame::remove_hover_frame;
use crate::tool_toggle::tool_toggle;
use crate::tool_visibility;
use crate::tool_visibility::tool_visibility;
use crate::types::*;
use crate::wheel_properties::wheel_properties;
pub struct ToolbeltPlugin;

impl Plugin for ToolbeltPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Toolbelt>()
            .register_type::<Wheel>()
            .register_type::<Tool>()
            .register_type::<ActiveTool>()
            .register_type::<HoveredTool>()
            .add_event::<ToolbeltEvent>()
            .add_event::<ToolHoveredEvent>()
            .add_event::<ToolActivationEvent>()
            .add_plugins(InputManagerPlugin::<ToolbeltAction>::default())
            .add_systems(
                Update,
                (
                    hover_tag,
                    insert_hover_frame,
                    insert_toolbelt,
                    remove_hover_frame,
                    tool_color,
                    tool_toggle,
                    tool_visibility,
                    wheel_properties,
                    pointer_reach,
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
