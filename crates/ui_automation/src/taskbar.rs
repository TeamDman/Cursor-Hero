use crate::gather_children::gather_children;
use crate::gather_children::StopBehaviour;
use crate::prelude::*;
use uiautomation::controls::ControlType;
use uiautomation::UIAutomation;

pub fn get_taskbar() -> Result<Taskbar, uiautomation::Error> {
    let automation = UIAutomation::new()?;
    let root = automation.get_root_element()?;
    let taskbar_matcher = automation
        .create_matcher()
        .from(root)
        .classname("MSTaskListWClass")
        .control_type(ControlType::ToolBar);
    let taskbar = taskbar_matcher.find_first()?;
    let taskbar_entry_walker = automation.create_tree_walker()?;
    let taskbar_entries = gather_children(
        &taskbar_entry_walker,
        &taskbar,
        &StopBehaviour::TaskbarEndEncountered,
    );
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
