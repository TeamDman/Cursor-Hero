use crate::prelude::*;
use bevy::prelude::*;

pub struct GladosTtsTypesPlugin;

impl Plugin for GladosTtsTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GladosTtsStatus>();
        app.register_type::<GladosTtsStatusButton>();
        app.register_type::<GladosTtsStatusButtonVisualState>();
        app.register_type::<GladosTtsStatusEvent>();
        app.add_event::<GladosTtsStatusEvent>();

        app.register_type::<GladosTtsPingEvent>();
        app.add_event::<GladosTtsPingEvent>();

        app.register_type::<GladosTtsVscodeButton>();
        app.register_type::<GladosTtsVscodeButtonVisualState>();
        app.register_type::<GladosTtsVscodeEvent>();
        app.add_event::<GladosTtsVscodeEvent>();
    }
}
