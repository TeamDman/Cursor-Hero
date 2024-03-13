use bevy::prelude::*;
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use uiautomation::UIElement;

pub enum VSCodeResolveError {
    BadChildCount { tried_accessing: u32 },
    UI(uiautomation::Error),
    UnknownSideTabKind(String),
    UnknownState,
}
impl From<u32> for VSCodeResolveError {
    fn from(tried_accessing: u32) -> Self {
        VSCodeResolveError::BadChildCount { tried_accessing }
    }
}

impl From<uiautomation::Error> for VSCodeResolveError {
    fn from(e: uiautomation::Error) -> Self {
        VSCodeResolveError::UI(e)
    }
}
impl TryFrom<VecDeque<UIElement>> for VSCodeCrawlState {
    type Error = VSCodeResolveError;
    fn try_from(mut kids: VecDeque<UIElement>) -> Result<Self, Self::Error> {
        let state = match kids.len() {
            2 => VSCodeCrawlState::LeftTabClosed {
                tabs: kids.pop_front().ok_or(0u32)?,
                editor: kids.pop_front().ok_or(1u32)?,
            },
            3 => VSCodeCrawlState::LeftTabOpen {
                side_nav_tabs: kids.pop_front().ok_or(0u32)?,
                side_nav_view: kids.pop_front().ok_or(1u32)?,
                editor: kids.pop_front().ok_or(2u32)?,
            },
            _ => VSCodeCrawlState::Unknown,
        };
        Ok(state)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub enum View {
    Explorer {
        sticky: Vec<ExplorerItem>,
        items: Vec<ExplorerItem>,
    },
    Unknown {},
}
impl Display for View {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            View::Explorer { sticky, items } => {
                writeln!(f, "Explorer entries:")?;
                writeln!(f, "=== BEGIN STICKY ===")?;
                for item in sticky.iter().sorted_by_key(|x| x.bounds.min.y) {
                    writeln!(
                        f,
                        "{}- {} ({})",
                        " ".repeat(item.ui_level as usize),
                        item.label.clone()
                            + if matches!(item.kind, ExplorerItemKind::Directory { .. }) {
                                "/"
                            } else {
                                ""
                            },
                        item.path
                    )?;
                }
                writeln!(f, "=== END STICKY ===")?;
                for item in items.iter().sorted_by_key(|x| x.bounds.min.y) {
                    writeln!(
                        f,
                        "{}- {} ({})",
                        " ".repeat(item.ui_level as usize),
                        item.label.clone()
                            + if matches!(item.kind, ExplorerItemKind::Directory { .. }) {
                                "/"
                            } else {
                                ""
                            },
                        item.path
                    )?;
                }
                fmt::Result::Ok(())
            }
            View::Unknown {} => {
                writeln!(f, "Unknown view")?;
                fmt::Result::Ok(())
            }
        }
    }
}

pub enum VSCodeCrawlState {
    LeftTabClosed {
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
impl VSCodeCrawlState {
    pub fn get_side_nav_tabs_root_elem(&self) -> Result<&UIElement, VSCodeResolveError> {
        match self {
            VSCodeCrawlState::LeftTabClosed { tabs, .. } => Ok(tabs),
            VSCodeCrawlState::LeftTabOpen {
                side_nav_tabs: tabs,
                ..
            } => Ok(tabs),
            VSCodeCrawlState::Unknown => Err(VSCodeResolveError::UnknownState),
        }
    }
    pub fn get_side_nav_view_root_elem(&self) -> Result<&UIElement, VSCodeResolveError> {
        match self {
            VSCodeCrawlState::LeftTabClosed { tabs, .. } => Ok(tabs),
            VSCodeCrawlState::LeftTabOpen {
                side_nav_view: view,
                ..
            } => Ok(view),
            VSCodeCrawlState::Unknown => Err(VSCodeResolveError::UnknownState),
        }
    }
    pub fn get_editor_root_elem(&self) -> Result<&UIElement, VSCodeResolveError> {
        match self {
            VSCodeCrawlState::LeftTabClosed { editor, .. } => Ok(editor),
            VSCodeCrawlState::LeftTabOpen { editor, .. } => Ok(editor),
            VSCodeCrawlState::Unknown => Err(VSCodeResolveError::UnknownState),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub struct VSCodeWindowHeader {}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub struct VSCodeWindowBody {
    pub editor_area: EditorArea,
    pub side_nav: Vec<SideTab>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub struct VSCodeWindowFooter {}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub struct VSCodeWindow {
    pub focused: bool,
    pub header: VSCodeWindowHeader,
    pub body: VSCodeWindowBody,
    pub footer: VSCodeWindowFooter,
}

impl Display for VSCodeWindow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            ":D :D :D Visual Studio Code {} owo owo owo",
            if self.focused { "(focused)" } else { "" }
        )?;

        writeln!(f, "Side tabs:")?;
        for tab in self.body.side_nav.iter() {
            match tab {
                SideTab::Open { kind, view } => {
                    writeln!(f, "- (open) {:?} {{{{\n{}}}}}", kind, view)?;
                }
                SideTab::Closed { kind } => {
                    writeln!(f, "- {:?}", kind)?;
                }
            }
        }

        writeln!(f, "Editor groups:")?;
        for (i, group) in self.body.editor_area.groups.iter().enumerate() {
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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub enum SideTab {
    Closed { kind: SideTabKind },
    Open { kind: SideTabKind, view: View },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub enum ExplorerItemKind {
    File,
    Directory { expanded: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Reflect)]
pub struct ExplorerItem {
    pub label: String,
    pub path: String,
    pub ui_position_in_set: u32,
    pub ui_size_of_set: u32,
    pub ui_level: u32,
    pub bounds: IRect,
    pub kind: ExplorerItemKind,
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
    type Error = VSCodeResolveError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let before_first_lparen = s.split_once(" (").map(|x| x.0).unwrap_or(s.as_str());
        match before_first_lparen {
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
            "TODOs" => Ok(SideTabKind::Todo),
            _ => Err(VSCodeResolveError::UnknownSideTabKind(s)),
        }
    }
}
