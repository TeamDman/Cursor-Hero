use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Debug, Reflect, Eq, PartialEq, Component, Clone, Copy)]
pub enum CalculatorElementKind {
    ExpressionDisplay,
    ValueDisplay,
    DigitButton(u8),
    EqualsButton,
    MultiplyButton,
    DivideButton,
    PlusButton,
    MinusButton,
    Background,
}
impl CalculatorElementKind {
    pub fn populate(&self, commands: &mut EntityCommands) {
        commands.insert(*self);
        if let CalculatorElementKind::ExpressionDisplay = self {
            commands.insert(CalculatorExpression);
        } else if let CalculatorElementKind::ValueDisplay = self {
            commands.insert(CalculatorDisplay);
        }
    }
    pub fn variants() -> Vec<CalculatorElementKind> {
        vec![
            CalculatorElementKind::ExpressionDisplay,
            CalculatorElementKind::ValueDisplay,
            CalculatorElementKind::DigitButton(0),
            CalculatorElementKind::DigitButton(1),
            CalculatorElementKind::DigitButton(2),
            CalculatorElementKind::DigitButton(3),
            CalculatorElementKind::DigitButton(4),
            CalculatorElementKind::DigitButton(5),
            CalculatorElementKind::DigitButton(6),
            CalculatorElementKind::DigitButton(7),
            CalculatorElementKind::DigitButton(8),
            CalculatorElementKind::DigitButton(9),
            CalculatorElementKind::EqualsButton,
            CalculatorElementKind::MultiplyButton,
            CalculatorElementKind::DivideButton,
            CalculatorElementKind::PlusButton,
            CalculatorElementKind::MinusButton,
            CalculatorElementKind::Background,
        ]
    }
    pub fn get_default_text(&self) -> Option<String> {
        match self {
            CalculatorElementKind::ExpressionDisplay => Some("".to_string()),
            CalculatorElementKind::ValueDisplay => Some("0".to_string()),
            CalculatorElementKind::DigitButton(digit) => Some(digit.to_string()),
            CalculatorElementKind::EqualsButton => Some("=".to_string()),
            CalculatorElementKind::MultiplyButton => Some("*".to_string()),
            CalculatorElementKind::DivideButton => Some("/".to_string()),
            CalculatorElementKind::PlusButton => Some("+".to_string()),
            CalculatorElementKind::MinusButton => Some("-".to_string()),
            CalculatorElementKind::Background => None,
        }
    }
    pub fn get_text_from_state(&self, state: &CalculatorState) -> Option<String> {
        match self {
            CalculatorElementKind::ExpressionDisplay => Some(state.expression.clone()),
            CalculatorElementKind::ValueDisplay => Some(state.value.clone()),
            _ => None,
        }
    }
    pub fn get_full_name(&self) -> String {
        format!("CalculatorElementKind::{}", self.get_name())
    }
    pub fn get_name(&self) -> String {
        match self {
            CalculatorElementKind::ExpressionDisplay => "ExpressionDisplay".to_string(),
            CalculatorElementKind::ValueDisplay => "ValueDisplay".to_string(),
            CalculatorElementKind::DigitButton(digit) => format!("DigitButton({})", digit),
            CalculatorElementKind::EqualsButton => "EqualsButton".to_string(),
            CalculatorElementKind::MultiplyButton => "MultiplyButton".to_string(),
            CalculatorElementKind::DivideButton => "DivideButton".to_string(),
            CalculatorElementKind::PlusButton => "PlusButton".to_string(),
            CalculatorElementKind::MinusButton => "MinusButton".to_string(),
            CalculatorElementKind::Background => "Background".to_string(),
        }
    }
    pub fn from_identifier(name: &str) -> Option<CalculatorElementKind> {
        match name {
            "one_button" => Some(CalculatorElementKind::DigitButton(1)),
            "two_button" => Some(CalculatorElementKind::DigitButton(2)),
            "three_button" => Some(CalculatorElementKind::DigitButton(3)),
            "four_button" => Some(CalculatorElementKind::DigitButton(4)),
            "five_button" => Some(CalculatorElementKind::DigitButton(5)),
            "six_button" => Some(CalculatorElementKind::DigitButton(6)),
            "seven_button" => Some(CalculatorElementKind::DigitButton(7)),
            "eight_button" => Some(CalculatorElementKind::DigitButton(8)),
            "nine_button" => Some(CalculatorElementKind::DigitButton(9)),
            "zero_button" => Some(CalculatorElementKind::DigitButton(0)),
            "equals_button" => Some(CalculatorElementKind::EqualsButton),
            "plus_button" => Some(CalculatorElementKind::PlusButton),
            "minus_button" => Some(CalculatorElementKind::MinusButton),
            "multiply_by_button" => Some(CalculatorElementKind::MultiplyButton),
            "divide_by_button" => Some(CalculatorElementKind::DivideButton),
            "_landmarktarget" => Some(CalculatorElementKind::Background),
            x if x.starts_with("display_is_") => Some(CalculatorElementKind::ValueDisplay),
            x if x.starts_with("expression_is_") => Some(CalculatorElementKind::ExpressionDisplay),
            _ => None,
        }
    }
}

