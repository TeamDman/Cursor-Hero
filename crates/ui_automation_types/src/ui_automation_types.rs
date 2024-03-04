use bevy::prelude::*;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use uiautomation::controls::ControlType;
use uiautomation::UIElement;

#[derive(Debug, Reflect, Clone)]
pub struct Taskbar {
    pub entries: Vec<TaskbarEntry>,
}
#[derive(Debug, Reflect, Clone)]
pub struct TaskbarEntry {
    pub name: String,
    pub bounds: IRect,
}

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
                write!(f, "Visual Studio Code: {:?}", elem.get_name())
                // match get_tree_string(elem) {
                //     Ok(text) => write!(f, "Visual Studio Code: {}", text),
                //     Err(e) => write!(f, "Visual Studio Code: {:?}", e),
                // }
            }
            AppUIElement::Unknown(elem) => write!(f, "Unknown: {:?}", elem),
        }
    }
}

pub trait ToBevyIRect {
    fn to_bevy_irect(&self) -> IRect;
}
impl ToBevyIRect for uiautomation::types::Rect {
    fn to_bevy_irect(&self) -> IRect {
        IRect {
            min: IVec2::new(self.get_left(), self.get_top()),
            max: IVec2::new(self.get_right(), self.get_bottom()),
        }
    }
}

#[derive(Debug, Clone, Reflect)]
// #[reflect(no_field_bounds)] //https://github.com/bevyengine/bevy/issues/8965
pub struct ElementInfo {
    pub name: String,
    pub bounding_rect: Rect,
    pub control_type: String,
    pub class_name: String,
    pub automation_id: String,
    pub runtime_id: Vec<i32>,
    #[reflect(ignore)]
    pub children: Vec<ElementInfo>,
}
