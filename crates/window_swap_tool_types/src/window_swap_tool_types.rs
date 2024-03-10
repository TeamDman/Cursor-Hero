use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use cursor_hero_toolbelt_types::prelude::*;

#[derive(Component, Reflect, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct WindowSwapTool;

impl Default for WindowSwapTool {
    fn default() -> Self {
        match ActiveInput::default() {
            ActiveInput::MouseAndKeyboard => Self::default_mnk(),
            ActiveInput::Gamepad => Self::default_gamepad(),
        }
    }
}
impl WindowSwapTool {
    pub fn default_mnk() -> WindowSwapTool {
        WindowSwapTool
    }
    pub fn default_gamepad() -> WindowSwapTool {
        WindowSwapTool
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum WindowSwapToolAction {
    Use,
}

impl WindowSwapToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Use => GamepadButtonType::North.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Use => KeyCode::Q.into(),
        }
    }
}
impl ToolAction for WindowSwapToolAction {
    fn default_input_map(_event: &ToolbeltPopulateEvent) -> Option<InputMap<WindowSwapToolAction>> {
        let mut input_map = InputMap::default();

        for variant in WindowSwapToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}
