use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use cursor_hero_toolbelt_types::prelude::*;

#[derive(Component, Reflect, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct MovementTool {
    #[inspector(min = 0.0)]
    pub speed: f32,
    #[inspector(min = 0.0)]
    pub default_speed: f32,
    #[inspector(min = 0.0)]
    pub sprint_speed: f32,
    pub target: MovementTarget,
}
impl Default for MovementTool {
    fn default() -> Self {
        match ActiveInput::default() {
            ActiveInput::MouseAndKeyboard => Self::default_mnk(),
            ActiveInput::Gamepad => Self::default_gamepad(),
        }
    }
}
impl MovementTool {
    pub fn default_mnk() -> MovementTool {
        MovementTool {
            speed: 8000.0,
            default_speed: 8000.0,
            sprint_speed: 40000.0,
            target: MovementTarget::Character,
        }
    }
    pub fn default_gamepad() -> MovementTool {
        MovementTool {
            speed: 800.0,
            default_speed: 800.0,
            sprint_speed: 80000.0,
            target: MovementTarget::Character,
        }
    }
}

#[derive(Reflect, Debug, Clone, Copy)]
pub enum MovementTarget {
    Character,
    Camera(Entity),
}

#[derive(Event, Debug, Reflect)]
pub enum MovementTargetEvent {
    SetTarget {
        tool_id: Entity,
        target: MovementTarget,
    },
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MovementToolAction {
    Move,
}

impl MovementToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Move => UserInput::Single(InputKind::DualAxis(DualAxis::left_stick())),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Move => UserInput::VirtualDPad(VirtualDPad::wasd()),
        }
    }
}
impl ToolAction for MovementToolAction {
    fn default_input_map(_event: &PopulateToolbeltEvent) -> Option<InputMap<MovementToolAction>> {
        let mut input_map = InputMap::default();

        for variant in MovementToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}
