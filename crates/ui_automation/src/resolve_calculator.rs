use anyhow::Context;
use anyhow::Result;
use cursor_hero_ui_automation_types::prelude::*;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
pub(crate) fn resolve_calculator(
    elem: &UIElement,
    automation: &UIAutomation,
    _focused: bool,
) -> Result<AppSnapshot> {
    let walker = automation.create_tree_walker()?;
    let root = elem;
    let expression = root
        .drill(&walker, vec![1, 2, 1, 0])
        .context("expression")?
        .try_into()?;
    let display = root
        .drill(&walker, vec![1, 2, 1, 1, 0])
        .context("display")?
        .try_into()?;
    let zero_button = root
        .drill(&walker, vec![1, 2, 1, 20, 0])
        .context("zero_button")?
        .try_into()?;
    let one_button = root
        .drill(&walker, vec![1, 2, 1, 20, 1])
        .context("one_button")?
        .try_into()?;
    let two_button = root
        .drill(&walker, vec![1, 2, 1, 20, 2])
        .context("two_button")?
        .try_into()?;
    let three_button = root
        .drill(&walker, vec![1, 2, 1, 20, 3])
        .context("three_button")?
        .try_into()?;
    let four_button = root
        .drill(&walker, vec![1, 2, 1, 20, 4])
        .context("four_button")?
        .try_into()?;
    let five_button = root
        .drill(&walker, vec![1, 2, 1, 20, 5])
        .context("five_button")?
        .try_into()?;
    let six_button = root
        .drill(&walker, vec![1, 2, 1, 20, 6])
        .context("six_button")?
        .try_into()?;
    let seven_button = root
        .drill(&walker, vec![1, 2, 1, 20, 7])
        .context("seven_button")?
        .try_into()?;
    let eight_button = root
        .drill(&walker, vec![1, 2, 1, 20, 8])
        .context("eight_button")?
        .try_into()?;
    let nine_button = root
        .drill(&walker, vec![1, 2, 1, 20, 9])
        .context("nine_button")?
        .try_into()?;
    let equals_button = root
        .drill(&walker, vec![1, 2, 1, 18, 4])
        .context("equals_button")?
        .try_into()?;
    let plus_button = root
        .drill(&walker, vec![1, 2, 1, 18, 3])
        .context("plus_button")?
        .try_into()?;
    let minus_button = root
        .drill(&walker, vec![1, 2, 1, 18, 2])
        .context("minus_button")?
        .try_into()?;
    let multiply_by_button = root
        .drill(&walker, vec![1, 2, 1, 18, 1])
        .context("multiply_by_button")?
        .try_into()?;
    let divide_by_button = root
        .drill(&walker, vec![1, 2, 1, 18, 0])
        .context("divide_by_button")?
        .try_into()?;
    let positive_negative_button = root
        .drill(&walker, vec![1, 2, 1, 19])
        .context("positive_negative_button")?
        .try_into()?;
    let left_parenthesis_button = root
        .drill(&walker, vec![1, 2, 1, 15])
        .context("left_parenthesis_button")?
        .try_into()?;
    let right_parenthesis_button = root
        .drill(&walker, vec![1, 2, 1, 16])
        .context("right_parenthesis_button")?
        .try_into()?;
    let factorial_button = root
        .drill(&walker, vec![1, 2, 1, 17])
        .context("factorial_button")?
        .try_into()?;
    let reciprocal_button = root
        .drill(&walker, vec![1, 2, 1, 11])
        .context("reciprocal_button")?
        .try_into()?;
    let absolute_value_button = root
        .drill(&walker, vec![1, 2, 1, 12])
        .context("absolute_value_button")?
        .try_into()?;
    let exponential_button = root
        .drill(&walker, vec![1, 2, 1, 13])
        .context("exponential_button")?
        .try_into()?;
    let modulo_button = root
        .drill(&walker, vec![1, 2, 1, 14])
        .context("modulo_button")?
        .try_into()?;
    let display_controls_namedcontainerautomationpeer = root
        .drill(&walker, vec![1, 2, 1, 9])
        .context("drill")?
        .try_into()?;
    let clear_button = root
        .drill(&walker, vec![1, 2, 1, 9, 0])
        .context("clear_button")?
        .try_into()?;
    let eulers_number_button = root
        .drill(&walker, vec![1, 2, 1, 8])
        .context("eulers_number_button")?
        .try_into()?;
    let pi_button = root
        .drill(&walker, vec![1, 2, 1, 7])
        .context("pi_button")?
        .try_into()?;
    let inverse_function_togglebutton = root
        .drill(&walker, vec![1, 2, 1, 6])
        .context("inverse_function_togglebutton")?
        .try_into()?;
    let scientific_functions_namedcontainerautomationpeer = root
        .drill(&walker, vec![1, 2, 1, 10])
        .context("drill")?
        .try_into()?;
    let square_root_button = root
        .drill(&walker, vec![1, 2, 1, 10, 1])
        .context("square_root_button")?
        .try_into()?;
    let x_to_the_exponent_button = root
        .drill(&walker, vec![1, 2, 1, 10, 2])
        .context("x_to_the_exponent_button")?
        .try_into()?;
    let ten_to_the_exponent_button = root
        .drill(&walker, vec![1, 2, 1, 10, 3])
        .context("ten_to_the_exponent_button")?
        .try_into()?;
    let log_button = root
        .drill(&walker, vec![1, 2, 1, 10, 4])
        .context("log_button")?
        .try_into()?;
    let natural_log_button = root
        .drill(&walker, vec![1, 2, 1, 10, 5])
        .context("natural_log_button")?
        .try_into()?;
    let memory_controls_namedcontainerautomationpeer = root
        .drill(&walker, vec![1, 2, 1, 4])
        .context("drill")?
        .try_into()?;
    let memory_recall_button = root
        .drill(&walker, vec![1, 2, 1, 4, 1])
        .context("memory_recall_button")?
        .try_into()?;
    let memory_add_button = root
        .drill(&walker, vec![1, 2, 1, 4, 2])
        .context("memory_add_button")?
        .try_into()?;
    let memory_subtract_button = root
        .drill(&walker, vec![1, 2, 1, 4, 3])
        .context("memory_subtract_button")?
        .try_into()?;
    let memory_store_button = root
        .drill(&walker, vec![1, 2, 1, 4, 4])
        .context("memory_store_button")?
        .try_into()?;
    let open_memory_flyout_button = root
        .drill(&walker, vec![1, 2, 1, 4, 5])
        .context("open_memory_flyout_button")?
        .try_into()?;
    let degrees_toggle_button = root
        .drill(&walker, vec![1, 2, 1, 3, 0])
        .context("degrees_toggle_button")?
        .try_into()?;
    let scientific_notation_togglebutton = root
        .drill(&walker, vec![1, 2, 1, 3, 1])
        .context("scientific_notation_togglebutton")?
        .try_into()?;

    Ok(AppSnapshot::Calculator(CalculatorSnapshot {
        expression,
        display,
        zero_button,
        one_button,
        two_button,
        three_button,
        four_button,
        five_button,
        six_button,
        seven_button,
        eight_button,
        nine_button,
        equals_button,
        plus_button,
        minus_button,
        multiply_by_button,
        divide_by_button,
        positive_negative_button,
        left_parenthesis_button,
        right_parenthesis_button,
        factorial_button,
        reciprocal_button,
        absolute_value_button,
        exponential_button,
        modulo_button,
        display_controls_namedcontainerautomationpeer,
        clear_button,
        eulers_number_button,
        pi_button,
        inverse_function_togglebutton,
        scientific_functions_namedcontainerautomationpeer,
        square_root_button,
        x_to_the_exponent_button,
        ten_to_the_exponent_button,
        log_button,
        natural_log_button,
        memory_controls_namedcontainerautomationpeer,
        memory_recall_button,
        memory_add_button,
        memory_subtract_button,
        memory_store_button,
        open_memory_flyout_button,
        degrees_toggle_button,
        scientific_notation_togglebutton,
    }))
}
