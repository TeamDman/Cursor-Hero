use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use cursor_hero_ui_automation_types::prelude::ElementInfo;

#[derive(Debug, Reflect, Eq, PartialEq, Component, Clone, Copy)]
pub enum CalculatorElementKind {
    ExpressionDisplay,
    ValueDisplay,
    ClearButton,
    ClearEntryButton,
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
        } else if let CalculatorElementKind::ClearButton = self {
            commands.insert(CalculatorClearButton);
        } else if let CalculatorElementKind::ClearEntryButton = self {
            commands.insert(CalculatorClearEntryButton);
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
            CalculatorElementKind::ClearButton,
            CalculatorElementKind::ClearEntryButton,
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
            CalculatorElementKind::ClearButton => Some("C".to_string()),
            CalculatorElementKind::ClearEntryButton => Some("CE".to_string()),
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
    pub fn get_enum_variant_instance(&self) -> String {
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
            CalculatorElementKind::ClearButton => "ClearButton".to_string(),
            CalculatorElementKind::ClearEntryButton => "ClearExpressionButton".to_string(),
        }
    }
    pub fn get_enum_variant_declaration(&self) -> String {
        match self {
            CalculatorElementKind::ExpressionDisplay => "ExpressionDisplay".to_string(),
            CalculatorElementKind::ValueDisplay => "ValueDisplay".to_string(),
            CalculatorElementKind::DigitButton(_) => "DigitButton(u8)".to_string(),
            CalculatorElementKind::EqualsButton => "EqualsButton".to_string(),
            CalculatorElementKind::MultiplyButton => "MultiplyButton".to_string(),
            CalculatorElementKind::DivideButton => "DivideButton".to_string(),
            CalculatorElementKind::PlusButton => "PlusButton".to_string(),
            CalculatorElementKind::MinusButton => "MinusButton".to_string(),
            CalculatorElementKind::Background => "Background".to_string(),
            CalculatorElementKind::ClearButton => "ClearButton".to_string(),
            CalculatorElementKind::ClearEntryButton => "ClearExpressionButton".to_string(),
        }
    }
    #[rustfmt::skip]
    pub fn from_info(info: &ElementInfo) -> Option<CalculatorElementKind> {
        match info {
            info if info.name == "One" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(1)),
            info if info.name == "Two" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(2)),
            info if info.name == "Three" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(3)),
            info if info.name == "Four" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(4)),
            info if info.name == "Five" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(5)),
            info if info.name == "Six" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(6)),
            info if info.name == "Seven" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(7)),
            info if info.name == "Eight" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(8)),
            info if info.name == "Nine" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(9)),
            info if info.name == "Zero" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(0)),
            info if info.name == "Equals" && info.class_name == "Button" => Some(CalculatorElementKind::EqualsButton),
            info if info.name == "Plus" && info.class_name == "Button" => Some(CalculatorElementKind::PlusButton),
            info if info.name == "Minus" && info.class_name == "Button" => Some(CalculatorElementKind::MinusButton),
            info if info.name == "Multiply" && info.class_name == "Button" => Some(CalculatorElementKind::MultiplyButton),
            info if info.name == "Divide" && info.class_name == "Button" => Some(CalculatorElementKind::DivideButton),
            info if info.name == "Clear" && info.class_name == "Button" => Some(CalculatorElementKind::ClearButton),
            info if info.name == "Clear entry" && info.class_name == "Button" => Some(CalculatorElementKind::ClearEntryButton),
            info if info.name.is_empty() && info.class_name == "LandmarkTarget" => Some(CalculatorElementKind::Background),
            info if info.automation_id == "CalculatorExpression" => Some(CalculatorElementKind::ValueDisplay),
            info if info.automation_id == "CalculatorResults" => Some(CalculatorElementKind::ExpressionDisplay),
            _ => None,
        }
    }
}

pub trait CalculatorTheme {
    fn get_bounds(&self, element_kind: &CalculatorElementKind) -> Rect;
    fn get_background_color(&self, element_kind: &CalculatorElementKind) -> Color;
    fn get_text_style(&self, element_kind: &CalculatorElementKind) -> TextStyle;
    fn get_text_anchor(&self, element_kind: &CalculatorElementKind) -> Anchor;
    fn get_z_offset(&self, element_kind: &CalculatorElementKind) -> f32 {
        match element_kind {
            CalculatorElementKind::Background => 0.0,
            _ => 1.0,
        }
    }
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
            CalculatorElementKind::ClearEntryButton => Rect::new(201.0, -259.0, 262.0, -291.0),
            CalculatorElementKind::ClearButton => Rect::new(201.0, -259.0, 262.0, -291.0),
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
        match element_kind {
            CalculatorElementKind::ExpressionDisplay => TextStyle {
                font_size: 20.0,
                color: Color::GRAY,
                ..default()
            },
            CalculatorElementKind::EqualsButton => TextStyle {
                font_size: 32.0,
                color: Color::rgb(0.227, 0.106, 0.18),
                ..default()
            },
            _ => TextStyle {
                font_size: 32.0,
                color: Color::WHITE,
                ..default()
            },
        }
    }

    fn get_text_anchor(&self, element_kind: &CalculatorElementKind) -> Anchor {
        match element_kind {
            CalculatorElementKind::ExpressionDisplay => Anchor::CenterRight,
            _ => Anchor::Center,
        }
    }
}

/// When you hit a symbol (+-*/), the expression is updated
///
/// ```
/// format!("{existing}{value}{symbol}")
/// ```
/// and the value is simultaneously updated to
///
/// ```
/// format!("{}", eval(format!("{existing}{value}")))
/// ```
///
/// # Example
///
/// ```
/// let expression = "1+"
/// let value = "3"
/// let hidden_state = CalculatorHiddenState::Appending
/// let pressed = &CalculatorElementKind::PlusButton
///
/// let (new_expression, new_value, new_hidden_state) = calculator_state_transition(pressed, expression, value)
/// assert_eq!(new_expression, "1+3+")
/// assert_eq!(new_value, "4")
/// assert_eq!(new_hidden_state, CalculatorHiddenState::Previewing)
/// ```
///
/// The value is now showing "4", but the next time a digit is pressed,
/// the value will be clobbered with the digit and the hidden state will become `Appending`
#[derive(Debug, Reflect, Default, Clone, Eq, PartialEq)]
pub enum CalculatorHiddenState {
    #[default]
    Appending,
    Previewing,
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
pub struct Calculator {
    pub hidden_state: CalculatorHiddenState,
}
#[derive(Component, Debug, Reflect, Default, Clone, PartialEq)]
pub struct CalculatorStartMenuPanelButton;

#[derive(Component, Debug, Reflect, Default)]
pub struct CalculatorDisplay;
#[derive(Component, Debug, Reflect, Default)]
pub struct CalculatorExpression;
#[derive(Component, Debug, Reflect, Default)]
pub struct CalculatorButton;
#[derive(Component, Debug, Reflect, Default)]
pub struct CalculatorClearButton;
#[derive(Component, Debug, Reflect, Default)]
pub struct CalculatorClearEntryButton;
