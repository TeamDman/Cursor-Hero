use crate::prelude::*;
use bevy::prelude::*;

pub struct {{crate_name_pascal}}ToolTypesPlugin;

impl Plugin for {{crate_name_pascal}}ToolTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<{{crate_name_pascal}}Tool>();
    }
}
