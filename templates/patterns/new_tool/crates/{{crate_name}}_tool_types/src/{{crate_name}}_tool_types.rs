use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use cursor_hero_input::active_input_state_plugin::InputMethod;
use cursor_hero_toolbelt_types::prelude::*;

#[derive(Component, Reflect, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct {{crate_name_pascal}}Tool;

impl Default for {{crate_name_pascal}}Tool {
    fn default() -> Self {
        match InputMethod::default() {
            InputMethod::MouseAndKeyboard => Self::default_mnk(),
            InputMethod::Gamepad => Self::default_gamepad(),
        }
    }
}
impl {{crate_name_pascal}}Tool {
    pub fn default_mnk() -> {{crate_name_pascal}}Tool {
        {{crate_name_pascal}}Tool
    }
    pub fn default_gamepad() -> {{crate_name_pascal}}Tool {
        {{crate_name_pascal}}Tool
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum {{crate_name_pascal}}ToolAction {
    Use,
}

impl {{crate_name_pascal}}ToolAction {
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
impl ToolAction for {{crate_name_pascal}}ToolAction {
    fn default_input_map(_event: &PopulateToolbeltEvent) -> Option<InputMap<{{crate_name_pascal}}ToolAction>> {
        let mut input_map = InputMap::default();

        for variant in {{crate_name_pascal}}ToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}
