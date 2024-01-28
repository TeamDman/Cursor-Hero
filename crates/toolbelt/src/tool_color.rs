use super::types::*;
use bevy::prelude::*;
use cursor_hero_pointer::pointer_hover_plugin::Hovered;

enum ToolColor {
    Active,
    Hovered,
    HoveredActive,
    Disabled,
}

impl ToolColor {
    fn color(&self) -> Color {
        match self {
            ToolColor::Active => Color::WHITE,
            ToolColor::Hovered => Color::YELLOW,
            ToolColor::HoveredActive => Color::ORANGE,
            ToolColor::Disabled => Color::GRAY,
        }
    }

    fn from(hovered: bool, active: bool) -> Self {
        match (hovered, active) {
            (true, true) => ToolColor::HoveredActive,
            (true, false) => ToolColor::Hovered,
            (false, true) => ToolColor::Active,
            (false, false) => ToolColor::Disabled,
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn tool_color(
    mut query: Query<(&mut Sprite, Option<&Hovered>, Option<&ActiveTool>), With<Tool>>,
) {
    for (mut sprite, hovered, active) in query.iter_mut() {
        let color = ToolColor::from(hovered.is_some(), active.is_some()).color();
        sprite.color = color;
    }
}
