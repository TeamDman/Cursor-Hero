use bevy::{prelude::*, window::{WindowMode, WindowResolution}};

#[derive(Debug, Reflect)]
pub enum HostWindowPosition {
    Fullscreen {
        monitor: u32,
    },
    Corner {
        corner: cursor_hero_math::prelude::Corner,
        monitor: u32,
    },
}

#[derive(Component, Reflect, Default)]
pub struct WindowPositionLoadoutSwitcherTool;

#[derive(Component, Reflect)]
pub struct WindowPositionTool {
    pub window_position: HostWindowPosition,
}


#[derive(Reflect, Debug, Default)]
pub struct WindowPositionCommand {
    pub mode: Option<WindowMode>,
    pub resolution: Option<WindowResolution>,
    pub position: Option<WindowPosition>,
}

#[derive(Resource, Reflect, Debug, Default)]
pub struct WindowPositionCommandQueue {
    pub commands: Vec<WindowPositionCommand>,
}