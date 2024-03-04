use bevy::prelude::*;
use crate::prelude::*;

pub struct UiAutomationTypesPlugin;

impl Plugin for UiAutomationTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ElementInfo>();
    }
}
