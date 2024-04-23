use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::cursor_behaviour_types::CursorMovementBehaviour;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum CursorSystemSet {
    Position,
}

#[derive(Reflect, Debug, PartialEq, Eq, Clone)]
pub enum CursorLogBehaviour {
    Errors,
    ErrorsAndPositionUpdates,
}

#[derive(Component, Debug, Reflect)]
pub struct MainCursor;

#[derive(Component, InspectorOptions, Debug, Reflect)]
#[reflect(InspectorOptions)]
pub struct Cursor {
    #[inspector(min = 0.0)]
    pub reach: f32,
    #[inspector(min = 0.0)]
    pub default_reach: f32,
    #[inspector(min = 0.0)]
    pub sprint_reach: f32,
    pub movement_behaviour: CursorMovementBehaviour,
    pub log_behaviour: CursorLogBehaviour,
    pub desired_position: Option<Vec2>,
}
impl Default for Cursor {
    fn default() -> Self {
        Cursor::new_unknown_cursor()
    }
}
impl Cursor {
    pub fn new_host_cursor() -> Self {
        Self {
            reach: 50.0,
            default_reach: 50.0,
            sprint_reach: 2000.0,
            movement_behaviour: CursorMovementBehaviour::None,
            log_behaviour: CursorLogBehaviour::Errors,
            desired_position: None,
            // log_behaviour: CursorLogBehaviour::ErrorsAndPositionUpdates,
        }
    }
    pub fn new_agent_cursor() -> Self {
        Self {
            reach: 50.0,
            default_reach: 50.0,
            sprint_reach: 2000.0,
            movement_behaviour: CursorMovementBehaviour::None,
            log_behaviour: CursorLogBehaviour::Errors,
            desired_position: None,
            // log_behaviour: CursorLogBehaviour::ErrorsAndPositionUpdates,
        }
    }
    pub fn new_unknown_cursor() -> Self {
        Self {
            reach: 0.0,
            default_reach: 0.0,
            sprint_reach: 0.0,
            movement_behaviour: CursorMovementBehaviour::None,
            log_behaviour: CursorLogBehaviour::Errors,
            desired_position: None,
            // log_behaviour: CursorLogBehaviour::ErrorsAndPositionUpdates,
        }
    }
}
