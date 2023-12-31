pub mod gamepad_connected_event_preference_update_system;
pub mod tool_activated_tag_update_system;
pub mod tool_hovered_tag_update_system;
pub mod tool_visual_update_system;
pub mod toolbelt_circle_radius_update_system;
pub mod toolbelt_spawn_setup_system;
pub mod toolbelt_visibility_update_system;
pub mod types;

use bevy::prelude::*;
use cursor_hero_character::character_plugin::CharacterSystemSet;
use leafwing_input_manager::prelude::*;

use crate::gamepad_connected_event_preference_update_system::update_gamepad_settings;
use crate::tool_activated_tag_update_system::tool_activation_update_system;
use crate::tool_hovered_tag_update_system::tool_hovered_tag_update_system;
use crate::tool_visual_update_system::tool_visual_update_system;
use crate::toolbelt_circle_radius_update_system::toolbelt_circle_radius_update_system;
use crate::toolbelt_spawn_setup_system::toolbelt_spawn_setup_system;
use crate::types::*;
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
            .configure_sets(Startup, ToolbeltSystemSet::Spawn)
            .add_systems(
                Startup,
                (apply_deferred, toolbelt_spawn_setup_system)
                    .chain()
                    .in_set(ToolbeltSystemSet::Spawn)
                    .after(CharacterSystemSet::Spawn),
            )
            .add_systems(
                Update,
                (
                    update_gamepad_settings,
                    (
                        toolbelt_visibility_update_system::update_toolbelt_visibility,
                        tool_visual_update_system,
                        tool_hovered_tag_update_system,
                        tool_activation_update_system,
                    )
                        .chain(),
                    toolbelt_circle_radius_update_system,
                ),
            );
    }
}
