use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use cursor_hero_toolbelt_types::prelude::*;

#[derive(Component, Reflect, Debug, InspectorOptions, Default)]
#[reflect(Component, InspectorOptions)]
pub struct FullscreenTool;
