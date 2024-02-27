use bevy::prelude::*;
use crate::prelude::*;

pub struct BrickTypesPlugin;

impl Plugin for BrickTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Brick>();
    }
}