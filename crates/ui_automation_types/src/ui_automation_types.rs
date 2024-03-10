use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use uiautomation::core::UICondition;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;

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

#[derive(Debug, Reflect, Clone)]
pub struct Taskbar {
    pub entries: Vec<TaskbarEntry>,
}
#[derive(Debug, Reflect, Clone)]
pub struct TaskbarEntry {
    pub name: String,
    pub bounds: IRect,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub struct EditorArea {
    pub groups: Vec<EditorGroup>,
}
impl EditorArea {
    pub fn get_expected_automation_id() -> &'static str {
        "workbench.parts.editor"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub struct EditorGroup {
    pub tabs: Vec<EditorTab>,
    pub content: Option<EditorContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub struct EditorTab {
    pub title: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub struct EditorContent {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub enum SideTabKind {
    Explorer,
    Search,
    SourceControl,
    RunAndDebug,
    Extensions,
    GitLens,
    Azure,
    Jupyter,
    Chat,
    GitHubActions,
    Todo,
}
impl SideTabKind {
    pub fn variants() -> Vec<SideTabKind> {
        vec![
            SideTabKind::Explorer,
            SideTabKind::Search,
            SideTabKind::SourceControl,
            SideTabKind::RunAndDebug,
            SideTabKind::Extensions,
            SideTabKind::GitLens,
            SideTabKind::Azure,
            SideTabKind::Jupyter,
            SideTabKind::Chat,
            SideTabKind::GitHubActions,
            SideTabKind::Todo,
        ]
    }
    pub fn get_view_automation_id(&self) -> Option<&str> {
        match self {
            SideTabKind::Explorer => Some("workbench.view.explorer"),
            _ => None,
        }
    }
}
impl TryFrom<String> for SideTabKind {
    type Error = AppResolveError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Explorer" => Ok(SideTabKind::Explorer),
            "Search" => Ok(SideTabKind::Search),
            "Source Control" => Ok(SideTabKind::SourceControl),
            "Run and Debug" => Ok(SideTabKind::RunAndDebug),
            "Extensions" => Ok(SideTabKind::Extensions),
            "GitLens" => Ok(SideTabKind::GitLens),
            "Azure" => Ok(SideTabKind::Azure),
            "Jupyter" => Ok(SideTabKind::Jupyter),
            "Chat" => Ok(SideTabKind::Chat),
            "GitHub Actions" => Ok(SideTabKind::GitHubActions),
            "Todo" => Ok(SideTabKind::Todo),
            _ => Err(AppResolveError::BadStructure(format!(
                "Unknown SideTabKind: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub enum SideTab {
    Closed { kind: SideTabKind },
    Open { kind: SideTabKind, view: View },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub enum View {
    Explorer {},
    Unknown {},
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
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

pub enum VSCodeState {
    Editor {
        tabs: UIElement,
        editor: UIElement,
    },
    LeftTabOpen {
        side_nav_tabs: UIElement,
        side_nav_view: UIElement,
        editor: UIElement,
    },
    Unknown,
}
impl VSCodeState {
    pub fn get_side_nav_tabs_root_elem(&self) -> Result<&UIElement, AppResolveError> {
        match self {
            VSCodeState::Editor { tabs, .. } => Ok(tabs),
            VSCodeState::LeftTabOpen {
                side_nav_tabs: tabs,
                ..
            } => Ok(tabs),
            VSCodeState::Unknown => Err(AppResolveError::BadStructure(
                "Unknown VSCodeState".to_string(),
            )),
        }
    }
    pub fn get_side_nav_view_root_elem(&self) -> Result<&UIElement, AppResolveError> {
        match self {
            VSCodeState::Editor { tabs, .. } => Ok(tabs),
            VSCodeState::LeftTabOpen {
                side_nav_view: view,
                ..
            } => Ok(view),
            VSCodeState::Unknown => Err(AppResolveError::BadStructure(
                "Unknown VSCodeState".to_string(),
            )),
        }
    }
    pub fn get_editor_root_elem(&self) -> Result<&UIElement, AppResolveError> {
        match self {
            VSCodeState::Editor { editor, .. } => Ok(editor),
            VSCodeState::LeftTabOpen { editor, .. } => Ok(editor),
            VSCodeState::Unknown => Err(AppResolveError::BadStructure(
                "Unknown VSCodeState".to_string(),
            )),
        }
    }
}
pub enum VSCodeStateResolveError {
    BadChildCount { tried_accessing: u32 },
}
impl From<u32> for VSCodeStateResolveError {
    fn from(tried_accessing: u32) -> Self {
        VSCodeStateResolveError::BadChildCount { tried_accessing }
    }
}
impl TryFrom<VecDeque<UIElement>> for VSCodeState {
    type Error = VSCodeStateResolveError;
    fn try_from(mut kids: VecDeque<UIElement>) -> Result<Self, Self::Error> {
        let state = match kids.len() {
            2 => VSCodeState::Editor {
                tabs: kids.pop_front().ok_or(0u32)?,
                editor: kids.pop_front().ok_or(1u32)?,
            },
            3 => VSCodeState::LeftTabOpen {
                side_nav_tabs: kids.pop_front().ok_or(0u32)?,
                side_nav_view: kids.pop_front().ok_or(1u32)?,
                editor: kids.pop_front().ok_or(2u32)?,
            },
            _ => VSCodeState::Unknown,
        };
        Ok(state)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Reflect)]
pub enum AppWindow {
    VSCode {
        focused: bool,
        editor_area: EditorArea,
        side_nav: Vec<SideTab>,
    },
}

impl Display for AppWindow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppWindow::VSCode {
                focused,
                editor_area,
                side_nav,
            } => {
                writeln!(
                    f,
                    ":D :D :D Visual Studio Code {} owo owo owo",
                    if *focused { "(focused)" } else { "" }
                )?;

                writeln!(f, "Side tabs:")?;
                for tab in side_nav.iter() {
                    match tab {
                        SideTab::Open { kind, view } => {
                            writeln!(f, "- (open) {:?} {{{{\n{:?}\n||}}", kind, view)?;
                        }
                        SideTab::Closed { kind } => {
                            writeln!(f, "- {:?}", kind)?;
                        }
                    }
                }

                writeln!(f, "Editor groups:")?;
                for (i, group) in editor_area.groups.iter().enumerate() {
                    writeln!(f, "Group {} tabs:", i + 1)?;
                    for tab in group.tabs.iter() {
                        if tab.active {
                            writeln!(f, "- (active) {}", tab.title)?;
                        } else {
                            writeln!(f, "- {}", tab.title)?;
                        }
                    }
                    if let Some(ref content) = group.content {
                        writeln!(
                            f,
                            "Group {} buffer:\n=======\n{}\n=======",
                            i + 1,
                            content.content
                        )?;
                    }
                }

                fmt::Result::Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub enum AppResolveError {
    UI(uiautomation::Error),
    BadStructure(String),
    NoMatch,
}
impl From<uiautomation::Error> for AppResolveError {
    fn from(e: uiautomation::Error) -> Self {
        AppResolveError::UI(e)
    }
}
impl From<DrillError> for AppResolveError {
    fn from(e: DrillError) -> Self {
        match e {
            DrillError::UI(e) => AppResolveError::UI(e),
            DrillError::EmptyPath => AppResolveError::BadStructure("Empty path".to_string()),
            DrillError::BadPath => AppResolveError::BadStructure("Bad path".to_string()),
            DrillError::OutOfBounds {
                given,
                max,
                error: e,
            } => AppResolveError::BadStructure(format!(
                "Out of bounds: given: {}, max: {}, error: {}",
                given, max, e
            )),
        }
    }
}
impl From<VSCodeStateResolveError> for AppResolveError {
    fn from(e: VSCodeStateResolveError) -> Self {
        match e {
            VSCodeStateResolveError::BadChildCount { tried_accessing } => {
                AppResolveError::BadStructure(format!(
                    "Bad child count when accessing index={}",
                    tried_accessing
                ))
            }
        }
    }
}

impl fmt::Display for AppResolveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write a descriptive message for the error.
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for AppResolveError {}

#[derive(Debug)]
pub enum GatherAppsError {
    UI(uiautomation::Error),
    NoneMatch,
    ResolveFailed(Vec<AppResolveError>),
}
impl From<uiautomation::Error> for GatherAppsError {
    fn from(e: uiautomation::Error) -> Self {
        GatherAppsError::UI(e)
    }
}

impl fmt::Display for GatherAppsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write a descriptive message for the error.
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for GatherAppsError {}

pub fn all_of(
    automation: &UIAutomation,
    conditions: Vec<UICondition>,
) -> Result<UICondition, uiautomation::Error> {
    let mut iter = conditions.into_iter();
    let mut current = automation.create_true_condition()?;

    while let Some(cond) = iter.next() {
        current = automation.create_and_condition(current, cond)?;
    }

    Ok(current)
}

pub enum DrillError {
    UI(uiautomation::Error),
    EmptyPath,
    BadPath,
    OutOfBounds {
        given: u32,
        max: u32,
        error: uiautomation::Error,
    },
}
impl From<uiautomation::Error> for DrillError {
    fn from(e: uiautomation::Error) -> Self {
        DrillError::UI(e)
    }
}
pub trait Drillable {
    fn drill(&self, walker: &UITreeWalker, path: Vec<i32>) -> Result<UIElement, DrillError>;
}
impl Drillable for UIElement {
    fn drill(&self, walker: &UITreeWalker, path: Vec<i32>) -> Result<UIElement, DrillError> {
        let mut path = path
            .into_iter()
            .map(|x| x as u32)
            .collect::<VecDeque<u32>>();
        if path.iter().any(|x| (*x as i32) < 0) {
            return Err(DrillError::BadPath);
        }
        drill_inner(self, walker, &mut path)
    }
}
fn drill_inner(
    start: &UIElement,
    walker: &UITreeWalker,
    path: &mut VecDeque<u32>,
) -> Result<UIElement, DrillError> {
    let target_index = match path.pop_front() {
        Some(x) => x,
        None => return Err(DrillError::EmptyPath),
    };
    let mut child = walker.get_first_child(start)?;
    let mut i = 0;
    while i < target_index {
        i += 1;
        child = match walker.get_next_sibling(&child) {
            Ok(x) => x,
            Err(e) => {
                return Err(DrillError::OutOfBounds {
                    given: i,
                    max: target_index,
                    error: e,
                })
            }
        };
    }
    if path.is_empty() {
        Ok(child)
    } else {
        drill_inner(&child, walker, path)
    }
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

#[derive(Debug, Clone, Reflect)]
// #[reflect(no_field_bounds)] //https://github.com/bevyengine/bevy/issues/8965
pub struct ElementInfo {
    pub name: String,
    pub bounding_rect: Rect,
    pub control_type: String,
    pub class_name: String,
    pub automation_id: String,
    pub runtime_id: Vec<i32>,
    #[reflect(ignore)]
    pub children: Option<Vec<ElementInfo>>,
}
