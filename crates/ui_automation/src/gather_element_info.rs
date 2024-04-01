use bevy::math::IRect;
use cursor_hero_ui_automation_types::prelude::DrillId;
use cursor_hero_ui_automation_types::prelude::ElementInfo;
use cursor_hero_ui_automation_types::prelude::RuntimeId;
use itertools::Itertools;
use std::collections::VecDeque;
use uiautomation::Error;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;

use crate::gather_children::GatherChildrenable;
use crate::gather_children::StopBehaviour;

pub struct GatherUITreeOkResult {
    pub ui_tree: ElementInfo,
    pub start_info: ElementInfo,
}
pub fn gather_incomplete_ui_tree_starting_deep(
    start_element: UIElement,
) -> Result<GatherUITreeOkResult, Error> {
    let automation = UIAutomation::new()?;
    let walker = automation.create_tree_walker()?;
    let ancestors = collect_ancestors(&start_element, &walker)?;
    // println!("ancestors: {:?}", ancestors);

    let root_element = ancestors
        .front()
        .ok_or(Error::new(-1, "No root element found"))?
        .clone();
    let mut root_info = gather_tree(&root_element, &walker, &ancestors, 0)?;
    root_info.drill_id = DrillId::Root;

    update_drill_ids(root_info.children.as_mut(), &DrillId::Root);

    let start_info = root_info
        .get_descendents()
        .into_iter()
        .find(|info| match start_element.get_runtime_id() {
            Ok(id) => info.runtime_id.0 == id,
            Err(_) => false,
        })
        .cloned();
    let Some(start_info) = start_info else {
        return Err(Error::new(
            -1,
            format!(
                "Start element {:?} (id: {:?}) not found in tree: {:?}",
                start_element,
                start_element.get_runtime_id(),
                root_info
            )
            .as_str(),
        ));
    };
    // let start_info = start_info.unwrap_or_else(|| root_info.clone());

    Ok(GatherUITreeOkResult {
        ui_tree: root_info,
        start_info,
    })
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

    if on_ancestor {
        let children = element
            .gather_children(
                walker,
                if depth == 0 {
                    &StopBehaviour::RootEndEncountered
                } else {
                    &StopBehaviour::EndOfSiblings
                },
            )
            .into_iter()
            .enumerate()
            .filter_map(|(i, child)| {
                if is_ancestor(&child) {
                    gather_tree(&child, walker, ancestors, depth + 1).ok()
                } else {
                    gather_single_element_info(&child).ok()
                }
                .map(|mut child_info| {
                    child_info.drill_id = DrillId::Child(vec![i].into_iter().collect());
                    child_info
                })
            })
            .collect_vec();

        element_info.children = Some(children);
    }

    Ok(element_info)
}

pub fn update_drill_ids(children: Option<&mut Vec<ElementInfo>>, ancestor_path: &DrillId) {
    if let Some(children) = children {
        for child_info in children.iter_mut() {
            // Check if the child has a base drill_id set
            if let DrillId::Child(base_drill_id) = &child_info.drill_id {
                let mut new_path = ancestor_path.clone();
                if let Some(&child_position) = base_drill_id.back() {
                    new_path = match new_path {
                        DrillId::Root | DrillId::Unknown => {
                            DrillId::Child(vec![child_position].into())
                        }
                        DrillId::Child(ref mut path) => {
                            let mut new_path = path.clone();
                            new_path.push_back(child_position);
                            DrillId::Child(new_path)
                        }
                    };

                    // Update the child's drill_id by concatenating the ancestor_path with its own position
                    child_info.drill_id = new_path.clone();
                }

                // Recursively update this child's children
                update_drill_ids(child_info.children.as_mut(), &new_path);
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
    let runtime_id = RuntimeId(element.get_runtime_id()?);

    let info = ElementInfo {
        name,
        bounding_rect: IRect::new(bb.get_left(), bb.get_top(), bb.get_right(), bb.get_bottom()),
        control_type,
        localized_control_type,
        class_name,
        automation_id,
        runtime_id,
        children: None,
        drill_id: DrillId::Unknown,
    };
    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uiautomation::UIAutomation;

    /// Discord doesn't play nice with new UIAutomaion
    ///
    /// Element children aren't shown like they are in the MSAA tree
    #[test]
    fn test_gather_discord_element_info() {
        let automation = UIAutomation::new().unwrap();
        let walker = automation.create_tree_walker().unwrap();
        let start = automation
            .element_from_point(uiautomation::types::Point::new(2359, 959))
            .unwrap();
        println!("start {:#?}", start);
        let info = gather_single_element_info(&start).unwrap();
        println!("info {:#?}", info);

        // let parent = walker.get_parent(&start).unwrap();
        // let parent_info = gather_single_element_info(&parent).unwrap();
        // println!("parent_info {:#?}", parent_info);

        let ancestors = collect_ancestors(&start, &walker).unwrap();
        println!("got {} ancestors", ancestors.len());
        // println!("ancestors {:#?}", ancestors);
        for ancestor in ancestors.iter().skip(1) {
            print!(
                "ancestor {:?} (runtimeid={:?})\t",
                ancestor,
                ancestor.get_runtime_id()
            );
            for behaviour in vec![
                StopBehaviour::EndOfSiblings,
                // StopBehaviour::LastChildEncountered,
                // StopBehaviour::TaskbarEndEncountered,
                // StopBehaviour::RootEndEncountered,
            ] {
                let children = ancestor
                    .gather_children(&walker, &behaviour)
                    .into_iter()
                    .map(|child| gather_single_element_info(&child).unwrap())
                    .collect::<Vec<_>>();
                if children.is_empty() {
                    eprintln!("No children found using {:?}", behaviour);
                }
                println!("children using {:?} {:#?}", behaviour, children.len());
            }
        }

        let gathered = gather_incomplete_ui_tree_starting_deep(start).unwrap();
        // println!("tree {:#?}", gathered.ui_tree);
    }
}
