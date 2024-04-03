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
    }))
}
