use crate::prelude::*;
use bevy::prelude::*;
pub struct ObservationTypesPlugin;

impl Plugin for ObservationTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ObservationTool>();
        app.register_type::<ObservationBuffer>();
        app.register_type::<ObservationBufferEntry>();
        app.register_type::<WhatsNew>();
        app.add_event::<ObservationEvent>();
        app.add_event::<ObservationBufferEvent>();
    }
}
