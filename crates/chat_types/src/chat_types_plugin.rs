use bevy::prelude::*;
use crate::prelude::*;
pub struct ChatTypesPlugin;

impl Plugin for ChatTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChatWheelTool>();       
        app.register_type::<ChatTool>();       
        app.register_type::<ChatBubble>();       
        app.register_type::<ChatInput>();       
        app.add_event::<ChatEvent>();
        app.add_event::<ChatInputEvent>();
    }
}