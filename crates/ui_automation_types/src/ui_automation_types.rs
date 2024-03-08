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

#[derive(Debug, Clone, Reflect)]
pub struct DetachableUIElement(Option<UIElement>);
impl Eq for DetachableUIElement {}
impl PartialEq for DetachableUIElement {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Some(x), Some(y)) => x.get_runtime_id() == y.get_runtime_id(),
            (None, None) => true,
            _ => false,
        }
    }
}
pub trait TryAsRef<T> {
    type Error;

    fn try_as_ref(&self) -> Result<&T, Self::Error>;
}
impl TryInto<UIElement> for DetachableUIElement {
    type Error = AppResolveError;
    fn try_into(self) -> Result<UIElement, Self::Error> {
        match self.0 {
            Some(x) => Ok(x),
            None => Err(AppResolveError::Detached),
        }
    }
}
impl TryAsRef<UIElement> for DetachableUIElement {
    type Error = AppResolveError;

    fn try_as_ref(&self) -> Result<&UIElement, Self::Error> {
        self.0.as_ref().ok_or(AppResolveError::Detached)
    }
}
impl From<UIElement> for DetachableUIElement {
    fn from(elem: UIElement) -> Self {
        DetachableUIElement(Some(elem))
    }
}
impl Serialize for DetachableUIElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_none()
    }
}
impl<'de> Deserialize<'de> for DetachableUIElement {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        Ok(DetachableUIElement(None))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub struct EditorArea {
    pub groups: Vec<EditorGroup>,
    // pub elem: DetachableUIElement,
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
    // pub elem: DetachableUIElement,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub struct EditorTab {
    pub title: String,
    pub active: bool,
    // pub elem: DetachableUIElement,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub struct EditorContent {
    pub content: String,
    // pub elem: DetachableUIElement,
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
    Closed {
        kind: SideTabKind,
        // button: DetachableUIElement,
    },
    Open {
        kind: SideTabKind,
        // button: DetachableUIElement,
        view: View,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub enum View {
    Explorer {
        // workbench.view.explorer
        // elem: DetachableUIElement,
    },
    Unknown {
        // elem: DetachableUIElement,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub struct UISnapshot {
    pub app_windows: Vec<AppWindow>,
}

impl Display for UISnapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "UISnapshot: {:?}", self.app_windows)
    }
}

pub enum VSCodeState {
    Editor {
        tabs: DetachableUIElement,
        editor: DetachableUIElement,
    },
    LeftTabOpen {
        side_nav_tabs: DetachableUIElement,
        side_nav_view: DetachableUIElement,
        editor: DetachableUIElement,
    },
    Unknown,
}
impl VSCodeState {
    pub fn get_side_nav_tabs_root_elem(&self) -> Result<&UIElement, AppResolveError> {
        match self {
            VSCodeState::Editor { tabs, .. } => Ok(tabs.try_as_ref()?),
            VSCodeState::LeftTabOpen {
                side_nav_tabs: tabs,
                ..
            } => Ok(tabs.try_as_ref()?),
            VSCodeState::Unknown => Err(AppResolveError::BadStructure(
                "Unknown VSCodeState".to_string(),
            )),
        }
    }
    pub fn get_side_nav_view_root_elem(&self) -> Result<&UIElement, AppResolveError> {
        match self {
            VSCodeState::Editor { tabs, .. } => Ok(tabs.try_as_ref()?),
            VSCodeState::LeftTabOpen {
                side_nav_view: view,
                ..
            } => Ok(view.try_as_ref()?),
            VSCodeState::Unknown => Err(AppResolveError::BadStructure(
                "Unknown VSCodeState".to_string(),
            )),
        }
    }
    pub fn get_editor_root_elem(&self) -> Result<&UIElement, AppResolveError> {
        match self {
            VSCodeState::Editor { editor, .. } => Ok(editor.try_as_ref()?),
            VSCodeState::LeftTabOpen { editor, .. } => Ok(editor.try_as_ref()?),
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
                tabs: kids.pop_front().ok_or(0u32)?.into(),
                editor: kids.pop_front().ok_or(1u32)?.into(),
            },
            3 => VSCodeState::LeftTabOpen {
                side_nav_tabs: kids.pop_front().ok_or(0u32)?.into(),
                side_nav_view: kids.pop_front().ok_or(1u32)?.into(),
                editor: kids.pop_front().ok_or(2u32)?.into(),
            },
            _ => VSCodeState::Unknown,
        };
        Ok(state)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Reflect)]
pub enum AppWindow {
    VSCode {
        // root: DetachableUIElement,
        focused: bool,
        editor_area: EditorArea,
        side_nav: Vec<SideTab>,
    },
}

impl Display for AppWindow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppWindow::VSCode { .. } => {
                write!(f, "Visual Studio Code: {:?}", self)
                // match get_tree_string(elem) {
                //     Ok(text) => write!(f, "Visual Studio Code: {}", text),
                //     Err(e) => write!(f, "Visual Studio Code: {:?}", e),
                // }
            } // AppWindow::VSCodeTerminal(elem) => {
              //     write!(f, "VSCodeTerminal: {:?}", elem.get_name())
              // }
              // AppWindow::VSCodeEditor(elem) => {
              //     write!(
              //         f,
              //         "VSCodeEditor: {:?}\n{}",
              //         elem.get_name(),
              //         elem.get_property_value(UIProperty::LegacyIAccessibleValue)
              //             .map(|v| v.to_string())
              //             .unwrap_or_default()
              //     )
              // }
        }
    }
}

#[derive(Debug)]
pub enum AppResolveError {
    UI(uiautomation::Error),
    BadStructure(String),
    NoMatch,
    Detached,
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
    enum FoldState {
        Start,
        First(UICondition),
        Current(UICondition),
    }
    let state = conditions
        .into_iter()
        .fold(Ok(FoldState::Start), |current, x| match current {
            Ok(FoldState::Start) => Ok(FoldState::First(x)),
            Ok(FoldState::First(c) | FoldState::Current(c)) => {
                Ok(FoldState::Current(automation.create_and_condition(c, x)?))
            }
            Err(e) => Err(e),
        });
    match state {
        Ok(FoldState::Start) => automation.create_true_condition(),
        Ok(FoldState::First(c)) => Ok(c),
        Ok(FoldState::Current(c)) => Ok(c),
        Err(e) => Err(e),
    }
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
