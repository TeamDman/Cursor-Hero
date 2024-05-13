use crate::prelude::*;
use bevy::prelude::*;

pub struct ZoomToolTypesPlugin;

impl Plugin for ZoomToolTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ZoomTool>();
    }
}
