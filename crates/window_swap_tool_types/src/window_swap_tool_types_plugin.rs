use crate::prelude::*;
use bevy::prelude::*;

pub struct WindowSwapToolTypesPlugin;

impl Plugin for WindowSwapToolTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WindowSwapTool>();
    }
}
