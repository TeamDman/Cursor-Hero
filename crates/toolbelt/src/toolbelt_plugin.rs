use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::hover_tag::hover_tag;
use crate::insert_toolbelt::insert_toolbelt;
use crate::tool_color::tool_color;
use crate::tool_frame::insert_hover_frame;
use crate::tool_frame::remove_hover_frame;
use crate::tool_toggle::tool_toggle;
use crate::types::*;
use crate::wheel_radius::wheel_radius;
use crate::wheel_visibility::wheel_visibility;
pub struct ToolbeltPlugin;

impl Plugin for ToolbeltPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Toolbelt>()
            .register_type::<Wheel>()
            .register_type::<Tool>()
            .register_type::<ToolActiveTag>()
            .register_type::<ToolHoveredTag>()
            .add_event::<ToolbeltEvent>()
            .add_event::<ToolHoveredEvent>()
            .add_event::<ToolActivationEvent>()
            .add_plugins(InputManagerPlugin::<ToolbeltAction>::default())
            .add_systems(
                Update,
                (
                    wheel_visibility,
                    tool_color,
                    hover_tag,
                    tool_toggle,
                    wheel_radius,
                    insert_toolbelt,
                    insert_hover_frame,
                    remove_hover_frame,
                ),
            );
    }
}
