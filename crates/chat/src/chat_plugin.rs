use crate::chat_bubble_plugin::ChatBubblePlugin;
use crate::chat_focus_exclusivity_plugin::ChatFocusExclusivityPlugin;
use crate::chat_input_buffer_plugin::ChatInputBufferPlugin;
use crate::chat_sfx_plugin::ChatSfxPlugin;
use crate::chat_tool_plugin::ChatToolPlugin;
use bevy::prelude::*;
pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ChatToolPlugin);
        app.add_plugins(ChatFocusExclusivityPlugin);
        app.add_plugins(ChatBubblePlugin);
        app.add_plugins(ChatInputBufferPlugin);
        app.add_plugins(ChatSfxPlugin);
    }
}
