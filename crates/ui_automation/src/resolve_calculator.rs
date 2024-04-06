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
    let expression = group.drill(&walker, vec![0])?.try_into()?;
    let display = group.drill(&walker, vec![1, 0])?.try_into()?;

    Ok(AppWindow::Calculator(CalculatorState {
        expression,
        display,
        one_button: ElementInfo::default(),
        two_button: ElementInfo::default(),
        three_button: ElementInfo::default(),
        four_button: ElementInfo::default(),
        five_button: ElementInfo::default(),
        six_button: ElementInfo::default(),
        seven_button: ElementInfo::default(),
        eight_button: ElementInfo::default(),
        nine_button: ElementInfo::default(),
        zero_button: ElementInfo::default(),
        add_button: ElementInfo::default(),
        subtract_button: ElementInfo::default(),
        multiply_button: ElementInfo::default(),
        divide_button: ElementInfo::default(),
        equals_button: ElementInfo::default(),
        clear_button: ElementInfo::default(),
        backspace_button: ElementInfo::default(),
        memory_clear_button: ElementInfo::default(),
        memory_recall_button: ElementInfo::default(),
        memory_store_button: ElementInfo::default(),
        memory_add_button: ElementInfo::default(),
        memory_subtract_button: ElementInfo::default(),
        decimal_button: ElementInfo::default(),
        left_paren_button: ElementInfo::default(),
        right_paren_button: ElementInfo::default(),
        square_root_button: ElementInfo::default(),
        factorial_button: ElementInfo::default(),
        ln_button: ElementInfo::default(),
        log_button: ElementInfo::default(),
        pi_button: ElementInfo::default(),
        e_button: ElementInfo::default(),
        abs_button: ElementInfo::default(),
        x_y_button: ElementInfo::default(),
        ten_to_x_button: ElementInfo::default(),
        square_button: ElementInfo::default(),
    }))
}
