use crate::prelude::Calculator;
use crate::vscode_ui_types::*;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use uiautomation::core::UICondition;
use uiautomation::UIAutomation;


pub trait HexList {
    fn to_hex_list(&self) -> String;
}
impl HexList for Vec<i32> {
    fn to_hex_list(&self) -> String {
        format!(
            "[{}]",
            self.iter()
                .map(|x| format!("{:X}", x).to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub struct UISnapshot {
    pub app_windows: Vec<AppWindow>,
}

impl Display for UISnapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "!!! UISnapshot !!!")?;
        for window in self.app_windows.iter() {
            write!(f, "{}", window)?;
        }
        fmt::Result::Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub enum AppWindow {
    VSCode(VSCodeWindow),
    Calculator(Calculator),
}

impl Display for AppWindow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppWindow::VSCode(window) => write!(f, "{}", window),
            AppWindow::Calculator(window) => write!(f, "{}", window), 
        }
    }
}

pub fn all_of(
    automation: &UIAutomation,
    conditions: Vec<UICondition>,
) -> Result<UICondition, uiautomation::Error> {
    let iter = conditions.into_iter();
    let mut current = automation.create_true_condition()?;

    for condition in iter {
        current = automation.create_and_condition(current, condition)?;
    }

    Ok(current)
}

pub trait ToBevyIRect {
    fn to_bevy_irect(&self) -> IRect;
}
impl ToBevyIRect for uiautomation::types::Rect {
    fn to_bevy_irect(&self) -> IRect {
        IRect {
            min: IVec2::new(self.get_left(), self.get_top()),
            max: IVec2::new(self.get_right(), self.get_bottom()),
        }
    }
}

/// Defines enum for `windows::Win32::UI::Accessibility::UIA_CONTROLTYPE_ID`.
///
/// Contains the named constants used to identify Microsoft UI Automation control types.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum ControlType {
    /// Identifies the Button control type.
    Button = 50000u32,
    /// Identifies the Calendar control type.
    Calendar = 50001u32,
    /// Identifies the CheckBox control type.
    CheckBox = 50002u32,
    /// Identifies the ComboBox control type.
    ComboBox = 50003u32,
    /// Identifies the Edit control type.
    Edit = 50004u32,
    /// Identifies the Hyperlink control type.
    Hyperlink = 50005u32,
    /// Identifies the Image control type.
    Image = 50006u32,
    /// Identifies the ListItem control type.
    ListItem = 50007u32,
    /// Identifies the List control type.
    List = 50008u32,
    /// Identifies the Menu control type.
    Menu = 50009u32,
    /// Identifies the MenuBar control type.
    MenuBar = 50010u32,
    /// Identifies the MenuItem control type.
    MenuItem = 50011u32,
    /// Identifies the ProgressBar control type.
    ProgressBar = 50012u32,
    /// Identifies the RadioButton control type.
    RadioButton = 50013u32,
    /// Identifies the ScrollBar control type.
    ScrollBar = 50014u32,
    /// Identifies the Slider control type.
    Slider = 50015u32,
    /// Identifies the Spinner control type.
    Spinner = 50016u32,
    /// Identifies the StatusBar control type.
    StatusBar = 50017u32,
    /// Identifies the Tab control type.
    Tab = 50018u32,
    /// Identifies the TabItem control type.
    TabItem = 50019u32,
    /// Identifies the Text control type.
    Text = 50020u32,
    /// Identifies the ToolBar control type.
    ToolBar = 50021u32,
    /// Identifies the ToolTip control type.
    ToolTip = 50022u32,
    /// Identifies the Tree control type.
    Tree = 50023u32,
    /// Identifies the TreeItem control type.
    TreeItem = 50024u32,
    /// Identifies the Custom control type. For more information, see Custom Properties, Events, and Control Patterns.
    Custom = 50025u32,
    /// Identifies the Group control type.
    Group = 50026u32,
    /// Identifies the Thumb control type.
    Thumb = 50027u32,
    /// Identifies the DataGrid control type.
    DataGrid = 50028u32,
    /// Identifies the DataItem control type.
    DataItem = 50029u32,
    /// Identifies the Document control type.
    Document = 50030u32,
    /// Identifies the SplitButton control type.
    SplitButton = 50031u32,
    /// Identifies the Window control type.
    Window = 50032u32,
    /// Identifies the Pane control type.
    Pane = 50033u32,
    /// Identifies the Header control type.
    Header = 50034u32,
    /// Identifies the HeaderItem control type.
    HeaderItem = 50035u32,
    /// Identifies the Table control type.
    Table = 50036u32,
    /// Identifies the TitleBar control type.
    TitleBar = 50037u32,
    /// Identifies the Separator control type.
    Separator = 50038u32,
    /// Identifies the SemanticZoom control type. Supported starting with Windows 8.
    SemanticZoom = 50039u32,
    /// Identifies the AppBar control type. Supported starting with Windows 8.1.
    AppBar = 50040u32,
}
impl From<uiautomation::controls::ControlType> for ControlType {
    fn from(value: uiautomation::controls::ControlType) -> Self {
        match value {
            uiautomation::controls::ControlType::Button => ControlType::Button,
            uiautomation::controls::ControlType::Calendar => ControlType::Calendar,
            uiautomation::controls::ControlType::CheckBox => ControlType::CheckBox,
            uiautomation::controls::ControlType::ComboBox => ControlType::ComboBox,
            uiautomation::controls::ControlType::Edit => ControlType::Edit,
            uiautomation::controls::ControlType::Hyperlink => ControlType::Hyperlink,
            uiautomation::controls::ControlType::Image => ControlType::Image,
            uiautomation::controls::ControlType::ListItem => ControlType::ListItem,
            uiautomation::controls::ControlType::List => ControlType::List,
            uiautomation::controls::ControlType::Menu => ControlType::Menu,
            uiautomation::controls::ControlType::MenuBar => ControlType::MenuBar,
            uiautomation::controls::ControlType::MenuItem => ControlType::MenuItem,
            uiautomation::controls::ControlType::ProgressBar => ControlType::ProgressBar,
            uiautomation::controls::ControlType::RadioButton => ControlType::RadioButton,
            uiautomation::controls::ControlType::ScrollBar => ControlType::ScrollBar,
            uiautomation::controls::ControlType::Slider => ControlType::Slider,
            uiautomation::controls::ControlType::Spinner => ControlType::Spinner,
            uiautomation::controls::ControlType::StatusBar => ControlType::StatusBar,
            uiautomation::controls::ControlType::Tab => ControlType::Tab,
            uiautomation::controls::ControlType::TabItem => ControlType::TabItem,
            uiautomation::controls::ControlType::Text => ControlType::Text,
            uiautomation::controls::ControlType::ToolBar => ControlType::ToolBar,
            uiautomation::controls::ControlType::ToolTip => ControlType::ToolTip,
            uiautomation::controls::ControlType::Tree => ControlType::Tree,
            uiautomation::controls::ControlType::TreeItem => ControlType::TreeItem,
            uiautomation::controls::ControlType::Custom => ControlType::Custom,
            uiautomation::controls::ControlType::Group => ControlType::Group,
            uiautomation::controls::ControlType::Thumb => ControlType::Thumb,
            uiautomation::controls::ControlType::DataGrid => ControlType::DataGrid,
            uiautomation::controls::ControlType::DataItem => ControlType::DataItem,
            uiautomation::controls::ControlType::Document => ControlType::Document,
            uiautomation::controls::ControlType::SplitButton => ControlType::SplitButton,
            uiautomation::controls::ControlType::Window => ControlType::Window,
            uiautomation::controls::ControlType::Pane => ControlType::Pane,
            uiautomation::controls::ControlType::Header => ControlType::Header,
            uiautomation::controls::ControlType::HeaderItem => ControlType::HeaderItem,
            uiautomation::controls::ControlType::Table => ControlType::Table,
            uiautomation::controls::ControlType::TitleBar => ControlType::TitleBar,
            uiautomation::controls::ControlType::Separator => ControlType::Separator,
            uiautomation::controls::ControlType::SemanticZoom => ControlType::SemanticZoom,
            uiautomation::controls::ControlType::AppBar => ControlType::AppBar,
        }
    }
}

