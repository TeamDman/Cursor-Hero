use crate::prelude::*;
use bevy::prelude::*;

pub struct FullscreenToolTypesPlugin;

impl Plugin for FullscreenToolTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FullscreenTool>();
    }
}
