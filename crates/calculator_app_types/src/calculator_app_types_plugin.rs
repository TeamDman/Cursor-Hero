use bevy::prelude::*;
use crate::prelude::*;

pub struct CalculatorAppTypesPlugin;

impl Plugin for CalculatorAppTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AppWindow>();
    }
}