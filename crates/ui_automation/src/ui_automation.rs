use bevy::math::IVec2;
use cursor_hero_ui_automation_types::ui_automation_types::AppUIElement;
use std::collections::VecDeque;
use uiautomation::types::Point;
use uiautomation::UIAutomation;
use uiautomation::UIElement;

use crate::gather_children::gather_children;
use crate::gather_children::GatherChildrenStopBehaviour;

pub fn find_element_at(pos: IVec2) -> Result<UIElement, uiautomation::Error> {
    let automation = UIAutomation::new()?;
    automation.element_from_point(Point::new(pos.x, pos.y))
}

pub fn gather_elements_at(pos: IVec2) -> Result<Vec<(UIElement, usize)>, uiautomation::Error> {
    let automation = UIAutomation::new()?;
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
pub fn gather_toplevel_elements() -> Result<Vec<UIElement>, uiautomation::Error> {
    let automation = UIAutomation::new()?;
    let root = automation.get_root_element()?;
    let walker = automation.create_tree_walker()?;
    // println!("boutta gather children");
    let found = gather_children(&walker, &root, GatherChildrenStopBehaviour::EndOfSiblings);
    Ok(found)
}

pub fn gather_apps() -> Result<Vec<AppUIElement>, uiautomation::Error> {
    let elements = gather_toplevel_elements()?;
    Ok(elements.into_iter().map(AppUIElement::from).collect())
}

// pub fn get_element_from_identifier(id: &str) -> Result<UIElement, uiautomation::Error> {
//     let automation = UIAutomation::new()?;
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
    use crate::prelude::get_taskbar;

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
