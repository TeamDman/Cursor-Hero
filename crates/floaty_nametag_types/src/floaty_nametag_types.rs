use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct FloatyName {
    pub text: String,
    pub vertical_offset: f32,
    pub appearance: NametagAppearance,
}

#[derive(Component, Debug, Reflect)]
pub struct FloatyNametag {
    pub owner: Entity,
}

#[derive(Debug, Reflect)]
pub enum NametagAppearance {
    Character,
    Databrick,
}
impl NametagAppearance {
    pub fn get_font_path(&self) -> String {
        match self {
            NametagAppearance::Character => "fonts/kenney_kenney-fonts/Fonts/Kenney Rocket.ttf",
            NametagAppearance::Databrick => "fonts/kenney_kenney-fonts/Fonts/Kenney Blocks.ttf",
        }.to_string()
    }
    pub fn get_text_color(&self) -> Color {
        match self {
            NametagAppearance::Character => Color::GREEN,
            NametagAppearance::Databrick => Color::BLUE,
        }
    }
}