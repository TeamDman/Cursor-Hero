use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::prelude::ElementInfo;

#[derive(Debug, Reflect, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalculatorState {
    pub expression: (ElementInfo, String),
    pub display: (ElementInfo, String),
    pub one_button: (ElementInfo, String),
    pub two_button: (ElementInfo, String),
    pub three_button: (ElementInfo, String),
    pub four_button: (ElementInfo, String),
    pub five_button: (ElementInfo, String),
    pub six_button: (ElementInfo, String),
    pub seven_button: (ElementInfo, String),
    pub eight_button: (ElementInfo, String),
    pub nine_button: (ElementInfo, String),
    pub zero_button: (ElementInfo, String),
    pub add_button: (ElementInfo, String),
    pub subtract_button: (ElementInfo, String),
    pub multiply_button: (ElementInfo, String),
    pub divide_button: (ElementInfo, String),
    pub equals_button: (ElementInfo, String),
    pub clear_button: (ElementInfo, String),
    pub backspace_button: (ElementInfo, String),
    pub memory_clear_button: (ElementInfo, String),
    pub memory_recall_button: (ElementInfo, String),
    pub memory_store_button: (ElementInfo, String),
    pub memory_add_button: (ElementInfo, String),
    pub memory_subtract_button: (ElementInfo, String),
    pub decimal_button: (ElementInfo, String),
    pub left_paren_button: (ElementInfo, String),
    pub right_paren_button: (ElementInfo, String),
    pub square_root_button: (ElementInfo, String),
    pub factorial_button: (ElementInfo, String),
    pub ln_button: (ElementInfo, String),
    pub log_button: (ElementInfo, String),
    pub pi_button: (ElementInfo, String),
    pub e_button: (ElementInfo, String),
    pub abs_button: (ElementInfo, String),
    pub x_y_button: (ElementInfo, String),
    pub ten_to_x_button: (ElementInfo, String),
    pub square_button: (ElementInfo, String),

}
impl std::fmt::Display for CalculatorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Calculator ({}{})", self.expression.1, self.display.1)
    }
}


#[derive(Event, Debug, Reflect, Default)]
pub struct SpawnCalculatorRequest {
    calculator_state: CalculatorState,
}


#[derive(Component, Debug, Reflect, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Calculator;

#[derive(Component, Debug, Reflect, Default)]
pub struct NumberDisplayPanel;
