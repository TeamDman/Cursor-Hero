#![feature(let_chains)]

pub mod cube_tool;
pub mod default_wheel_tool;
pub mod focus_tool;
pub mod keyboard_tool;
pub mod keyboard_wheel_tool;
pub mod level_bounds_visibility_tool;
pub mod placeholder_tool;
pub mod restart_tool;
pub mod scroll_tool;
pub mod talk_tool;
pub mod tool_plugin;
pub mod tool_spawning;
pub mod window_drag_tool;

pub use crate::tool_plugin::ToolPlugin;

pub mod prelude {
    pub use crate::tool_spawning::NoInputs;
    pub use crate::tool_spawning::StartingState;
    pub use crate::tool_spawning::ToolSpawnConfig;
}
