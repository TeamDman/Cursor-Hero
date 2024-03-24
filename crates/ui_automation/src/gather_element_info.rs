use bevy::math::Rect;
use cursor_hero_ui_automation_types::prelude::ElementChildren;
use cursor_hero_ui_automation_types::prelude::ElementInfo;
use itertools::Itertools;
use std::collections::VecDeque;
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
    _depth: usize,
) -> Result<ElementInfo, Error> {
    let is_ancestor = |element: &UIElement| {
        ancestors
            .iter()
            .any(|ancestor| ancestor.get_runtime_id() == element.get_runtime_id())
    };
    let on_ancestor = is_ancestor(element);
    let mut element_info = gather_single_element_info(element)?;

    if on_ancestor {
        let children = element
            .gather_children(walker, &StopBehaviour::RootEndEncountered)
            .into_iter()
            .enumerate()
            .filter_map(|(i, child)| {
                if is_ancestor(&child) {
                    gather_tree(&child, walker, ancestors, _depth + 1).ok()
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

pub fn gather_single_element_info(element: &UIElement) -> Result<ElementInfo, uiautomation::Error> {
    let name = element.get_name()?;
    let bb = element.get_bounding_rectangle()?;
    let class_name = element.get_classname()?;
    let control_type = element.get_control_type()?.into();
    let localized_control_type = element.get_localized_control_type()?;
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
        control_type,
        localized_control_type,
        class_name,
        automation_id,
        runtime_id,
        children: None,
        drill_id: None,
    };
    Ok(info)
}
