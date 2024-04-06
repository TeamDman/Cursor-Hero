use bevy::prelude::*;
use crate::prelude::*;

pub struct UiHoverTypesPlugin;

impl Plugin for UiHoverTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HoverInfo>();
        app.register_type::<HostHoverIndicator>();
        app.register_type::<GameHoverIndicator>();
    }
}