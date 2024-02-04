use bevy::prelude::*;

use crate::tts_dispatch_plugin::TtsDispatchPlugin;

pub struct TtsPlugin;

impl Plugin for TtsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TtsDispatchPlugin);
    }
}
