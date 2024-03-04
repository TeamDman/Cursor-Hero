use crate::toolbelt_types::*;
use bevy::prelude::*;

pub struct ToolbeltTypesPlugin;

impl Plugin for ToolbeltTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Toolbelt>();
        app.register_type::<Wheel>();
        app.register_type::<Tool>();
        app.register_type::<ActiveTool>();
        app.add_event::<PopulateToolbeltEvent>();
        app.add_event::<ToolbeltOpeningEvent>();
        app.add_event::<ToolActivationEvent>();
    }
}
