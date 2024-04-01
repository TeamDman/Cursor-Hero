use cursor_hero_ui_automation_types::prelude::*;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
use anyhow::Result;
pub(crate) fn resolve_calculator(
    elem: &UIElement,
    automation: &UIAutomation,
    focused: bool,
) -> Result<AppWindow> {
    let walker = automation.create_tree_walker()?;
    let root = elem;

    let group = root.drill(&walker, vec![1,2,1])?;
    let expression = group.drill(&walker, vec![0])?.get_name()?.strip_prefix("Expression is ").unwrap_or_default().to_string();
    let display = group.drill(&walker, vec![1,0])?.get_name()?;

    Ok(AppWindow::Calculator(CalculatorState {
        expression,
        display,
    }))
}
