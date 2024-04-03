use anyhow::Result;
use cursor_hero_ui_automation_types::prelude::*;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
pub(crate) fn resolve_calculator(
    elem: &UIElement,
    automation: &UIAutomation,
    _focused: bool,
) -> Result<AppWindow> {
    let walker = automation.create_tree_walker()?;
    let root = elem;

    let group = root.drill(&walker, vec![1, 2, 1])?;
    let expression_elem = group.drill(&walker, vec![0])?;
    let expression = expression_elem
        .get_name()?
        .strip_prefix("Expression is ")
        .unwrap_or_default()
        .to_string();
    let display = group.drill(&walker, vec![1, 0])?;
    let display_text = display.get_name()?;

    Ok(AppWindow::Calculator(CalculatorState {
        expression: (expression_elem.try_into()?, expression),
        display: (display.try_into()?, display_text),
        one_button: todo!(),
        two_button: todo!(),
        three_button: todo!(),
        four_button: todo!(),
        five_button: todo!(),
        six_button: todo!(),
        seven_button: todo!(),
        eight_button: todo!(),
        nine_button: todo!(),
        zero_button: todo!(),
        add_button: todo!(),
        subtract_button: todo!(),
        multiply_button: todo!(),
        divide_button: todo!(),
        equals_button: todo!(),
        clear_button: todo!(),
        backspace_button: todo!(),
        memory_clear_button: todo!(),
        memory_recall_button: todo!(),
        memory_store_button: todo!(),
        memory_add_button: todo!(),
        memory_subtract_button: todo!(),
        period_button: todo!(),
        left_paren_button: todo!(),
        right_paren_button: todo!(),
        square_root_button: todo!(),
        factorial_button: todo!(),
        ln_button: todo!(),
        log_button: todo!(),
        pi_button: todo!(),
        e_button: todo!(),
        abs_button: todo!(),
        x_y_button: todo!(),
        ten_to_x_button: todo!(),
        square_button: todo!(),
    }))
}
