use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use cursor_hero_input::active_input_state_plugin::InputMethod;
use cursor_hero_toolbelt_types::prelude::*;

#[derive(Component, Reflect, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct ExplorerTool;

impl Default for ExplorerTool {
    fn default() -> Self {
        match InputMethod::default() {
            InputMethod::MouseAndKeyboard | InputMethod::Keyboard => Self::default_mnk(),
            InputMethod::Gamepad => Self::default_gamepad(),
        }
    }
}
impl ExplorerTool {
    pub fn default_mnk() -> ExplorerTool {
        ExplorerTool
    }
    pub fn default_gamepad() -> ExplorerTool {
        ExplorerTool
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ExplorerToolAction {
    Use,
}

impl ExplorerToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Use => GamepadButtonType::South.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Use => MouseButton::Left.into(),
        }
    }
}
impl ToolAction for ExplorerToolAction {
    fn default_input_map(_event: &ToolbeltPopulateEvent) -> Option<InputMap<ExplorerToolAction>> {
        let mut input_map = InputMap::default();

        for variant in ExplorerToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}
