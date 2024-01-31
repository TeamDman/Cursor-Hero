use bevy::prelude::*;

use crate::prelude::*;

pub struct PointerTypesPlugin;
impl Plugin for PointerTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pointer>();
        app.configure_sets(Update, PointerSystemSet::Position);

        app.add_event::<PointerReachEvent>();
        app.register_type::<PointerEnvironment>();
        
        app.register_type::<Hovered>();
        app.register_type::<Hoverable>();
        app.register_type::<Hovering>();
        app.add_event::<HoverEvent>();
        
        app.register_type::<Clickable>();
        app.register_type::<Pressed>();
        app.register_type::<Pressing>();
        app.add_event::<ClickEvent>();
        app.add_event::<ToolClickEvent>();
    }
}
