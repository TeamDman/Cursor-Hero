use bevy::prelude::*;
use cursor_hero_ui_automation_types::prelude::ElementInfo;

#[derive(Component, Debug, Reflect)]
pub struct CursorHeroApp {
    kind: CursorHeroAppKind,
}

#[derive(Debug, Reflect)]
pub enum CursorHeroAppKind {
    Calculator,
    Explorer,
    VSCode,
}

impl CursorHeroAppKind {
    pub fn from_window(window: &ElementInfo) -> Option<Self> {
        match window {
            window if window.name == "Calculator" => Some(CursorHeroAppKind::Calculator),
            window if window.class_name == "CabinetWClass" => Some(CursorHeroAppKind::Explorer),
            _ => None,
        }
    }
    pub fn element_kind_enum_name(&self) -> &str {
        match self {
            CursorHeroAppKind::Calculator => "CalculatorElementKind",
            CursorHeroAppKind::Explorer => "ExplorerElementKind",
            CursorHeroAppKind::VSCode => "VSCodeElementKind",
        }
    }
}
