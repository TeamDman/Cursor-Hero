use crate::prelude::*;
use bevy::prelude::*;

pub struct MovementToolTypesPlugin;

impl Plugin for MovementToolTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MovementTool>();
        app.add_event::<MovementTargetEvent>();
    }
}
