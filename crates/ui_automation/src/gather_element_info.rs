use bevy::math::Rect;
use cursor_hero_ui_automation_types::prelude::ElementInfo;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;

use crate::gather_children::gather_children;
use crate::gather_children::StopBehaviour;

pub fn gather_shallow_element_info(
    element: UIElement,
) -> Result<ElementInfo, uiautomation::errors::Error> {
    let name = element.get_name()?;
    let bb = element.get_bounding_rectangle()?;
    let class_name = element.get_classname()?;
    let automation_id = element.get_automation_id()?;
    let runtime_id = element.get_runtime_id()?;

    let info = ElementInfo {
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
    };
    Ok(info)
}

pub fn gather_deep_element_info(
    element: UIElement,
) -> Result<ElementInfo, uiautomation::errors::Error> {
    let automation = UIAutomation::new()?;
    let walker = automation.create_tree_walker()?;
    gather_deep_element_info_inner(element, &walker, &StopBehaviour::EndOfSiblings)
}

fn gather_deep_element_info_inner(
    element: UIElement,
    walker: &UITreeWalker,
    stop_behaviour: &StopBehaviour,
) -> Result<ElementInfo, uiautomation::errors::Error> {
    let name = element.get_name()?;
    let bb = element.get_bounding_rectangle()?;
    let class_name = element.get_classname()?;
    let automation_id = element.get_automation_id()?;
    let runtime_id = element.get_runtime_id()?;
    let mut children = vec![];

    for child in gather_children(walker, &element, stop_behaviour) {
        let child_info = gather_shallow_element_info(child)?;
        children.push(child_info);
    }

    let info = ElementInfo {
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
        children: Some(children),
    };
    Ok(info)
}
