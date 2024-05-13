use crate::prelude::*;
use bevy::prelude::*;

pub struct ClickToolTypesPlugin;

impl Plugin for ClickToolTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ClickTool>();
        app.register_type::<ThreadboundClickMessage>();
        app.register_type::<GameboundClickMessage>();
        app.register_type::<Motion>();
    }
}
