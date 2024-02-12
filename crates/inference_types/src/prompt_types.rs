use bevy::prelude::*;
use cursor_hero_text_asset_types::text_asset_loader_types::TextAsset;

use crate::prelude::TextInferenceOptions;

#[derive(Reflect, Debug, PartialEq, Eq, Clone)]
pub enum TextPrompt {
    Raw {
        content: String,
        options: Option<TextInferenceOptions>,
    },
    Chat {
        chat_history: String,
        options: Option<TextInferenceOptions>,
    },
}

impl TextPrompt {
    pub fn options(&self) -> Option<TextInferenceOptions> {
        match self {
            TextPrompt::Raw { options, .. } => options.clone(),
            TextPrompt::Chat { options, .. } => options.clone(),
        }
    }
}

#[derive(Reflect, Debug, PartialEq, Eq, Clone)]
pub struct MaterializedTextPrompt {
    pub prompt: TextPrompt,
    pub materialized: String,
}

#[derive(Resource, Debug, Default, Reflect, PartialEq, Eq, Clone)]
pub struct TextPromptHandles {
    pub raw: Handle<TextAsset>,
    pub chat: Handle<TextAsset>,
}

#[derive(Reflect, Debug, PartialEq, Eq, Clone)]
pub enum SpeechPrompt {
    Raw { content: String },
}

#[derive(Reflect, Debug, PartialEq, Eq, Clone)]
pub enum TranscriptionPrompt {
    Raw { content: Vec<u8> },
}
