use crate::prelude::*;
use bevy::prelude::*;

pub struct BrickTypesPlugin;

impl Plugin for BrickTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Brick>();
    }
}
