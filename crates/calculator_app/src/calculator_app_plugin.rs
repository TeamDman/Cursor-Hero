use bevy::prelude::*;

use crate::calculator_impl_plugin::CalculatorImplPlugin;
use crate::calculator_spawning_plugin::CalculatorSpawningPlugin;
use crate::calculator_start_menu_plugin::CalculatorStartMenuPlugin;

pub struct CalculatorAppPlugin;

impl Plugin for CalculatorAppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CalculatorStartMenuPlugin);
        app.add_plugins(CalculatorSpawningPlugin);
        app.add_plugins(CalculatorImplPlugin);
    }
}