pub type RuntimeId = Vec<i32>;

#[derive(Debug, Eq, PartialEq, Clone, Reflect, Default, Hash)]
pub enum DrillId {
    Root,
    Child(VecDeque<usize>),
    #[default]
    Unknown,
}
impl From<Vec<usize>> for DrillId {
    fn from(value: Vec<usize>) -> Self {
        DrillId::Child(value.into())
    }
}
impl From<VecDeque<usize>> for DrillId {
    fn from(value: VecDeque<usize>) -> Self {
        DrillId::Child(value)
    }
}
impl From<Vec<i32>> for DrillId {
    fn from(value: Vec<i32>) -> Self {
        DrillId::Child(value.into_iter().map(|x| x as usize).collect())
    }
}
impl From<VecDeque<i32>> for DrillId {
    fn from(value: VecDeque<i32>) -> Self {
        DrillId::Child(value.into_iter().map(|x| x as usize).collect())
    }
}
impl std::fmt::Display for DrillId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrillId::Root => write!(f, "Root"),
            DrillId::Child(drill_id) => write!(
                f,
                "{}",
                drill_id
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            DrillId::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Reflect, PartialEq)]
// #[reflect(no_field_bounds)] //https://github.com/bevyengine/bevy/issues/8965
pub struct ElementInfo {
    pub name: String,
    pub bounding_rect: Rect,
    pub control_type: ControlType,
    pub localized_control_type: String,
    pub class_name: String,
    pub automation_id: String,
    #[reflect(ignore)]
    pub runtime_id: Vec<i32>,
    #[reflect(ignore)]
    pub drill_id: DrillId,
    #[reflect(ignore)]
    pub children: Option<Vec<ElementInfo>>,
}
impl Default for ElementInfo {
    fn default() -> Self {
        ElementInfo {
            name: "".to_string(),
            bounding_rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            control_type: ControlType::Pane,
            localized_control_type: "".to_string(),
            class_name: "".to_string(),
            automation_id: "".to_string(),
            runtime_id: vec![],
            drill_id: DrillId::Unknown,
            children: None,
        }
    }
}
impl ElementInfo {
    pub fn lookup_drill_id(&self, drill_id: DrillId) -> Option<&ElementInfo> {
        self.lookup_drill_id_inner(drill_id, 0)
    }
    fn lookup_drill_id_inner(&self, drill_id: DrillId, skip: usize) -> Option<&ElementInfo> {
        // println!("Looking in {} for {:?} ({:?})", self.name, drill_id.map(|x| x.iter().skip(skip).collect::<Vec<&usize>>()), drill_id);
        if self.drill_id == drill_id {
            return Some(self);
        }
        let DrillId::Child(drill_id) = drill_id else {
            return None;
        };
        if drill_id.is_empty() {
            return None;
        }
        let Some(children) = &self.children else {
            return None;
        };
        // println!("found children {:?}", children.children.iter().map(|x| x.drill_id.clone()).collect_vec());
        for child in children {
            let DrillId::Child(child_drill_id) = &child.drill_id else {
                continue;
            };
            if child_drill_id.back() == drill_id.iter().skip(skip).next() {
                if skip == drill_id.len() - 1 {
                    return Some(child);
                } else {
                    return child.lookup_drill_id_inner(DrillId::Child(drill_id.clone()), skip + 1);
                }
            }
        }
        None
    }
    pub fn lookup_drill_id_mut(&mut self, drill_id: DrillId) -> Option<&mut ElementInfo> {
        self.lookup_drill_id_mut_inner(drill_id, 0)
    }
    
