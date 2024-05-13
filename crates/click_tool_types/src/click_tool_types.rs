use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use cursor_hero_cursor_types::cursor_click_types::Way;
use cursor_hero_input::active_input_state_plugin::InputMethod;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_worker_types::prelude::WorkerMessage;
use leafwing_input_manager::prelude::*;

#[derive(Component, Reflect, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct ClickTool;

impl Default for ClickTool {
    fn default() -> Self {
        match InputMethod::default() {
            InputMethod::MouseAndKeyboard | InputMethod::Keyboard => Self::default_mnk(),
            InputMethod::Gamepad => Self::default_gamepad(),
        }
    }
}
impl ClickTool {
    pub fn default_mnk() -> ClickTool {
        ClickTool
    }
    pub fn default_gamepad() -> ClickTool {
        ClickTool
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ClickToolAction {
    LeftClick,
    MiddleClick,
    RightClick,
}
impl From<ClickToolAction> for Way {
    fn from(action: ClickToolAction) -> Self {
        match action {
            ClickToolAction::LeftClick => Way::Left,
            ClickToolAction::MiddleClick => Way::Middle,
            ClickToolAction::RightClick => Way::Right,
        }
    }
}

#[derive(Debug, Reflect, Clone, Event, Eq, PartialEq)]
pub enum ThreadboundClickMessage {
    LeftMouse(Motion, IVec2),
    RightMouse(Motion, IVec2),
}
impl WorkerMessage for ThreadboundClickMessage {}

#[derive(Debug, Reflect, Clone, Event, Eq, PartialEq)]
pub enum GameboundClickMessage {}
impl WorkerMessage for GameboundClickMessage {}

#[derive(Debug, Reflect, Clone, Eq, PartialEq)]
pub enum Motion {
    Up,
    Down,
}

impl ClickToolAction {
    pub fn get_audio_path(&self, motion: Motion) -> &'static str {
        match (self, motion) {
            (Self::LeftClick, Motion::Down) => "sounds/mouse1down.ogg",
            (Self::LeftClick, Motion::Up) => "sounds/mouse1up.ogg",
            (Self::MiddleClick, Motion::Down) => "sounds/mouse1down.ogg",
            (Self::MiddleClick, Motion::Up) => "sounds/mouse1up.ogg",
            (Self::RightClick, Motion::Down) => "sounds/mouse2down.ogg",
            (Self::RightClick, Motion::Up) => "sounds/mouse2up.ogg",
        }
    }
    pub fn get_thread_message(&self, motion: Motion, pos: IVec2) -> ThreadboundClickMessage {
        match (self, motion) {
            (Self::LeftClick, Motion::Down) => {
                ThreadboundClickMessage::LeftMouse(Motion::Down, pos)
            }
            (Self::LeftClick, Motion::Up) => ThreadboundClickMessage::LeftMouse(Motion::Up, pos),
            (Self::MiddleClick, Motion::Down) => {
                ThreadboundClickMessage::LeftMouse(Motion::Down, pos)
            }
            (Self::MiddleClick, Motion::Up) => ThreadboundClickMessage::LeftMouse(Motion::Up, pos),
            (Self::RightClick, Motion::Down) => {
                ThreadboundClickMessage::RightMouse(Motion::Down, pos)
            }
            (Self::RightClick, Motion::Up) => ThreadboundClickMessage::RightMouse(Motion::Up, pos),
        }
    }
    fn default_wheel_gamepad_binding(&self) -> UserInput {
        match self {
            Self::LeftClick => GamepadButtonType::RightTrigger.into(),
            Self::RightClick => GamepadButtonType::LeftTrigger.into(),
            // Self::MiddleClick => GamepadButtonType::DPadRight.into(),
            Self::MiddleClick => UserInput::Chord(vec![
                GamepadButtonType::LeftTrigger2.into(),
                GamepadButtonType::RightTrigger.into(),
            ]),
        }
    }

    fn default_wheel_keyboard_binding(&self) -> UserInput {
        match self {
            Self::LeftClick => MouseButton::Left.into(),
            Self::RightClick => MouseButton::Right.into(),
            Self::MiddleClick => MouseButton::Middle.into(),
        }
    }
    fn keyboard_wheel_gamepad_binding(&self) -> UserInput {
        match self {
            Self::LeftClick => GamepadButtonType::RightThumb.into(),
            Self::RightClick => GamepadButtonType::LeftThumb.into(),
            Self::MiddleClick => GamepadButtonType::Select.into(),
        }
    }

    fn keyboard_wheel_keyboard_binding(&self) -> UserInput {
        match self {
            Self::LeftClick => MouseButton::Left.into(),
            Self::RightClick => MouseButton::Right.into(),
            Self::MiddleClick => MouseButton::Middle.into(),
        }
    }
}

impl ToolAction for ClickToolAction {
    fn default_input_map(event: &ToolbeltPopulateEvent) -> Option<InputMap<ClickToolAction>> {
        match event.loadout {
            ToolbeltLoadout::Default => Some(Self::with_defaults(
                Self::default_wheel_gamepad_binding,
                Self::default_wheel_keyboard_binding,
            )),
            ToolbeltLoadout::Keyboard => Some(Self::with_defaults(
                Self::keyboard_wheel_gamepad_binding,
                Self::keyboard_wheel_keyboard_binding,
            )),
            _ => None,
        }
    }
}
