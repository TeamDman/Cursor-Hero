use crate::chat_input_buffer_plugin::ChatInputBufferPlugin;
use crate::chat_input_visuals_plugin::ChatInputVisualsPlugin;
use crate::chat_tool_plugin::ChatToolPlugin;
use crate::chat_wheel_tool_plugin::ChatWheelToolPlugin;
use bevy::prelude::*;
pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ChatToolPlugin);
        app.add_plugins(ChatWheelToolPlugin);
        app.add_plugins(ChatInputVisualsPlugin);
        app.add_plugins(ChatInputBufferPlugin);
    }
}
