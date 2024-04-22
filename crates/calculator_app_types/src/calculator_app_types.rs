use bevy::prelude::*;

#[derive(Debug, Reflect)]
pub enum CalculatorElementKind {
    ExpressionDisplay,
    ValueDisplay,
    DigitButton(u8),
    EqualsButton,
    MultiplyButton,
    DivideButton,
    PlusButton,
    MinusButton,
}
impl CalculatorElementKind {
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
        ]
    }
    pub fn get_default_text(&self) -> String {
        match self {
            CalculatorElementKind::ExpressionDisplay => "".to_string(),
            CalculatorElementKind::ValueDisplay => "0".to_string(),
            CalculatorElementKind::DigitButton(digit) => digit.to_string(),
            CalculatorElementKind::EqualsButton => "=".to_string(),
            CalculatorElementKind::MultiplyButton => "*".to_string(),
            CalculatorElementKind::DivideButton => "/".to_string(),
            CalculatorElementKind::PlusButton => "+".to_string(),
            CalculatorElementKind::MinusButton => "-".to_string(),
        }
    }
    pub fn get_text_from_state(&self, state: &CalculatorState) -> Option<String> {
        match self {
            CalculatorElementKind::ExpressionDisplay => Some(state.expression.clone()),
            CalculatorElementKind::ValueDisplay => Some(state.value.clone()),
            _ => None,
        }
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
            CalculatorElementKind::ExpressionDisplay => Rect::new(168.0, -94.0, 284.0, 19.0),
            CalculatorElementKind::ValueDisplay => Rect::new(168.0, -126.0, 320.0, 44.0),
            CalculatorElementKind::DigitButton(0) => Rect::new(168.0, -481.0, 61.0, 32.0),
            CalculatorElementKind::DigitButton(1) => Rect::new(105.0, -446.0, 61.0, 33.0),
            CalculatorElementKind::DigitButton(2) => Rect::new(168.0, -446.0, 61.0, 33.0),
            CalculatorElementKind::DigitButton(3) => Rect::new(231.0, -446.0, 61.0, 33.0),
            CalculatorElementKind::DigitButton(4) => Rect::new(105.0, -412.0, 61.0, 32.0),
            CalculatorElementKind::DigitButton(5) => Rect::new(168.0, -412.0, 61.0, 32.0),
            CalculatorElementKind::DigitButton(6) => Rect::new(231.0, -412.0, 61.0, 32.0),
            CalculatorElementKind::DigitButton(7) => Rect::new(105.0, -377.0, 61.0, 33.0),
            CalculatorElementKind::DigitButton(8) => Rect::new(168.0, -377.0, 61.0, 33.0),
            CalculatorElementKind::DigitButton(9) => Rect::new(231.0, -377.0, 61.0, 33.0),
            CalculatorElementKind::DigitButton(n) => Rect::new(0.0, 0.0, 0.0, 0.0),
            CalculatorElementKind::PlusButton => Rect::new(294.0, -446.0, 60.0, 33.0),
            CalculatorElementKind::MinusButton => Rect::new(294.0, -412.0, 60.0, 32.0),
            CalculatorElementKind::MultiplyButton => Rect::new(294.0, -377.0, 60.0, 33.0),
            CalculatorElementKind::DivideButton => Rect::new(294.0, -343.0, 60.0, 32.0),
            CalculatorElementKind::EqualsButton => Rect::new(294.0, -481.0, 60.0, 32.0),
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

#[derive(Event, Debug, Reflect)]
pub struct SpawnCalculatorRequestEvent {
    pub environment_id: Entity,
    pub theme: CalculatorThemeKind,
    pub state: CalculatorState,
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
