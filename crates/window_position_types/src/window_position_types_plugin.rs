use crate::prelude::*;
use bevy::prelude::*;

pub struct WindowPositionTypesPlugin;

impl Plugin for WindowPositionTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<crate::window_position_types::HostWindowPosition>();
        app.register_type::<WindowPositionLoadoutSwitcherTool>();
        app.register_type::<WindowPositionTool>();
    }
}
