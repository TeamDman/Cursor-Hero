use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_2d::PhysicsSet;
use leafwing_input_manager::prelude::*;

use crate::hover_tag::hover_tag;
use crate::insert_toolbelt::insert_toolbelt;
use crate::tool_color::tool_color;
use crate::tool_frame::insert_hover_frame;
use crate::tool_frame::remove_hover_frame;
use crate::tool_toggle::tool_toggle;
use crate::types::*;
use crate::wheel_distribution::wheel_distribution;
use crate::wheel_opening::wheel_opening;
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
                    tool_color,
                    hover_tag,
                    tool_toggle,
                    wheel_opening,
                    insert_toolbelt,
                    insert_hover_frame,
                    remove_hover_frame,
                    wheel_distribution

                ),
            )
            // .add_systems(
            //     PostUpdate,
            //     wheel_distribution
            //         .after(PhysicsSet::Sync)
            //         .after(TransformSystem::TransformPropagate),
            // );
            ;
    }
}
