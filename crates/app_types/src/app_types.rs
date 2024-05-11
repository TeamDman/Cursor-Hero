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
}
