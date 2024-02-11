use crate::prelude::*;
use bevy::prelude::*;

pub struct GladosTtsTypesPlugin;

impl Plugin for GladosTtsTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GladosTtsStatus>();
        app.register_type::<GladosTtsStatusButton>();
        app.register_type::<GladosTtsStatusButtonVisualState>();
        app.register_type::<GladosTtsPingEvent>();
        app.add_event::<GladosTtsPingEvent>();
        app.add_event::<GladosTtsStatusEvent>();
    }
}
