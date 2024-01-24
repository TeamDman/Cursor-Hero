use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::reflect::Reflect;
use uiautomation::controls::ControlType;
use uiautomation::types::Point;
use uiautomation::types::UIProperty;
use uiautomation::variants::Variant;
use uiautomation::UIAutomation;

use crate::win_window::ToBevyRect;

pub struct Taskbar {
    pub entries: Vec<TaskbarEntry>,
}
#[derive(Debug, Reflect, Clone)]
pub struct TaskbarEntry {
    pub name: String,
    pub bounds: IRect,
}
pub fn get_taskbar() -> Result<Taskbar, uiautomation::Error> {
    let automation = UIAutomation::new()?;
    let root = automation.get_root_element()?;
    let taskbar_matcher = automation
        .create_matcher()
        .from(root)
        .classname("MSTaskListWClass")
        .control_type(ControlType::ToolBar);
    let taskbar = taskbar_matcher.find_first()?;
    let taskbar_entry_filter = automation.create_property_condition(
        UIProperty::ControlType,
        Variant::from(ControlType::Button as i32),
        None,
    )?;
    let taskbar_entry_walker = automation.filter_tree_walker(taskbar_entry_filter)?;

    let mut taskbar_entries = Vec::new();
    if let Ok(first) = taskbar_entry_walker.get_first_child(&taskbar)
        && let Ok(last) = taskbar_entry_walker.get_last_child(&taskbar)
    {
        taskbar_entries.push(first.clone());
        let mut next = first;
        while let Ok(sibling) = taskbar_entry_walker.get_next_sibling(&next) {
            taskbar_entries.push(sibling.clone());
            next = sibling;
            if next.get_runtime_id() == last.get_runtime_id() {
                break;
            }
        }
    }

    let entries = taskbar_entries
        .into_iter()
        .map(|entry| TaskbarEntry {
            name: entry.get_name().unwrap_or_default(),
            bounds: entry
                .get_bounding_rectangle()
                .unwrap_or_default()
                .to_bevy_rect(),
        })
        .collect();
    Ok(Taskbar { entries })
}

impl ToBevyRect for uiautomation::types::Rect {
    fn to_bevy_rect(&self) -> IRect {
        IRect {
            min: IVec2::new(self.get_left(), self.get_top()),
            max: IVec2::new(self.get_right(), self.get_bottom()),
        }
    }
}

pub fn get_element_at(pos: IVec2) -> Result<uiautomation::UIElement, uiautomation::Error> {
    let automation = UIAutomation::new()?;
    let element = automation.element_from_point(Point::new(pos.x, pos.y))?;
    Ok(element)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_taskbar() {
        let taskbar = get_taskbar().unwrap();
        assert!(taskbar.entries.len() > 0);
        // print the entries
        for entry in taskbar.entries {
            println!("entry: {:?}", entry);
        }
    }
}
