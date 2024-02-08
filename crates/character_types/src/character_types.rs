use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, InspectorOptions, Reflect, Default)]
#[reflect(Component, InspectorOptions)]
pub struct Character;

#[derive(Component, Reflect, Default)]
pub struct MainCharacter;
#[derive(Component, Reflect, Default)]
pub struct AgentCharacter;

#[derive(Component, Reflect, Eq, PartialEq, Debug)]
pub enum CharacterAppearance {
    Focused,
    Unfocused,
}
impl CharacterAppearance {
    pub fn get_texture_path(&self) -> &'static str {
        match self {
            Self::Focused => "textures/character/default_character_focused.png",
            Self::Unfocused => "textures/character/default_character_unfocused.png",
        }
    }
}
