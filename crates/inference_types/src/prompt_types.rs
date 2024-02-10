use bevy::prelude::*;

#[derive(Reflect, Debug, PartialEq, Eq, Clone)]
pub enum Prompt {
    Raw { content: String },
    Chat { chat_history: String },
}
impl Prompt {
    pub fn path(&self) -> &'static str {
        match self {
            Prompt::Raw { .. } => "prompt_templates/raw.txt",
            Prompt::Chat { .. } => "prompt_templates/chat.txt",
        }
    }
}
