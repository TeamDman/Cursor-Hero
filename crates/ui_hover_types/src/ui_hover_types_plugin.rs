use bevy::prelude::*;
use crate::prelude::*;

pub struct UiHoverTypesPlugin;

impl Plugin for UiHoverTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MyComponent>();
        app.add_event::<MyEvent>();
    }
}