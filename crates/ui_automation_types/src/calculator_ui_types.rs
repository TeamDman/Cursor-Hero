use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::prelude::ElementInfo;

#[derive(Debug, Reflect, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalculatorState {
    pub expression: ElementInfo,
    pub display: ElementInfo,
    pub one_button: ElementInfo,
    pub two_button: ElementInfo,
    pub three_button: ElementInfo,
    pub four_button: ElementInfo,
    pub five_button: ElementInfo,
    pub six_button: ElementInfo,
    pub seven_button: ElementInfo,
    pub eight_button: ElementInfo,
    pub nine_button: ElementInfo,
    pub zero_button: ElementInfo,
    pub add_button: ElementInfo,
    pub subtract_button: ElementInfo,
    pub multiply_button: ElementInfo,
    pub divide_button: ElementInfo,
    pub equals_button: ElementInfo,
    pub clear_button: ElementInfo,
    pub backspace_button: ElementInfo,
    pub memory_clear_button: ElementInfo,
    pub memory_recall_button: ElementInfo,
    pub memory_store_button: ElementInfo,
    pub memory_add_button: ElementInfo,
    pub memory_subtract_button: ElementInfo,
    pub decimal_button: ElementInfo,
    pub left_paren_button: ElementInfo,
    pub right_paren_button: ElementInfo,
    pub square_root_button: ElementInfo,
    pub factorial_button: ElementInfo,
    pub ln_button: ElementInfo,
    pub log_button: ElementInfo,
    pub pi_button: ElementInfo,
    pub e_button: ElementInfo,
    pub abs_button: ElementInfo,
    pub x_y_button: ElementInfo,
    pub ten_to_x_button: ElementInfo,
    pub square_button: ElementInfo,

}
impl std::fmt::Display for CalculatorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Calculator ({}{})", self.expression.name, self.display.name)
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
