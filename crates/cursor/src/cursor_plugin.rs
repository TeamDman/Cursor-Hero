use bevy::prelude::*;
use cursor_hero_cursor_types::cursor_action_types::CursorActionPlugin;

use crate::cursor_click_plugin::CursorClickPlugin;
use crate::cursor_hover_plugin::CursorHoverPlugin;
use crate::cursor_mirroring_plugin::CursorMirroringPlugin;
use crate::cursor_positioning_plugin::CursorPositioningPlugin;
use crate::cursor_reach_plugin::CursorReachPlugin;
use crate::cursor_spawning_plugin::CursorSpawningPlugin;

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CursorHoverPlugin,
            CursorReachPlugin,
            CursorClickPlugin,
            CursorPositioningPlugin,
            CursorSpawningPlugin,
            CursorActionPlugin,
            CursorMirroringPlugin,
        ));
    }
}
