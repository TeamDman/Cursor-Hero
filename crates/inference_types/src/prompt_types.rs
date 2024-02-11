use bevy::prelude::*;
use cursor_hero_text_asset_types::text_asset_loader_types::TextAsset;

#[derive(Reflect, Debug, PartialEq, Eq, Clone)]
pub enum Prompt {
    Raw { content: String },
    Chat { chat_history: String },
}

#[derive(Reflect, Debug, PartialEq, Eq, Clone)]
pub struct MaterializedPrompt {
    pub prompt: Prompt,
    pub materialized: String,
}

#[derive(Resource, Debug, Default, Reflect, PartialEq, Eq, Clone)]
pub struct PromptHandles {
    pub raw: Handle<TextAsset>,
    pub chat: Handle<TextAsset>,
}