use crate::prelude::*;
use bevy::prelude::*;

pub struct CalculatorAppTypesPlugin;

impl Plugin for CalculatorAppTypesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnCalculatorRequestEvent>();
        app.register_type::<SpawnCalculatorRequestEvent>();
        app.register_type::<Calculator>();
        app.register_type::<CalculatorElementKind>();
        app.register_type::<CalculatorStartMenuPanelButton>();
        app.register_type::<CalculatorDisplay>();
        app.register_type::<CalculatorExpression>();
        app.register_type::<CalculatorButton>();
    }
}
