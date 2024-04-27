use crate::prelude::*;
use bevy::prelude::*;

pub struct ExplorerToolTypesPlugin;

impl Plugin for ExplorerToolTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ExplorerTool>();
    }
}
