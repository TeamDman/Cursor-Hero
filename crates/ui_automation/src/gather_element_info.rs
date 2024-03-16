use std::collections::VecDeque;

use bevy::log::*;
use bevy::math::Rect;
use cursor_hero_ui_automation_types::prelude::ElementChildren;
use cursor_hero_ui_automation_types::prelude::ElementInfo;
use itertools::Itertools;
use uiautomation::Error;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;

use crate::gather_children::GatherChildrenable;
use crate::gather_children::StopBehaviour;

pub fn gather_incomplete_ui_tree_starting_deep(
    start_element: UIElement,
) -> Result<ElementInfo, Error> {
    let automation = UIAutomation::new()?;
    let walker = automation.create_tree_walker()?;
    let ancestors = collect_ancestors(&start_element, &walker)?;

    let root_element = ancestors
        .front()
        .ok_or(Error::new(-1, "No root element found"))?
        .clone();
    let mut root_info = gather_tree(&root_element, &walker, &ancestors, 0)?;

    update_drill_ids(&mut root_info, &VecDeque::new());

    Ok(root_info)
}

fn collect_ancestors(
    element: &UIElement,
    walker: &UITreeWalker,
) -> Result<VecDeque<UIElement>, Error> {
    let mut ancestors = VecDeque::new();
    let mut current_element = Some(element.clone());
    while let Some(elem) = current_element {
        ancestors.push_front(elem.clone());
        current_element = walker.get_parent(&elem).ok();
    }
    Ok(ancestors)
}

fn gather_tree(
    element: &UIElement,
    walker: &UITreeWalker,
    ancestors: &VecDeque<UIElement>,
    depth: usize,
) -> Result<ElementInfo, Error> {
    let is_ancestor = |element: &UIElement| {
        ancestors
            .iter()
            .any(|ancestor| ancestor.get_runtime_id() == element.get_runtime_id())
    };
    let on_ancestor = is_ancestor(element);
    let mut element_info = gather_single_element_info(element)?;

    // TODO: remove depth param since on_ancestor shouldbe true for root elem
    if on_ancestor || depth == 0 {
        let children = element
            .gather_children(walker, &StopBehaviour::RootEndEncountered)
            .into_iter()
            .enumerate()
            .filter_map(|(i, child)| {
                // TODO: remove on_ancestor check here
                if on_ancestor || is_ancestor(&child) {
                    gather_tree(&child, walker, ancestors, depth + 1).ok()
                } else {
                    gather_single_element_info(&child).ok()
                }
                .map(|mut child_info| {
                    child_info.drill_id = Some(vec![i].into_iter().collect());
                    child_info
                })
            })
            .collect_vec();

        element_info.children = Some(ElementChildren {
            children,
            expanded: true,
        });
    }

    Ok(element_info)
}

// pub fn gather_element_info_starting_deep(
//     element: UIElement,
// ) -> Result<ElementInfo, uiautomation::Error> {
//     debug!("Gathering element info starting deep.");
//     let automation = UIAutomation::new()?;
//     let walker = automation.create_tree_walker()?;

//     debug!("Gathering element info for element: {:?}", element);
//     let mut root_info = gather_element_info_upwards_recursive(element, &walker)?;

//     // Start the `drill_id` update process here with an empty path
//     debug!("Updating drill IDs.");
//     update_drill_ids(&mut root_info, &VecDeque::new());

//     Ok(root_info)
// }

fn update_drill_ids(parent_info: &mut ElementInfo, ancestor_path: &VecDeque<usize>) {
    if let Some(children) = &mut parent_info.children {
        for child_info in &mut children.children {
            // Check if the child has a base drill_id set
            if let Some(base_drill_id) = &child_info.drill_id {
                let mut new_path = ancestor_path.clone();

                // The last entry in base_drill_id represents the child's position in its immediate parent
                if let Some(&child_position) = base_drill_id.back() {
                    new_path.push_back(child_position); // Use the position from the base drill_id

                    // Update the child's drill_id by concatenating the ancestor_path with its own position
                    child_info.drill_id = Some(new_path.clone());
                }

                // Recursively update this child's children
                update_drill_ids(child_info, &new_path);
            }
        }
    }
}

// fn gather_element_info_upwards_recursive(
//     element: UIElement,
//     walker: &UITreeWalker,
// ) -> Result<ElementInfo, uiautomation::Error> {
//     let parent = match walker.get_parent(&element) {
//         Ok(parent) => {
//             debug!("Parent found: {:?}", parent);
//             parent
//         },
//         Err(_) => {
//             // Base case, return info without children
//             debug!("No parent found, returning single element info.");
//             return gather_single_element_info(&element)
//         },
//     };

//     // Get parent info so we can attach children to it
//     let mut parent_info = gather_element_info_upwards_recursive(parent, walker)?;
//     debug!("Parent info: {:?}", parent_info);

//     // Now, handle the current element and its siblings
//     let siblings_info = gather_siblings_info(&element, walker);

//     // Assuming `gather_siblings_info` also updates the children of the parent_info
//     if parent_info.children.is_some() {
//         return Err(uiautomation::Error::new(-1, "Parent info already has children!"));
//     }

//     parent_info.children = Some(ElementChildren {
//         children: siblings_info,
//         expanded: true,
//     });

//     Ok(parent_info)
// }

// fn gather_siblings_info(
//     element: &UIElement,
//     walker: &UITreeWalker,
// ) -> Vec<ElementInfo> {
//     // Implement logic similar to your current loop, but adjusted for recursion.
//     // This should iterate over siblings, gather their info, and include the current element's info.
//     let Ok(parent) = walker.get_parent(&element) else {
//         return vec![];
//     };

//     let children = parent
//         .gather_children(&walker, &StopBehaviour::RootEndEncountered)
//         .into_iter()
//         .enumerate()
//         .map(|(i, child)| {
//             let mut info = gather_single_element_info(&child)?;
//             let mut drill_id = VecDeque::new();
//             drill_id.push_back(i);
//             // This correctly maps to the index within the UI element children.
//             // Because of filtering, we can't use the index within ElementInfo.children to build the drill ID later.
//             info.drill_id = Some(drill_id);
//             Ok::<_, uiautomation::Error>(info)
//         })
//         .filter_map(|x| x.ok())
//         .collect_vec();
//     children
// }

pub fn gather_single_element_info(element: &UIElement) -> Result<ElementInfo, uiautomation::Error> {
    let name = element.get_name()?;
    let bb = element.get_bounding_rectangle()?;
    let class_name = element.get_classname()?;
    let automation_id = element.get_automation_id()?;
    let runtime_id = element.get_runtime_id()?;

    let info = ElementInfo {
        selected: false,
        name,
        bounding_rect: Rect::new(
            bb.get_left() as f32,
            bb.get_top() as f32,
            bb.get_right() as f32,
            bb.get_bottom() as f32,
        ),
        control_type: class_name.clone(),
        class_name,
        automation_id,
        runtime_id,
        children: None,
        drill_id: None,
    };
    Ok(info)
}
