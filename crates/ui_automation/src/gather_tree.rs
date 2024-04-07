use cursor_hero_ui_automation_types::prelude::DrillId;
use cursor_hero_ui_automation_types::prelude::ElementInfo;
use itertools::Itertools;
use std::collections::VecDeque;
use uiautomation::Error;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;

use crate::gather_children::GatherChildrenable;
use crate::gather_children::StopBehaviour;
use crate::gather_element_info::gather_single_element_info;

pub struct GatherUITreeOkResult {
    pub ui_tree: ElementInfo,
    pub start_info: ElementInfo,
}
pub fn gather_info_tree_ancestry_filtered(
    start_element: UIElement,
) -> Result<GatherUITreeOkResult, Error> {
    let automation = UIAutomation::new()?;
    let walker = automation.create_tree_walker()?;
    let ancestors = gather_ui_ancestors_including_start(&start_element, &walker)?;
    // println!("ancestors: {:?}", ancestors);

    let root_element = ancestors
        .front()
        .ok_or(Error::new(-1, "No root element found"))?
        .clone();

    let ancestry_filter = |element: &UIElement| {
        ancestors
            .iter()
            .any(|ancestor| ancestor.get_runtime_id() == element.get_runtime_id())
    };
    let mut root_info = gather_info_tree_filtered(&root_element, &walker, &ancestry_filter, 0)?;
    root_info.drill_id = DrillId::Root;

    update_drill_ids(root_info.children.as_mut(), &DrillId::Root);

    let start_info = root_info
        .get_descendents()
        .into_iter()
        .chain(std::iter::once(&root_info))
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
    Ok(GatherUITreeOkResult {
        ui_tree: root_info,
        start_info,
    })
}

pub fn gather_info_tree(start_element: UIElement) -> Result<ElementInfo, Error> {
    // Setup
    let automation = UIAutomation::new()?;
    let walker = automation.create_tree_walker()?;

    // Get start drill id
    let start_drill_id = gather_info_tree_ancestry_filtered(start_element.clone())?
        .start_info
        .drill_id;

    // Get unfiltered tree
    let filter = |_: &UIElement| true;
    let mut tree = gather_info_tree_filtered(&start_element, &walker, &filter, 0)?;

    // Update drill IDs
    update_drill_ids(tree.children.as_mut(), &start_drill_id);
    tree.drill_id = start_drill_id;

    // Return
    Ok(tree)
}

pub fn gather_info_children(
    element: &UIElement,
    parent_drill_id: &DrillId,
    walker: &UITreeWalker,
) -> Result<Vec<ElementInfo>, Error> {
    let mut children = element
        .gather_children(walker, &StopBehaviour::EndOfSiblings)
        .into_iter()
        .enumerate()
        .filter_map(|(i, child)| {
            gather_single_element_info(&child)
                .ok()
                .map(|mut child_info| {
                    child_info.drill_id = DrillId::Child(vec![i].into_iter().collect());
                    child_info
                })
        })
        .collect_vec();
    update_drill_ids(Some(&mut children), parent_drill_id);
    Ok(children)
}

fn gather_ui_ancestors_including_start(
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

fn gather_info_tree_filtered(
    element: &UIElement,
    walker: &UITreeWalker,
    filter: &dyn Fn(&UIElement) -> bool,
    depth: usize,
) -> Result<ElementInfo, Error> {
    let mut element_info = gather_single_element_info(element)?;
    if filter(element) {
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
                if filter(&child) {
                    gather_info_tree_filtered(&child, walker, filter, depth + 1).ok()
                } else {
                    gather_single_element_info(&child).ok()
                }
                .map(|mut child_info| {
                    child_info.drill_id = vec![i].into();
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
