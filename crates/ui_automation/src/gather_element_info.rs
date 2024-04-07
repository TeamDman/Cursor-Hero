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