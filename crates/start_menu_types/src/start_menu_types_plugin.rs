use bevy::prelude::*;
use crate::prelude::*;

pub struct StartMenuTypesPlugin;

impl Plugin for StartMenuTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StartMenuButton>();
        
        app.register_type::<StartMenu>();
        app.add_event::<StartMenuEvent>();
        
    }
}