use crate::prelude::*;
use bevy::prelude::*;
pub struct ObservationTypesPlugin;

impl Plugin for ObservationTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ObservationTool>();
        app.add_event::<ObservationEvent>();
    }
}
