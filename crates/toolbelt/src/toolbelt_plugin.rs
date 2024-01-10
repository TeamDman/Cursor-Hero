use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::hover_tag::hover_tag;
use crate::insert_toolbelt::insert_toolbelt;
use crate::tool_toggle::tool_toggle;
use crate::tool_visuals::tool_visuals;
use crate::types::*;
use crate::update_gamepad_settings::update_gamepad_settings;
use crate::wheel_radius::wheel_radius;
use crate::wheel_visibility::wheel_visibility;
pub struct ToolbeltPlugin;

impl Plugin for ToolbeltPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Toolbelt>()
            .register_type::<CirclularDistributionProperties>()
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
                    update_gamepad_settings,
                    wheel_visibility,
                    tool_visuals,
                    hover_tag,
                    tool_toggle,
                    wheel_radius,
                    insert_toolbelt,
                ),
            );
    }
}
