use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use cursor_hero_toolbelt_types::prelude::*;

#[derive(Component, InspectorOptions, Debug, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct ZoomTool {
    #[inspector(min = 0.0)]
    pub speed: f32,
    #[inspector(min = 0.0)]
    pub default_speed: f32,
    #[inspector(min = 0.0)]
    pub sprint_speed: f32,
    #[inspector(min = 0.0001, max = 10000.0)]
    pub scale_min: f32,
    #[inspector(min = 0.0001, max = 10000.0)]
    pub scale_max: f32,
}
impl Default for ZoomTool {
    fn default() -> Self {
        Self {
            speed: 1.0,
            default_speed: 1.0,
            sprint_speed: 50.0,
            scale_min: 0.001,
            scale_max: 10.0,
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ZoomToolAction {
    Out,
    In,
}

impl ZoomToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Out => GamepadButtonType::DPadLeft.into(),
            Self::In => GamepadButtonType::DPadRight.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Out => KeyCode::Home.into(),
            Self::In => KeyCode::End.into(),
        }
    }
}
impl ToolAction for ZoomToolAction {
    fn default_input_map(_event: &ToolbeltPopulateEvent) -> Option<InputMap<ZoomToolAction>> {
        let mut input_map = InputMap::default();

        for variant in ZoomToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}
