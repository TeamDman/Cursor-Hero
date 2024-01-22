use bevy::math::IRect;
use bevy::math::IVec2;
use uiautomation::controls::ControlType;
use uiautomation::types::UIProperty;
use uiautomation::variants::Variant;
use uiautomation::UIAutomation;

use crate::win_window::ToBevyRect;

#[derive(Debug)]
pub enum ToolbarError {
    UIAutomation(uiautomation::Error),
}

pub struct Toolbar {
    pub entries: Vec<ToolbarEntry>,
}
#[derive(Debug)]
pub struct ToolbarEntry {
    pub name: String,
    pub bounds: IRect,
}
pub fn get_toolbar() -> Result<Toolbar, ToolbarError> {
    let automation = UIAutomation::new().map_err(ToolbarError::UIAutomation)?;
    let root = automation
        .get_root_element()
        .map_err(ToolbarError::UIAutomation)?;
    let toolbar_matcher = automation
        .create_matcher()
        .from(root)
        .classname("MSTaskListWClass")
        .control_type(ControlType::ToolBar);
    let toolbar = toolbar_matcher
        .find_first()
        .map_err(ToolbarError::UIAutomation)?;
    let toolbar_entry_filter = automation
        .create_property_condition(
            UIProperty::ControlType,
            Variant::from(ControlType::Button as i32),
            None,
        )
        .map_err(ToolbarError::UIAutomation)?;
    let toolbar_entry_walker = automation
        .filter_tree_walker(toolbar_entry_filter)
        .map_err(ToolbarError::UIAutomation)?;

    let mut toolbar_entries = Vec::new();
    if let Ok(first) = toolbar_entry_walker.get_first_child(&toolbar)
        && let Ok(last) = toolbar_entry_walker.get_last_child(&toolbar)
    {
        toolbar_entries.push(first.clone());
        let mut next = first;
        while let Ok(sibling) = toolbar_entry_walker.get_next_sibling(&next) {
            toolbar_entries.push(sibling.clone());
            next = sibling;
            if next.get_runtime_id() == last.get_runtime_id() {
                break;
            }
        }
    }

    let entries = toolbar_entries
        .into_iter()
        .map(|entry| ToolbarEntry {
            name: entry.get_name().unwrap_or_default(),
            bounds: entry
                .get_bounding_rectangle()
                .unwrap_or_default()
                .to_bevy_rect(),
        })
        .collect();
    Ok(Toolbar { entries })
}

impl ToBevyRect for uiautomation::types::Rect {
    fn to_bevy_rect(&self) -> IRect {
        IRect {
            min: IVec2::new(self.get_left(), self.get_top()),
            max: IVec2::new(self.get_right(), self.get_bottom()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_toolbar() {
        let toolbar = get_toolbar().unwrap();
        assert!(toolbar.entries.len() > 0);
        // print the entries
        for entry in toolbar.entries {
            println!("entry: {:?}", entry);
        }
    }
}
