use cursor_hero_ui_automation_types::prelude::*;
use uiautomation::controls::ControlType;
use uiautomation::UIAutomation;
use uiautomation::UIElement;

use crate::resolve_vscode::resolve_vscode;

pub(crate) fn resolve_app(
    elem: &UIElement,
    automation: &UIAutomation,
    focused: bool,
) -> Result<AppWindow, AppResolveError> {
    match (
        elem.get_name(),
        elem.get_control_type(),
        elem.get_localized_control_type(),
        elem.get_classname(),
        elem.get_bounding_rectangle().map(|r| r.to_bevy_irect()),
    ) {
        (Ok(name), Ok(ControlType::Pane), _, Ok(class_name), _)
            if name.ends_with("Visual Studio Code") && class_name == "Chrome_WidgetWin_1" =>
        {
            resolve_vscode(elem, automation, focused)
        }
        _ => Err(AppResolveError::NoMatch),
    }
}
