use crate::prelude::*;
use bevy::prelude::*;

pub struct UiInspectorTypesPlugin;

impl Plugin for UiInspectorTypesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIData>();
        app.register_type::<UIData>();
        app.register_type::<FetchingState>();
        app.add_event::<InspectorScratchPadEvent>();
        app.register_type::<InspectorScratchPadEvent>();
        app.add_event::<ThreadboundUISnapshotMessage>();
        app.register_type::<ThreadboundUISnapshotMessage>();
        app.add_event::<GameboundUISnapshotMessage>();
        app.register_type::<GameboundUISnapshotMessage>();
    }
}
