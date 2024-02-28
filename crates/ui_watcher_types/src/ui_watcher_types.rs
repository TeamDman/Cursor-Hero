use uiautomation::controls::ControlType;
use uiautomation::UIElement;
use std::fmt::Formatter;
use std::fmt;
use std::fmt::Display;
use cursor_hero_winutils::ui_automation::get_tree_string;

pub enum AppUIElement {
    VSCode(UIElement),
    Unknown(UIElement),
}

impl From<UIElement> for AppUIElement {
    fn from(elem: UIElement) -> Self {
        let name = elem.get_name();
        let control_type = elem.get_control_type();
        let class_name = elem.get_classname();
        match (name, control_type, class_name) {
            (Ok(name), Ok(ControlType::Pane), Ok(class_name))
                if name.ends_with("Visual Studio Code") && class_name == "Chrome_WidgetWin_1" =>
            {
                AppUIElement::VSCode(elem)
            }
            _ => AppUIElement::Unknown(elem),
        }
    }
}

impl Display for AppUIElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppUIElement::VSCode(elem) => {
                match get_tree_string(elem) {
                    Ok(text) => write!(f, "Visual Studio Code: {}", text),
                    Err(e) => write!(f, "Visual Studio Code: {:?}", e),
                }
            },
            AppUIElement::Unknown(elem) => write!(f, "Unknown: {:?}", elem),
        }
    }
}