use std::path::PathBuf;

use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct Explorer;

#[derive(Debug, Reflect)]
pub struct ExplorerState {
    pub path: PathBuf,
}

#[derive(Component, Debug, Reflect, Default, Clone, PartialEq)]
pub struct ExplorerStartMenuPanelButton;

/*
let address:_this_pc_toolbarwindow32 = root.drill(&walker, vec![1, 0, 2, 0, 0, 0]).context("address:_this_pc_toolbarwindow32")?.try_into()?;
let _ = root.drill(&walker, vec![3]).context("_")?.try_into()?;
let this_pc_shelltabwindowclass = root.drill(&walker, vec![2]).context("this_pc_shelltabwindowclass")?.try_into()?;
let _workerw = root.drill(&walker, vec![1]).context("_workerw")?.try_into()?;
let uiribbondocktop_uiribboncommandbardock = root.drill(&walker, vec![0]).context("uiribbondocktop_uiribboncommandbardock")?.try_into()?;

*/

#[derive(Debug, Reflect, Eq, PartialEq)]
pub enum ExplorerElementKind {
    NavigationBar,
    Background,
}
impl ExplorerElementKind {
    pub fn variants() -> Vec<Self> {
        vec![
            Self::NavigationBar,
            Self::Background,            
        ]
    }
    pub fn get_default_text(&self) -> String {
        match self {
            Self::Background => "".to_string(),
            Self::NavigationBar => "".to_string(),
        }
    }
    pub fn get_text_from_state(&self, state: &ExplorerState) -> Option<String> {
        match self {
            Self::NavigationBar => Some(state.path.to_string_lossy().to_string()),
            _ => None,
        }
    }
    pub fn get_name(&self) -> String {
        match self {
            Self::NavigationBar => "NavigationBar".to_string(),
            Self::Background => "Background".to_string(),
        }
    }
    pub fn from_identifier(name: &str) -> Option<ExplorerElementKind> {
        match name {
            _ => None,
        }
    }
}

pub trait ExplorerTheme {
    fn get_bounds(&self, element_kind: &ExplorerElementKind) -> Rect;
    fn get_background_color(&self, element_kind: &ExplorerElementKind) -> Color;
    fn get_text_style(&self, element_kind: &ExplorerElementKind) -> TextStyle;
}

#[derive(Debug, Reflect)]
pub enum ExplorerThemeKind {
    WindowsDark,
}

#[derive(Event, Debug, Reflect)]
pub struct SpawnExplorerRequestEvent {
    pub environment_id: Entity,
    pub theme: ExplorerThemeKind,
    pub state: ExplorerState,
    pub position: Vec2,
}
