use bevy::prelude::*;
use crate::prelude::*;

pub struct TextAssetTypesPlugin;

impl Plugin for TextAssetTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MyComponent>();
        app.add_event::<MyEvent>();
    }
}