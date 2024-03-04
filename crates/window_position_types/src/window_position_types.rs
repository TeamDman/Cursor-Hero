use bevy::prelude::*;


#[derive(Debug, Reflect)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Reflect)]
pub enum HostWindowPosition {
    Fullscreen {
        monitor: u8,
    },
    Corner {
        corner: Corner,
        monitor: u8
    }
}

#[derive(Component, Reflect, Default)]
pub struct WindowPositionLoadoutSwitcherTool;

#[derive(Component, Reflect)]
pub struct WindowPositionTool {
    pub window_position: HostWindowPosition,
}
