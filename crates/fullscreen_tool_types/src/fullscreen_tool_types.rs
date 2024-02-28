use bevy::prelude::*;
use bevy::window::WindowMode;
use cursor_hero_tools::tool_spawning::StartingState;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, Reflect, Debug, InspectorOptions, Default)]
#[reflect(Component, InspectorOptions)]
pub struct FullscreenTool;

impl FullscreenTool {
    pub fn state_for_mode(mode: WindowMode) -> StartingState {
        match mode {
            WindowMode::Windowed => StartingState::Inactive,
            _ => StartingState::Active,
        }
    }
}