use crate::prelude::*;
use bevy::prelude::*;

pub struct UiAutomationTypesPlugin;

impl Plugin for UiAutomationTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ElementInfo>();
        app.register_type::<DrillId>();
        app.register_type::<RuntimeId>();
        app.register_type::<ControlType>();
        app.register_type::<UiSnapshot>();
        app.register_type::<AppSnapshot>();
        app.register_type::<CalculatorSnapshot>();
        app.register_type::<VSCodeSnapshot>();
        app.register_type::<Taskbar>();
        app.register_type::<TaskbarEntry>();
    }
}
