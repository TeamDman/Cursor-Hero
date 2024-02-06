use crate::prelude::*;
use bevy::prelude::*;
pub struct TtsTypesPlugin;

impl Plugin for TtsTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TtsSession>();
        app.add_event::<TtsEvent>();
    }
}
