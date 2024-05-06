use std::path::PathBuf;

use bevy::prelude::*;
use cursor_hero_ui_automation_types::prelude::DrillId;

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
let title_bar = root.drill(&walker, vec![3]).context("_")?.try_into()?;
let body = root.drill(&walker, vec![2]).context("this_pc_shelltabwindowclass")?.try_into()?;
let top_bar = root.drill(&walker, vec![1]).context("_workerw")?.try_into()?;
let ribbon = root.drill(&walker, vec![0]).context("uiribbondocktop_uiribboncommandbardock")?.try_into()?;
*/

#[derive(Debug, Reflect, Eq, PartialEq)]
pub enum ExplorerElementKind {
    Window,
    TitleBar,
    Ribbon,
    TopBar,
    AddressBox,
    Body,
    Main,
    NavigationPane,
}
impl ExplorerElementKind {
    pub fn variants() -> Vec<Self> {
        vec![
            Self::Window,
            Self::TitleBar,
            Self::Ribbon,
            Self::TopBar,
            Self::AddressBox,
            Self::Body,
            Self::Main,
            Self::NavigationPane,
        ]
    }
    pub fn get_drill_id(&self) -> DrillId {
        match self {
            Self::Window => DrillId::Root,
            Self::TitleBar => DrillId::Child([3].into()),
            Self::Ribbon => DrillId::Child([0].into()),
            Self::TopBar => DrillId::Child([1].into()),
            Self::AddressBox => DrillId::Child([1, 0, 2, 0, 0, 0].into()),
            Self::Body => DrillId::Child([2].into()),
            Self::Main => DrillId::Child([2, 0, 0, 1].into()),
            Self::NavigationPane => DrillId::Child([2, 0, 0, 0].into()),
        }
    }
    pub fn from_window_relative_drill_id(drill_id: &DrillId) -> Option<ExplorerElementKind> {
        Self::variants()
            .into_iter()
            .find(|variant| &variant.get_drill_id() == drill_id)
    }
    pub fn get_default_text(&self) -> Option<String> {
        None
    }
    pub fn get_text_from_state(&self, state: &ExplorerState) -> Option<String> {
        match self {
            Self::AddressBox => Some(state.path.to_string_lossy().to_string()),
            _ => None,
        }
    }
    pub fn get_name(&self) -> String {
        match self {
            Self::Window => "ExplorerElementKind::Window".to_string(),
            Self::AddressBox => "ExplorerElementKind::AddressBox".to_string(),
            Self::Body => "ExplorerElementKind::Body".to_string(),
            Self::Ribbon => "ExplorerElementKind::Ribbon".to_string(),
            Self::TitleBar => "ExplorerElementKind::TitleBar".to_string(),
            Self::TopBar => "ExplorerElementKind::TopBar".to_string(),
            Self::Main => "ExplorerElementKind::Main".to_string(),
            Self::NavigationPane => "ExplorerElementKind::NavigationPane".to_string(),
        }
    }
}

pub trait ExplorerTheme {
    fn get_bounds(&self, element_kind: &ExplorerElementKind) -> Rect;
    fn get_background_color(&self, element_kind: &ExplorerElementKind) -> Color;
    fn get_text_style(
        &self,
        element_kind: &ExplorerElementKind,
        asset_server: &AssetServer,
    ) -> TextStyle;
}

#[derive(Debug, Reflect)]
pub enum ExplorerThemeKind {
    WindowsDark,
}

impl ExplorerTheme for ExplorerThemeKind {
    fn get_bounds(&self, element_kind: &ExplorerElementKind) -> Rect {
        match element_kind {
            ExplorerElementKind::Window => Rect::new(0.0, 0.0, 908.0, -782.0),
            ExplorerElementKind::TitleBar => Rect::new(24.0, -3.0, 900.0, -34.0),
            ExplorerElementKind::Ribbon => Rect::new(8.0, -31.0, 900.0, -147.0),
            ExplorerElementKind::TopBar => Rect::new(8.0, -147.0, 900.0, -182.0),
            ExplorerElementKind::Body => Rect::new(8.0, -182.0, 900.0, -774.0),
            ExplorerElementKind::NavigationPane => Rect::new(8.0, -182.0, 221.0, -751.0),
            ExplorerElementKind::Main => Rect::new(225.0, -182.0, 900.0, -751.0),

            _ => Rect::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    fn get_background_color(&self, element_kind: &ExplorerElementKind) -> Color {
        match element_kind {
            ExplorerElementKind::Window => Color::rgb(0.1, 0.1, 0.1),
            ExplorerElementKind::TitleBar => Color::rgb(0.2, 0.2, 0.2),
            ExplorerElementKind::Ribbon => Color::rgb(0.1, 0.1, 0.1),
            ExplorerElementKind::TopBar => Color::rgb(0.1, 0.1, 0.1),
            ExplorerElementKind::Body => Color::rgb(0.1, 0.1, 0.1),
            ExplorerElementKind::NavigationPane => Color::rgb(0.1, 0.1, 0.1),
            ExplorerElementKind::Main => Color::rgb(0.1, 0.1, 0.1),

            _ => Color::PINK,
        }
    }

    fn get_text_style(
        &self,
        _element_kind: &ExplorerElementKind,
        asset_server: &AssetServer,
    ) -> TextStyle {
        TextStyle {
            font: asset_server.load("fonts/tom7/FixederSys2x.ttf"),
            font_size: 16.0,
            color: Color::rgb(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Event, Debug, Reflect)]
pub struct SpawnExplorerRequestEvent {
    pub environment_id: Entity,
    pub theme: ExplorerThemeKind,
    pub state: ExplorerState,
    pub position: Vec2,
}