    fn lookup_drill_id_mut_inner(&mut self, drill_id: DrillId, skip: usize) -> Option<&mut ElementInfo> {
        // println!("Looking in {} for {:?} ({:?})", self.name, drill_id.map(|x| x.iter().skip(skip).collect::<Vec<&usize>>()), drill_id);
        if self.drill_id == drill_id {
            return Some(self);
        }
        let DrillId::Child(drill_id) = drill_id else {
            return None;
        };
        if drill_id.is_empty() {
            return None;
        }
        let Some(ref mut children) = self.children else {
            return None;
        };
        // println!("found children {:?}", children.children.iter().map(|x| x.drill_id.clone()).collect_vec());
        for child in children.iter_mut() {
            let DrillId::Child(child_drill_id) = &child.drill_id else {
                continue;
            };
            if child_drill_id.back() == drill_id.iter().skip(skip).next() {
                if skip == drill_id.len() - 1 {
                    return Some(child);
                } else {
                    return child.lookup_drill_id_mut_inner(DrillId::Child(drill_id.clone()), skip + 1);
                }
            }
        }
        None
    }
    pub fn get_descendents(&self) -> Vec<&ElementInfo> {
        let mut descendents = vec![];
        if let Some(children) = &self.children {
            for child in children {
                descendents.push(child);
                descendents.extend(child.get_descendents());
            }
        }
        descendents
    }
}
// test lookup_drill_id
#[cfg(test)]
mod tests {
    #[test]
    fn test_lookup_drill_id() {
        use super::*;
        fn new_elem(name: &str, drill_id: Vec<usize>) -> ElementInfo {
            ElementInfo {
                name: name.to_string(),
                bounding_rect: Rect::new(0.0, 0.0, 100.0, 100.0),
                control_type: ControlType::Button,
                localized_control_type: "Button".to_string(),
                class_name: "Button".to_string(),
                automation_id: "Button".to_string(),
                runtime_id: vec![],
                drill_id: match drill_id.is_empty() {
                    true => DrillId::Root,
                    false => DrillId::Child(drill_id.into()),
                },
                children: None,
            }
        }
        let mut root = new_elem("root", vec![]);

        let mut a = new_elem("a", vec![0]);
        let a_a = new_elem("a_a", vec![0, 0]);
        let a_b = new_elem("a_b", vec![0, 1]);
        a.children = Some(vec![a_a.clone(), a_b.clone()]);

        let mut b = new_elem("b", vec![1]);
        let mut b_a = new_elem("b_a", vec![1, 0]);
        let b_a_a = new_elem("b_a_a", vec![1, 0, 0]);
        let b_a_b = new_elem("b_a_b", vec![1, 0, 1]);
        b_a.children = Some(vec![b_a_a.clone(), b_a_b.clone()]);
        let b_b = new_elem("b_b", vec![1, 1]);
        b.children = Some(vec![b_a.clone(), b_b.clone()]);

        root.children = Some(vec![a.clone(), b.clone()]);

        let items = vec![&root, &a, &a_a, &a_b, &b, &b_a, &b_a_a, &b_a_b, &b_b];
        for item in items {
            println!("Looking for {}", item.name);
            let found = root.lookup_drill_id(item.drill_id.clone());
            assert_eq!(found, Some(item));
            println!();
        }
    }
}