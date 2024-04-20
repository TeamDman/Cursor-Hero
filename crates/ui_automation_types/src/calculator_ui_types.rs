use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::prelude::ElementInfo;

#[derive(Debug, Reflect, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalculatorSnapshot {
    pub expression: ElementInfo,
    pub display: ElementInfo,
    pub zero_button: ElementInfo,
    pub one_button: ElementInfo,
    pub two_button: ElementInfo,
    pub three_button: ElementInfo,
    pub four_button: ElementInfo,
    pub five_button: ElementInfo,
    pub six_button: ElementInfo,
    pub seven_button: ElementInfo,
    pub eight_button: ElementInfo,
    pub nine_button: ElementInfo,
    pub equals_button: ElementInfo,
    pub plus_button: ElementInfo,
    pub minus_button: ElementInfo,
    pub multiply_by_button: ElementInfo,
    pub divide_by_button: ElementInfo,
    pub positive_negative_button: ElementInfo,
    pub left_parenthesis_button: ElementInfo,
    pub right_parenthesis_button: ElementInfo,
    pub factorial_button: ElementInfo,
    pub reciprocal_button: ElementInfo,
    pub absolute_value_button: ElementInfo,
    pub exponential_button: ElementInfo,
    pub modulo_button: ElementInfo,
    pub display_controls_namedcontainerautomationpeer: ElementInfo,
    pub clear_button: ElementInfo,
    pub eulers_number_button: ElementInfo,
    pub pi_button: ElementInfo,
    pub inverse_function_togglebutton: ElementInfo,
    pub scientific_functions_namedcontainerautomationpeer: ElementInfo,
    pub square_root_button: ElementInfo,
    pub x_to_the_exponent_button: ElementInfo,
    pub ten_to_the_exponent_button: ElementInfo,
    pub log_button: ElementInfo,
    pub natural_log_button: ElementInfo,
    pub memory_controls_namedcontainerautomationpeer: ElementInfo,
    pub memory_recall_button: ElementInfo,
    pub memory_add_button: ElementInfo,
    pub memory_subtract_button: ElementInfo,
    pub memory_store_button: ElementInfo,
    pub open_memory_flyout_button: ElementInfo,
    pub degrees_toggle_button: ElementInfo,
    pub scientific_notation_togglebutton: ElementInfo,
}
impl std::fmt::Display for CalculatorSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Calculator ({}{})",
            self.expression.name, self.display.name
        )
    }
}
