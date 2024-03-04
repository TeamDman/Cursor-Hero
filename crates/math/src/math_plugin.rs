use bevy::prelude::*;

use crate::prelude::Corner;

pub struct MathPlugin;

impl Plugin for MathPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Corner>();
    }
}
