use crate::prelude::*;
use bevy::prelude::*;

pub struct FloatyNametagTypesPlugin;

impl Plugin for FloatyNametagTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FloatyName>();
        app.register_type::<FloatyNametag>();
    }
}
