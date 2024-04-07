use crate::prelude::*;
use bevy::prelude::*;

pub struct UiWatcherTypesPlugin;

impl Plugin for UiWatcherTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ThreadboundUIWatcherMessage>();
        app.add_event::<ThreadboundUIWatcherMessage>();
        app.register_type::<GameboundUIWatcherMessage>();
        app.add_event::<GameboundUIWatcherMessage>();
    }
}
