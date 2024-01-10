use bevy::prelude::*;

use crate::active_input_state_plugin::ActiveInputStatePlugin;
use crate::update_gamepad_settings::update_gamepad_settings;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ActiveInputStatePlugin);
        app.add_systems(Update, update_gamepad_settings);
    }
}
