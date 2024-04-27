use bevy::prelude::*;
use crate::prelude::*;

pub struct ExplorerAppTypesPlugin;

impl Plugin for ExplorerAppTypesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnExplorerRequestEvent>();
        app.register_type::<SpawnExplorerRequestEvent>();
        app.register_type::<Explorer>();
        app.register_type::<ExplorerStartMenuPanelButton>();
    }
}