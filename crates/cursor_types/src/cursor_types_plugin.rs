use bevy::prelude::*;

use crate::prelude::*;

pub struct CursorTypesPlugin;
impl Plugin for CursorTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Cursor>();
        app.register_type::<MainCursor>();
        app.configure_sets(Update, CursorSystemSet::Position);

        app.add_event::<CursorReachEvent>();

        app.register_type::<Hovered>();
        app.register_type::<Hoverable>();
        app.register_type::<Hovering>();
        app.add_event::<HoverEvent>();

        app.register_type::<Clickable>();
        app.register_type::<Pressed>();
        app.register_type::<Pressing>();
        app.add_event::<ClickEvent>();
        app.add_event::<ToolClickEvent>();

        app.register_type::<CursorMirror>();
        app.register_type::<HostCursorPosition>();
    }
}
