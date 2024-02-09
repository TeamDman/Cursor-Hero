use bevy::prelude::*;
use crate::prelude::*;

pub struct {{crate_name_pascal}}TypesPlugin;

impl Plugin for {{crate_name_pascal}}TypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MyComponent>();
        app.add_event::<MyEvent>();
    }
}