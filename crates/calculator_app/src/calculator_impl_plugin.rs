use bevy::prelude::*;
use cursor_hero_calculator_app_types::calculator_app_types::Calculator;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorDisplay;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorElementKind;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorExpression;
use cursor_hero_cursor_types::cursor_click_types::ClickEvent;
use cursor_hero_cursor_types::cursor_click_types::Way;
use cursor_hero_cursor_types::cursor_types::Cursor;
use cursor_hero_environment_types::environment_types::TrackedEnvironment;

pub struct CalculatorImplPlugin;

impl Plugin for CalculatorImplPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_clicks);
    }
}

fn handle_clicks(
    mut click_events: EventReader<ClickEvent>,
    calculator_query: Query<(&TrackedEnvironment, &Children), With<Calculator>>,
    button_query: Query<(&CalculatorElementKind, &Parent)>,
    cursor_query: Query<&TrackedEnvironment, With<Cursor>>,
    calculator_expression_query: Query<
        &Children,
        (With<CalculatorExpression>, Without<CalculatorDisplay>),
    >,
    calculator_value_query: Query<
        &Children,
        (With<CalculatorDisplay>, Without<CalculatorExpression>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for event in click_events.read() {
        // Only handle left click events
        let ClickEvent::Clicked {
            target_id,
            cursor_id,
            way: Way::Left,
        } = event
        else {
            continue;
        };

        // Get the button
        let Ok(button) = button_query.get(*target_id) else {
            continue;
        };
        let (button_kind, button_parent) = button;

        // Get the calculator
        let calculator_id = button_parent.get();
        let Ok(calculator) = calculator_query.get(calculator_id) else {
            continue;
        };
        let (calculator_environment, calculator_children) = calculator;

        // Get the cursor
        let Ok(cursor) = cursor_query.get(*cursor_id) else {
            continue;
        };
        let cursor_environment = cursor.environment_id;

        // warn and skip if the calculator is not in the same environment as the cursor
        if calculator_environment.environment_id != cursor_environment {
            warn!("Calculator {calculator_id:?} is not in the same environment as cursor {cursor_id:?} that clicked button {button_kind:?}  ({calculator_environment:?} vs {cursor_environment:?})");
            continue;
        }

        // Get the expression and value
        let Some(calculator_expression_id) = calculator_children
            .iter()
            .find(|child| calculator_expression_query.contains(**child))
        else {
            warn!("Calculator {calculator_id:?} does not have a calculator expression");
            continue;
        };
        let Ok(calculator_expression) = calculator_expression_query.get(*calculator_expression_id)
        else {
            continue;
        };
        let calculator_expression_children = calculator_expression;
        let Some(calculator_expression_text_id) = calculator_expression_children
            .iter()
            .find(|child| text_query.contains(**child))
        else {
            warn!("Calculator {calculator_id:?} does not have a calculator expression text");
            continue;
        };
        let Some(calculator_value_id) = calculator_children
            .iter()
            .find(|child| calculator_value_query.contains(**child))
        else {
            warn!("Calculator {calculator_id:?} does not have a calculator value");
            continue;
        };
        let Ok(calculator_value) = calculator_value_query.get(*calculator_value_id) else {
            continue;
        };
        let calculator_value_children = calculator_value;
        let Some(calculator_value_text_id) = calculator_value_children
            .iter()
            .find(|child| text_query.contains(**child))
        else {
            warn!("Calculator {calculator_id:?} does not have a calculator value text");
            continue;
        };

        let Ok([mut calculator_expression_text, mut calculator_value_text]) =
            text_query.get_many_mut([*calculator_expression_text_id, *calculator_value_text_id])
        else {
            warn!(
                "Calculator {calculator_id:?} does not have a calculator expression or value text"
            );
            continue;
        };

        // Get the current expression and value
        let Some(expression) = calculator_expression_text.sections.first_mut() else {
            warn!("Calculator {calculator_id:?} does not have a calculator expression section");
            continue;
        };
        let Some(value) = calculator_value_text.sections.first_mut() else {
            warn!("Calculator {calculator_id:?} does not have a calculator value section");
            continue;
        };

        // Transition the state
        calculator_state_transition(
            button_kind,
            &mut expression.value,
            &mut value.value,
        );
    }
}

/// Evaluate an expression. Return the string representation of the answer
/// 
/// Illegal operations like log(0) should return "Invalid input"
/// 
/// f128 should be used for all calculations.
/// 
/// e.g.,
/// 1 / 1.1 = 0.90909090909090909090909090909091
fn evaluate_expression(expression: &str) -> String {
    let x:f64 = 0.0;
    todo!()
}

fn calculator_state_transition(
    button_kind: &CalculatorElementKind,
    expression: &mut String,
    value: &mut String,
) {
    match button_kind {
        CalculatorElementKind::DigitButton(digit) => {
            value.push_str(&digit.to_string());
        }
        CalculatorElementKind::PlusButton => {
            expression.push_str(&format!("{} + ", value));
            value.clear();
        }
        CalculatorElementKind::EqualsButton => {
            expression.push_str(&format!("{}=", value));
            // let result = 
        }
        _ => {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_buttons() {
        let mut expression = String::new();
        let mut value = String::new();

        calculator_state_transition(&CalculatorElementKind::DigitButton(1), &mut expression, &mut value);
        assert_eq!(expression, "");
        assert_eq!(value, "1");

        calculator_state_transition(&CalculatorElementKind::DigitButton(2), &mut expression, &mut value);
        assert_eq!(expression, "");
        assert_eq!(value, "12");

        calculator_state_transition(&CalculatorElementKind::DigitButton(3), &mut expression, &mut value);
        assert_eq!(expression, "");
        assert_eq!(value, "123");
    }

    #[test]
    fn one_plus_two_equals() {
        let mut expression = String::new();
        let mut value = String::new();

        calculator_state_transition(&CalculatorElementKind::DigitButton(1), &mut expression, &mut value);
        calculator_state_transition(&CalculatorElementKind::PlusButton, &mut expression, &mut value);
        calculator_state_transition(&CalculatorElementKind::DigitButton(2), &mut expression, &mut value);
        calculator_state_transition(&CalculatorElementKind::EqualsButton, &mut expression, &mut value);
        assert_eq!(expression, "1 + 2=");
        assert_eq!(value, "3");
    }
}
