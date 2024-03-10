use bevy::prelude::*;

use crate::agent_observation_memory_plugin::AgentObservationMemoryPlugin;
use crate::main_camera_memory_plugin::MainCameraMemoryPlugin;
use crate::main_character_memory_plugin::MainCharacterMemoryPlugin;
use crate::primary_window_memory_plugin::PrimaryWindowMemoryPlugin;
use crate::voice_to_text_memory_plugin::VoiceToTextMemoryPlugin;

pub struct MemoryPlugin;

impl Plugin for MemoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MainCharacterMemoryPlugin,
            PrimaryWindowMemoryPlugin,
            MainCameraMemoryPlugin,
            VoiceToTextMemoryPlugin,
            AgentObservationMemoryPlugin,
        ));
    }
}
