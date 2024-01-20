use bevy::prelude::*;

use crate::click_tool::ClickToolPlugin;
use crate::cube_tool::CubeToolPlugin;
use crate::cursor_tool::CursorToolPlugin;
use crate::focus_tool::FocusToolPlugin;
use crate::inspect_tool::InspectToolPlugin;
use crate::placeholder_tool::PlaceholderToolPlugin;
#[cfg(debug_assertions)]
use crate::restart_tool::RestartToolPlugin;
use crate::sprint_tool::SprintToolPlugin;
use crate::talk_tool::TalkToolPlugin;
use crate::zoom_tool::ZoomToolPlugin;

pub struct ToolPlugin;
impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CubeToolPlugin);
        app.add_plugins(PlaceholderToolPlugin);
        app.add_plugins(ClickToolPlugin);
        app.add_plugins(CursorToolPlugin);
        app.add_plugins(TalkToolPlugin);
        app.add_plugins(ZoomToolPlugin);
        app.add_plugins(FocusToolPlugin);
        #[cfg(debug_assertions)]
        app.add_plugins(RestartToolPlugin);
        app.add_plugins(InspectToolPlugin);
        app.add_plugins(SprintToolPlugin);
    }
}