pub trait CalculatorTheme {
    fn get_bounds(&self, element_kind: &CalculatorElementKind) -> Rect;
    fn get_background_color(&self, element_kind: &CalculatorElementKind) -> Color;
    fn get_text_style(&self, element_kind: &CalculatorElementKind) -> TextStyle;
}

#[derive(Debug, Reflect)]
pub enum CalculatorThemeKind {
    WindowsDark,
}
impl CalculatorTheme for CalculatorThemeKind {
    fn get_bounds(&self, element_kind: &CalculatorElementKind) -> Rect {
        let CalculatorThemeKind::WindowsDark = self;
        match element_kind {
            CalculatorElementKind::Background => Rect::new(0.0, 0.0, 320.0, -456.0),
            CalculatorElementKind::ExpressionDisplay => Rect::new(18.0, -44.0, 302.0, -63.0),
            CalculatorElementKind::ValueDisplay => Rect::new(0.0, -63.0, 320.0, -107.0),
            CalculatorElementKind::DivideButton => Rect::new(256.0, -286.0, 316.0, -318.0),
            CalculatorElementKind::MultiplyButton => Rect::new(256.0, -320.0, 316.0, -353.0),
            CalculatorElementKind::MinusButton => Rect::new(256.0, -355.0, 316.0, -387.0),
            CalculatorElementKind::PlusButton => Rect::new(256.0, -389.0, 316.0, -422.0),
            CalculatorElementKind::EqualsButton => Rect::new(256.0, -424.0, 316.0, -456.0),
            CalculatorElementKind::DigitButton(0) => Rect::new(130.0, -424.0, 191.0, -456.0),
            CalculatorElementKind::DigitButton(1) => Rect::new(67.0, -389.0, 128.0, -422.0),
            CalculatorElementKind::DigitButton(2) => Rect::new(130.0, -389.0, 191.0, -422.0),
            CalculatorElementKind::DigitButton(3) => Rect::new(193.0, -389.0, 254.0, -422.0),
            CalculatorElementKind::DigitButton(4) => Rect::new(67.0, -355.0, 128.0, -387.0),
            CalculatorElementKind::DigitButton(5) => Rect::new(130.0, -355.0, 191.0, -387.0),
            CalculatorElementKind::DigitButton(6) => Rect::new(193.0, -355.0, 254.0, -387.0),
            CalculatorElementKind::DigitButton(7) => Rect::new(67.0, -320.0, 128.0, -353.0),
            CalculatorElementKind::DigitButton(8) => Rect::new(130.0, -320.0, 191.0, -353.0),
            CalculatorElementKind::DigitButton(9) => Rect::new(193.0, -320.0, 254.0, -353.0),

            CalculatorElementKind::DigitButton(_) => Rect::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    fn get_background_color(&self, element_kind: &CalculatorElementKind) -> Color {
        let CalculatorThemeKind::WindowsDark = self;
        match element_kind {
            CalculatorElementKind::DigitButton(_) => Color::rgb(0.231, 0.231, 0.231),
            CalculatorElementKind::EqualsButton => Color::rgb(0.89, 0.412, 0.71),
            _ => Color::rgb(0.196, 0.196, 0.196),
        }
    }

    fn get_text_style(&self, element_kind: &CalculatorElementKind) -> TextStyle {
        let CalculatorThemeKind::WindowsDark = self;
        TextStyle {
            font_size: 32.0,
            color: match element_kind {
                CalculatorElementKind::EqualsButton => Color::rgb(0.227, 0.106, 0.18),
                _ => Color::WHITE,
            },
            ..default()
        }
    }
}

#[derive(Debug, Reflect)]
pub struct CalculatorState {
    pub expression: String,
    pub value: String,
}
impl Default for CalculatorState {
    fn default() -> Self {
        Self {
            expression: "".to_string(),
            value: "0".to_string(),
        }
    }
}

#[derive(Event, Debug, Reflect)]
pub struct SpawnCalculatorRequestEvent {
    pub environment_id: Entity,
    pub theme: CalculatorThemeKind,
    pub state: CalculatorState,
    pub position: Vec2,
}

#[derive(Component, Debug, Reflect, Default, Clone, PartialEq)]
pub struct Calculator;
#[derive(Component, Debug, Reflect, Default, Clone, PartialEq)]
pub struct CalculatorStartMenuPanelButton;

#[derive(Component, Debug, Reflect, Default)]
pub struct CalculatorDisplay;
#[derive(Component, Debug, Reflect, Default)]
pub struct CalculatorExpression;
#[derive(Component, Debug, Reflect, Default)]
pub struct CalculatorButton;
