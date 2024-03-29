use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::pointer_behaviour_types::PointerMovementBehaviour;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum PointerSystemSet {
    Position,
}

#[derive(Reflect, Debug, PartialEq, Eq, Clone)]
pub enum PointerLogBehaviour {
    Errors,
    ErrorsAndPositionUpdates,
}

#[derive(Component, Debug, Reflect)]
pub struct MainPointer;

#[derive(Component, InspectorOptions, Debug, Reflect)]
#[reflect(InspectorOptions)]
pub struct Pointer {
    #[inspector(min = 0.0)]
    pub reach: f32,
    #[inspector(min = 0.0)]
    pub default_reach: f32,
    #[inspector(min = 0.0)]
    pub sprint_reach: f32,
    pub movement_behaviour: PointerMovementBehaviour,
    pub log_behaviour: PointerLogBehaviour,
}
impl Default for Pointer {
    fn default() -> Self {
        Pointer::new_unknown_pointer()
    }
}
impl Pointer {
    pub fn new_host_pointer() -> Self {
        Self {
            reach: 50.0,
            default_reach: 50.0,
            sprint_reach: 2000.0,
            movement_behaviour: PointerMovementBehaviour::None,
            log_behaviour: PointerLogBehaviour::Errors,
            // log_behaviour: PointerLogBehaviour::ErrorsAndPositionUpdates,
        }
    }
    pub fn new_agent_pointer() -> Self {
        Self {
            reach: 50.0,
            default_reach: 50.0,
            sprint_reach: 2000.0,
            movement_behaviour: PointerMovementBehaviour::None,
            log_behaviour: PointerLogBehaviour::Errors,
            // log_behaviour: PointerLogBehaviour::ErrorsAndPositionUpdates,
        }
    }
    pub fn new_unknown_pointer() -> Self {
        Self {
            reach: 0.0,
            default_reach: 0.0,
            sprint_reach: 0.0,
            movement_behaviour: PointerMovementBehaviour::None,
            log_behaviour: PointerLogBehaviour::Errors,
            // log_behaviour: PointerLogBehaviour::ErrorsAndPositionUpdates,
        }
    }
}
