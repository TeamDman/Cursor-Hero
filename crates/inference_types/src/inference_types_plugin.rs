use bevy::prelude::*;
use crate::prelude::*;
pub struct InferenceTypesPlugin;

impl Plugin for InferenceTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InferenceSession>();
        app.add_event::<InferenceEvent>();
    }
}