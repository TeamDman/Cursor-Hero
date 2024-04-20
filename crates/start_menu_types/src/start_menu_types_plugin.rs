use crate::prelude::*;
use bevy::prelude::*;

pub struct StartMenuTypesPlugin;

impl Plugin for StartMenuTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StartMenuButton>();
        app.register_type::<StartMenuPanel>();

        app.add_event::<StartMenuPanelVisibilityChangeRequestEvent>();
        app.register_type::<StartMenuPanelVisibilityChangeRequestEvent>();
        
        app.add_event::<StartMenuPopulateEvent>();
        app.register_type::<StartMenuPopulateEvent>();
    }
}
