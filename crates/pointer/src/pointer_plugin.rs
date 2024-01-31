use bevy::prelude::*;
use cursor_hero_pointer_types::pointer_action_types::PointerActionPlugin;

use crate::pointer_click_plugin::PointerClickPlugin;
use crate::pointer_environment_plugin::PointerEnvironmentPlugin;
use crate::pointer_hover_plugin::PointerHoverPlugin;
use crate::pointer_positioning_plugin::PointerPositioningPlugin;
use crate::pointer_reach_plugin::PointerReachPlugin;
use crate::pointer_spawning_plugin::PointerSpawningPlugin;

pub struct PointerPlugin;
impl Plugin for PointerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PointerHoverPlugin,
            PointerReachPlugin,
            PointerClickPlugin,
            PointerEnvironmentPlugin,
            PointerPositioningPlugin,
            PointerSpawningPlugin,
            PointerActionPlugin,
        ));
    }
}
