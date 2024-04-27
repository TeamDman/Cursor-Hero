use bevy::prelude::*;

use crate::explorer_spawning_plugin::ExplorerSpawningPlugin;
use crate::explorer_start_menu_plugin::ExplorerStartMenuPlugin;

pub struct ExplorerAppPlugin;

impl Plugin for ExplorerAppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExplorerStartMenuPlugin);
        app.add_plugins(ExplorerSpawningPlugin);
    }
}
