use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use uiautomation::core::UICondition;
use uiautomation::UIAutomation;
pub use crate::vscode_ui_types::*;

pub trait HexList {
    fn to_hex_list(&self) -> String;
}
impl HexList for Vec<i32> {
    fn to_hex_list(&self) -> String {
        format!(
            "[{}]",
            self.iter()
                .map(|x| format!("{:X}", x).to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub struct UISnapshot {
    pub app_windows: Vec<AppWindow>,
}

impl Display for UISnapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "!!! UISnapshot !!!")?;
        for window in self.app_windows.iter() {
            write!(f, "{}", window)?;
        }
        fmt::Result::Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Reflect)]
pub enum AppWindow {
    VSCode(VSCodeWindow),
}

impl Display for AppWindow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppWindow::VSCode(window) => write!(f, "{}", window),
        }
    }
}

pub fn all_of(
    automation: &UIAutomation,
    conditions: Vec<UICondition>,
) -> Result<UICondition, uiautomation::Error> {
    let iter = conditions.into_iter();
    let mut current = automation.create_true_condition()?;

    for condition in iter {
        current = automation.create_and_condition(current, condition)?;
    }

    Ok(current)
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
    pub children: Option<Vec<ElementInfo>>,
}
