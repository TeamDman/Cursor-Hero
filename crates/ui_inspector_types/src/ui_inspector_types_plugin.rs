use crate::prelude::*;
use bevy::prelude::*;

pub struct UIInspectorTypesPlugin;

impl Plugin for UIInspectorTypesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIData>();
        app.register_type::<UIData>();
        app.register_type::<FetchingState>();
        app.add_event::<InspectorEvent>();
        app.register_type::<InspectorEvent>();
    }
}
