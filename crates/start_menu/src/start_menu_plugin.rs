use crate::{start_menu_button_plugin::StartMenuButtonPlugin, start_menu_panel_plugin::StartMenuPanelPlugin};
use bevy::prelude::*;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StartMenuButtonPlugin);
        app.add_plugins(StartMenuPanelPlugin);
    }
}