use std::collections::VecDeque;

use crate::ToBevyIRect;
use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::reflect::Reflect;
use uiautomation::controls::ControlType;
use uiautomation::types::Point;
use uiautomation::types::UIProperty;
use uiautomation::variants::Variant;
use uiautomation::UIAutomation;
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
                .to_bevy_irect(),
        })
        .collect();
    Ok(Taskbar { entries })
}

impl ToBevyIRect for uiautomation::types::Rect {
    fn to_bevy_irect(&self) -> IRect {
        IRect {
            min: IVec2::new(self.get_left(), self.get_top()),
            max: IVec2::new(self.get_right(), self.get_bottom()),
        }
    }
}

pub fn find_element_at(pos: IVec2) -> Result<UIElement, uiautomation::Error> {
    let automation = UIAutomation::new().unwrap();
    automation.element_from_point(Point::new(pos.x, pos.y))
}

pub fn gather_elements_at(pos: IVec2) -> Result<Vec<(UIElement, usize)>, uiautomation::Error> {
    let automation = UIAutomation::new().unwrap();
    let walker = automation.create_tree_walker()?;
    let start = automation.element_from_point(Point::new(pos.x, pos.y))?;
    let mut rtn = vec![];
    let mut next = VecDeque::new();
    next.push_back((start, 0));
    while let Some((elem, depth)) = next.pop_front() {
        rtn.push((elem.clone(), depth));
        if let Ok(child) = walker.get_first_child(&elem) {
            next.push_back((child.clone(), depth + 1));
            let mut next_sibling = child;
            while let Ok(sibling) = walker.get_next_sibling(&next_sibling) {
                next.push_back((sibling.clone(), depth + 1));
                next_sibling = sibling;
            }
        }
    }
    Ok(rtn)
}

// pub fn get_element_from_identifier(id: &str) -> Result<UIElement, uiautomation::Error> {
//     let automation = UIAutomation::new().unwrap();
//     // find the elem.get_automation_id() that matches id
//     let filter = automation.create_property_condition(
//         uiautomation::types::UIProperty::AutomationId,
//         uiautomation::variants::Variant::from(id),
//         None,
//     )?;
//     let walker = automation.filter_tree_walker(filter)?;
//     let root = automation.get_root_element()?;
//     let elem = find_recursive(&walker, &root)?;

// }

// fn find_recursive(walker: &UITreeWalker, element: &UIElement) -> Result<UIElement, uiautomation::Error> {
//     if element.get_automation_id()? == id {
//         return Ok(element);
//     }

//     if let Ok(child) = walker.get_first_child(&element) {
//         if let Ok(elem) = find_recursive(walker, &child) {
//             return Ok(elem);
//         }

//         let mut next = child;
//         while let Ok(sibling) = walker.get_next_sibling(&next) {
//             if let Ok(elem) = find_recursive(walker, &sibling) {
//                 return Ok(elem);
//             }

//             next = sibling;
//         }
//     }

//     Err(uiautomation::Error::from_win32(0))
// }

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
