use cursor_hero_ui_automation_types::prelude::*;
use uiautomation::controls::ControlType;
use uiautomation::UIAutomation;
use uiautomation::UIElement;

use anyhow::Context;
use anyhow::Result;

use crate::resolve_calculator::resolve_calculator;
use crate::resolve_vscode::resolve_vscode;

pub(crate) fn resolve_app(
    elem: &UIElement,
    automation: &UIAutomation,
    focused: bool,
) -> Result<AppSnapshot> {
    // turning this off for now
    let should_resolve_vscode = false;

    match (
        elem.get_name(),
        elem.get_control_type(),
        elem.get_classname(),
    ) {
        (Ok(name), Ok(ControlType::Pane), Ok(class_name))
            if name.ends_with("Visual Studio Code")
                && class_name == "Chrome_WidgetWin_1"
                && should_resolve_vscode =>
        {
            resolve_vscode(elem, automation, focused).context("resolving vscode")
        }
        (Ok(name), Ok(ControlType::Window), Ok(class_name))
            if name == "Calculator" && class_name == "ApplicationFrameWindow" =>
        {
            resolve_calculator(elem, automation, focused).context("resolving calculator")
        }
        _ => Ok(AppSnapshot::Unknown),
    }
}
