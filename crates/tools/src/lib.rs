pub mod click_tool;
pub mod cube_tool;
pub mod cursor_monitor_position_tool;
pub mod cursor_window_position_tool;
pub mod focus_tool;
pub mod inspect_tool;
pub mod placeholder_tool;
pub mod restart_tool;
pub mod sprint_tool;
pub mod talk_tool;
pub mod tool_naming;
pub mod tool_plugin;
pub mod tool_spawning;
pub mod zoom_tool;

pub use crate::tool_plugin::ToolPlugin;

pub mod prelude {
    pub use crate::tool_spawning::spawn_action_tool;
    pub use crate::tool_spawning::spawn_tool;
}
