use crate::prelude::*;
use bevy::prelude::*;
pub struct InferenceTypesPlugin;

impl Plugin for InferenceTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InferenceSession>();
        app.register_type::<Prompt>();
        app.add_event::<InferenceEvent>();
    }
}
