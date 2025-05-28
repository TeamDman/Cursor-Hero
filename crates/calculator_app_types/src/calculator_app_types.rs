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
    DecimalSeparatorButton,
    PositiveNegativeButton,
    NaturalLogButton,
    LogButton,
    TenToTheExponentButton,
    XToTheExponentButton,
    DivideByButton,
    FactorialButton,
    RightParenthesisButton,
    LeftParenthesisButton,
    SquareRootButton,
    ModuloButton,
    ExponentialButton,
    AbsoluteValueButton,
    ReciprocalButton,
    SquareButton,
    BackspaceButton,
    EulersNumberButton,
    PiButton,
    AppIcon,
    AppTitle,
    CloseCalculatorButton,
    MaximizeCalculatorButton,
    MinimizeCalculatorButton,
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
            _ => None,
        }
    }
    pub fn get_text_from_state(&self, state: &CalculatorState) -> Option<String> {
        match self {
            CalculatorElementKind::ExpressionDisplay => Some(state.expression.clone()),
            CalculatorElementKind::ValueDisplay => Some(state.value.clone()),
            _ => None,
        }
    }
    #[rustfmt::skip]
    pub fn from_info(info: &ElementInfo) -> Option<CalculatorElementKind> {

        // consideration: name for expression and display should not be checked because they are dynamic
        match info {



            
            info if info.automation_id == "CalculatorExpression"=> Some(CalculatorElementKind::ValueDisplay),
            info if info.class_name == "Image" && info.automation_id == "AppIcon"           => Some(CalculatorElementKind::AppIcon),
            info if info.name == "Calculator" && info.class_name == "TextBlock" && info.automation_id == "AppName"=> Some(CalculatorElementKind::AppTitle),
            info if info.class_name == "LandmarkTarget"                                                                    => Some(CalculatorElementKind::Background),
            info if info.automation_id == "CalculatorResults"                                                              => Some(CalculatorElementKind::ExpressionDisplay),
            info if info.name == "Pi" && info.class_name == "Button" && info.automation_id == "piButton"                   => Some(CalculatorElementKind::PiButton),
            info if info.name == "Euler's number" && info.class_name == "Button" && info.automation_id == "eulerButton"    => Some(CalculatorElementKind::EulersNumberButton),
            info if info.name == "Clear" && info.class_name == "Button" && info.automation_id == "clearButton"             => Some(CalculatorElementKind::ClearButton),
            info if info.name == "Backspace" && info.class_name == "Button" && info.automation_id == "backSpaceButton"     => Some(CalculatorElementKind::BackspaceButton),
            info if info.name == "Square" && info.class_name == "Button" && info.automation_id == "xpower2Button"          => Some(CalculatorElementKind::SquareButton),
            info if info.name == "Reciprocal" && info.class_name == "Button" && info.automation_id == "invertButton"       => Some(CalculatorElementKind::ReciprocalButton),
            info if info.name == "Absolute value" && info.class_name == "Button" && info.automation_id == "absButton"      => Some(CalculatorElementKind::AbsoluteValueButton),
            info if info.name == "Exponential" && info.class_name == "Button" && info.automation_id == "expButton"         => Some(CalculatorElementKind::ExponentialButton),
            info if info.name == "Modulo" && info.class_name == "Button" && info.automation_id == "modButton"              => Some(CalculatorElementKind::ModuloButton),
            info if info.name == "Square root" && info.class_name == "Button" && info.automation_id == "squareRootButton"  => Some(CalculatorElementKind::SquareRootButton),
            info if info.name == "Left parenthesis" && info.class_name == "Button" && info.automation_id == "openParenthesisButton"=> Some(CalculatorElementKind::LeftParenthesisButton),
            info if info.name == "Right parenthesis" && info.class_name == "Button" && info.automation_id == "closeParenthesisButton"=> Some(CalculatorElementKind::RightParenthesisButton),
            info if info.name == "Factorial" && info.class_name == "Button" && info.automation_id == "factorialButton"=> Some(CalculatorElementKind::FactorialButton),
            info if info.name == "Divide by" && info.class_name == "Button" && info.automation_id == "divideButton"=> Some(CalculatorElementKind::DivideByButton),
            info if info.name == "'X' to the exponent" && info.class_name == "Button" && info.automation_id == "powerButton"=> Some(CalculatorElementKind::XToTheExponentButton),
            info if info.name == "Seven" && info.class_name == "Button" && info.automation_id == "num7Button"=> Some(CalculatorElementKind::DigitButton(7)),
            info if info.name == "Eight" && info.class_name == "Button" && info.automation_id == "num8Button"=> Some(CalculatorElementKind::DigitButton(8)),
            info if info.name == "Nine" && info.class_name == "Button" && info.automation_id == "num9Button"=> Some(CalculatorElementKind::DigitButton(9)),
            info if info.name == "Multiply by" && info.class_name == "Button" && info.automation_id == "multiplyButton"=> Some(CalculatorElementKind::MultiplyButton),
            info if info.name == "Ten to the exponent" && info.class_name == "Button" && info.automation_id == "powerOf10Button"=> Some(CalculatorElementKind::TenToTheExponentButton),
            info if info.name == "Four" && info.class_name == "Button" && info.automation_id == "num4Button"=> Some(CalculatorElementKind::DigitButton(4)),
            info if info.name == "Five" && info.class_name == "Button" && info.automation_id == "num5Button"=> Some(CalculatorElementKind::DigitButton(5)),
            info if info.name == "Six" && info.class_name == "Button" && info.automation_id == "num6Button"=> Some(CalculatorElementKind::DigitButton(6)),
            info if info.name == "Minus" && info.class_name == "Button" && info.automation_id == "minusButton"=> Some(CalculatorElementKind::MinusButton),
            info if info.name == "Log" && info.class_name == "Button" && info.automation_id == "logBase10Button"=> Some(CalculatorElementKind::LogButton),
            info if info.name == "One" && info.class_name == "Button" && info.automation_id == "num1Button"=> Some(CalculatorElementKind::DigitButton(1)),
            info if info.name == "Two" && info.class_name == "Button" && info.automation_id == "num2Button"=> Some(CalculatorElementKind::DigitButton(2)),
            info if info.name == "Three" && info.class_name == "Button" && info.automation_id == "num3Button"=> Some(CalculatorElementKind::DigitButton(3)),
            info if info.name == "Plus" && info.class_name == "Button" && info.automation_id == "plusButton"=> Some(CalculatorElementKind::PlusButton),
            info if info.name == "Natural log" && info.class_name == "Button" && info.automation_id == "logBaseEButton"=> Some(CalculatorElementKind::NaturalLogButton),
            info if info.name == "Positive negative" && info.class_name == "Button" && info.automation_id == "negateButton"=> Some(CalculatorElementKind::PositiveNegativeButton),
            info if info.name == "Zero" && info.class_name == "Button" && info.automation_id == "num0Button"=> Some(CalculatorElementKind::DigitButton(0)),
            info if info.name == "Decimal separator" && info.class_name == "Button" && info.automation_id == "decimalSeparatorButton"=> Some(CalculatorElementKind::DecimalSeparatorButton),
            info if info.name == "Equals" && info.class_name == "Button" && info.automation_id == "equalButton"=> Some(CalculatorElementKind::EqualsButton),

            info if info.name == "Equals" && info.class_name == "Button" => Some(CalculatorElementKind::EqualsButton),
            info if info.name == "Decimal separator" && info.class_name == "Button" => Some(CalculatorElementKind::DecimalSeparatorButton),
            info if info.name == "Zero" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(0)),
            info if info.name == "Positive negative" && info.class_name == "Button" => Some(CalculatorElementKind::PositiveNegativeButton),
            info if info.name == "Natural log" && info.class_name == "Button" => Some(CalculatorElementKind::NaturalLogButton),
            info if info.name == "Plus" && info.class_name == "Button" => Some(CalculatorElementKind::PlusButton),
            info if info.name == "Three" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(3)),
            info if info.name == "Two" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(2)),
            info if info.name == "One" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(1)),
            info if info.name == "Log" && info.class_name == "Button" => Some(CalculatorElementKind::LogButton),
            info if info.name == "Minus" && info.class_name == "Button" => Some(CalculatorElementKind::MinusButton),
            info if info.name == "Six" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(6)),
            info if info.name == "Five" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(5)),
            info if info.name == "Four" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(4)),
            info if info.name == "Ten to the exponent" && info.class_name == "Button" => Some(CalculatorElementKind::TenToTheExponentButton),
            info if info.name == "Multiply by" && info.class_name == "Button" => Some(CalculatorElementKind::MultiplyButton),
            info if info.name == "Nine" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(9)),
            info if info.name == "Eight" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(8)),
            info if info.name == "Seven" && info.class_name == "Button" => Some(CalculatorElementKind::DigitButton(7)),
            info if info.name == "'X' to the exponent" && info.class_name == "Button" => Some(CalculatorElementKind::XToTheExponentButton),
            info if info.name == "Divide by" && info.class_name == "Button" => Some(CalculatorElementKind::DivideByButton),
            info if info.name == "Factorial" && info.class_name == "Button" => Some(CalculatorElementKind::FactorialButton),
            info if info.name == "Right parenthesis" && info.class_name == "Button" => Some(CalculatorElementKind::RightParenthesisButton),
            info if info.name == "Left parenthesis" && info.class_name == "Button" => Some(CalculatorElementKind::LeftParenthesisButton),
            info if info.name == "Square root" && info.class_name == "Button" => Some(CalculatorElementKind::SquareRootButton),
            info if info.name == "Modulo" && info.class_name == "Button" => Some(CalculatorElementKind::ModuloButton),
            info if info.name == "Exponential" && info.class_name == "Button" => Some(CalculatorElementKind::ExponentialButton),
            info if info.name == "Absolute value" && info.class_name == "Button" => Some(CalculatorElementKind::AbsoluteValueButton),
            info if info.name == "Reciprocal" && info.class_name == "Button" => Some(CalculatorElementKind::ReciprocalButton),
            info if info.name == "Square" && info.class_name == "Button" => Some(CalculatorElementKind::SquareButton),
            info if info.name == "Backspace" && info.class_name == "Button" => Some(CalculatorElementKind::BackspaceButton),
            info if info.name == "Euler's number" && info.class_name == "Button" => Some(CalculatorElementKind::EulersNumberButton),
            info if info.name == "Pi" && info.class_name == "Button" => Some(CalculatorElementKind::PiButton),

            info if info.name == "Clear" && info.class_name == "Button" => Some(CalculatorElementKind::ClearButton),
            info if info.name == "Clear entry" && info.class_name == "Button" => Some(CalculatorElementKind::ClearEntryButton),

            info if info.automation_id == "AppIcon" && info.class_name == "Image" => Some(CalculatorElementKind::AppIcon),
            info if info.name == "Calculator" && info.class_name == "TextBlock" => Some(CalculatorElementKind::AppTitle),
            info if info.name == "Close Calculator" && info.class_name == "" => Some(CalculatorElementKind::CloseCalculatorButton),
            info if info.name == "Maximize Calculator" && info.class_name == "" => Some(CalculatorElementKind::MaximizeCalculatorButton),
            info if info.name == "Minimize Calculator" && info.class_name == "" => Some(CalculatorElementKind::MinimizeCalculatorButton),


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
            CalculatorElementKind::ValueDisplay => Rect::new(-1935.0, 36.0, -1935.0, 36.0),
            CalculatorElementKind::Background => Rect::new(8.0, -41.0, 328.0, -529.0),
            CalculatorElementKind::ExpressionDisplay => Rect::new(8.0, -105.0, 328.0, -152.0),
            CalculatorElementKind::PiButton => Rect::new(75.0, -269.0, 136.0, -304.0),
            CalculatorElementKind::EulersNumberButton => Rect::new(138.0, -269.0, 199.0, -304.0),
            CalculatorElementKind::ClearButton => Rect::new(201.0, -269.0, 262.0, -304.0),
            CalculatorElementKind::BackspaceButton => Rect::new(264.0, -269.0, 324.0, -304.0),
            CalculatorElementKind::SquareButton => Rect::new(12.0, -306.0, 73.0, -342.0),
            CalculatorElementKind::ReciprocalButton => Rect::new(75.0, -306.0, 136.0, -342.0),
            CalculatorElementKind::AbsoluteValueButton => Rect::new(138.0, -306.0, 199.0, -342.0),
            CalculatorElementKind::ExponentialButton => Rect::new(201.0, -306.0, 262.0, -342.0),
            CalculatorElementKind::ModuloButton => Rect::new(264.0, -306.0, 324.0, -342.0),
            CalculatorElementKind::SquareRootButton => Rect::new(12.0, -344.0, 73.0, -379.0),
            CalculatorElementKind::LeftParenthesisButton => Rect::new(75.0, -344.0, 136.0, -379.0),
            CalculatorElementKind::RightParenthesisButton => {
                Rect::new(138.0, -344.0, 199.0, -379.0)
            }
            CalculatorElementKind::FactorialButton => Rect::new(201.0, -344.0, 262.0, -379.0),
            CalculatorElementKind::DivideByButton => Rect::new(264.0, -344.0, 324.0, -379.0),
            CalculatorElementKind::XToTheExponentButton => Rect::new(12.0, -381.0, 73.0, -417.0),
            CalculatorElementKind::DigitButton(7) => Rect::new(75.0, -381.0, 136.0, -417.0),
            CalculatorElementKind::DigitButton(8) => Rect::new(138.0, -381.0, 199.0, -417.0),
            CalculatorElementKind::DigitButton(9) => Rect::new(201.0, -381.0, 262.0, -417.0),
            CalculatorElementKind::MultiplyButton => Rect::new(264.0, -381.0, 324.0, -417.0),
            CalculatorElementKind::TenToTheExponentButton => Rect::new(12.0, -419.0, 73.0, -454.0),
            CalculatorElementKind::DigitButton(4) => Rect::new(75.0, -419.0, 136.0, -454.0),
            CalculatorElementKind::DigitButton(5) => Rect::new(138.0, -419.0, 199.0, -454.0),
            CalculatorElementKind::DigitButton(6) => Rect::new(201.0, -419.0, 262.0, -454.0),
            CalculatorElementKind::MinusButton => Rect::new(264.0, -419.0, 324.0, -454.0),
            CalculatorElementKind::LogButton => Rect::new(12.0, -456.0, 73.0, -492.0),
            CalculatorElementKind::DigitButton(1) => Rect::new(75.0, -456.0, 136.0, -492.0),
            CalculatorElementKind::DigitButton(2) => Rect::new(138.0, -456.0, 199.0, -492.0),
            CalculatorElementKind::DigitButton(3) => Rect::new(201.0, -456.0, 262.0, -492.0),
            CalculatorElementKind::PlusButton => Rect::new(264.0, -456.0, 324.0, -492.0),
            CalculatorElementKind::NaturalLogButton => Rect::new(12.0, -494.0, 73.0, -529.0),
            CalculatorElementKind::PositiveNegativeButton => Rect::new(75.0, -494.0, 136.0, -529.0),
            CalculatorElementKind::DigitButton(0) => Rect::new(138.0, -494.0, 199.0, -529.0),
            CalculatorElementKind::DecimalSeparatorButton => {
                Rect::new(201.0, -494.0, 262.0, -529.0)
            }
            CalculatorElementKind::EqualsButton => Rect::new(264.0, -494.0, 324.0, -529.0),
            _ => Rect::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    fn get_background_color(&self, element_kind: &CalculatorElementKind) -> Color {
        let CalculatorThemeKind::WindowsDark = self;
        match element_kind {
            CalculatorElementKind::ValueDisplay => Color::rgb(0.0, 0.0, 0.0),
            CalculatorElementKind::Background => Color::rgb(0.1, 0.1, 0.1),
            CalculatorElementKind::ExpressionDisplay => Color::rgb(0.1, 0.1, 0.1),
            CalculatorElementKind::PiButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::EulersNumberButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::ClearButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::BackspaceButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::SquareButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::ReciprocalButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::AbsoluteValueButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::ExponentialButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::ModuloButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::SquareRootButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::LeftParenthesisButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::RightParenthesisButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::FactorialButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::DivideByButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::XToTheExponentButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::DigitButton(7) => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::DigitButton(8) => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::DigitButton(9) => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::MultiplyButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::TenToTheExponentButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::DigitButton(4) => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::DigitButton(5) => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::DigitButton(6) => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::MinusButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::LogButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::DigitButton(1) => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::DigitButton(2) => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::DigitButton(3) => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::PlusButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::NaturalLogButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::PositiveNegativeButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::DigitButton(0) => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::DecimalSeparatorButton => Color::rgb(0.2, 0.2, 0.2),
            CalculatorElementKind::EqualsButton => Color::rgb(0.9, 0.4, 0.7),
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
