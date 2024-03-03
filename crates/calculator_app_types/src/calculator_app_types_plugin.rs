use crate::prelude::*;
use bevy::prelude::*;

pub struct CalculatorAppTypesPlugin;

impl Plugin for CalculatorAppTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AppWindow>();
    }
}
