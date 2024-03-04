use bevy::prelude::*;

#[derive(Debug, Reflect)]
pub enum HostWindowPosition {
    Fullscreen {
        monitor: u32,
    },
    Corner {
        corner: cursor_hero_math::prelude::Corner,
        monitor: u32
    }
}

#[derive(Component, Reflect, Default)]
pub struct WindowPositionLoadoutSwitcherTool;

#[derive(Component, Reflect)]
pub struct WindowPositionTool {
    pub window_position: HostWindowPosition,
}
