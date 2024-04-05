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
        one_button: (ElementInfo::default(), "todo".to_string()),
        two_button: (ElementInfo::default(), "todo".to_string()),
        three_button: (ElementInfo::default(), "todo".to_string()),
        four_button: (ElementInfo::default(), "todo".to_string()),
        five_button: (ElementInfo::default(), "todo".to_string()),
        six_button: (ElementInfo::default(), "todo".to_string()),
        seven_button: (ElementInfo::default(), "todo".to_string()),
        eight_button: (ElementInfo::default(), "todo".to_string()),
        nine_button: (ElementInfo::default(), "todo".to_string()),
        zero_button: (ElementInfo::default(), "todo".to_string()),
        add_button: (ElementInfo::default(), "todo".to_string()),
        subtract_button: (ElementInfo::default(), "todo".to_string()),
        multiply_button: (ElementInfo::default(), "todo".to_string()),
        divide_button: (ElementInfo::default(), "todo".to_string()),
        equals_button: (ElementInfo::default(), "todo".to_string()),
        clear_button: (ElementInfo::default(), "todo".to_string()),
        backspace_button: (ElementInfo::default(), "todo".to_string()),
        memory_clear_button: (ElementInfo::default(), "todo".to_string()),
        memory_recall_button: (ElementInfo::default(), "todo".to_string()),
        memory_store_button: (ElementInfo::default(), "todo".to_string()),
        memory_add_button: (ElementInfo::default(), "todo".to_string()),
        memory_subtract_button: (ElementInfo::default(), "todo".to_string()),
        period_button: (ElementInfo::default(), "todo".to_string()),
        left_paren_button: (ElementInfo::default(), "todo".to_string()),
        right_paren_button: (ElementInfo::default(), "todo".to_string()),
        square_root_button: (ElementInfo::default(), "todo".to_string()),
        factorial_button: (ElementInfo::default(), "todo".to_string()),
        ln_button: (ElementInfo::default(), "todo".to_string()),
        log_button: (ElementInfo::default(), "todo".to_string()),
        pi_button: (ElementInfo::default(), "todo".to_string()),
        e_button: (ElementInfo::default(), "todo".to_string()),
        abs_button: (ElementInfo::default(), "todo".to_string()),
        x_y_button: (ElementInfo::default(), "todo".to_string()),
        ten_to_x_button: (ElementInfo::default(), "todo".to_string()),
        square_button: (ElementInfo::default(), "todo".to_string()),
    }))
}
