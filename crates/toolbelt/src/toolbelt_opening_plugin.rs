use bevy::prelude::*;
use cursor_hero_toolbelt_types::toolbelt_types::*;
use leafwing_input_manager::action_state::ActionState;

pub struct ToolbeltOpeningPlugin;

impl Plugin for ToolbeltOpeningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, opening);
    }
}

#[allow(clippy::type_complexity)]
pub fn opening(
    mut toolbelt_query: Query<(Entity, &mut Toolbelt, &ActionState<ToolbeltAction>), Without<Tool>>,
    mut toolbelt_events: EventWriter<ToolbeltOpeningEvent>,
) {
    for toolbelt in toolbelt_query.iter_mut() {
        let (toolbelt_id, mut toolbelt, toolbelt_actions) = toolbelt;
        match (
            toolbelt.open,
            toolbelt_actions.pressed(ToolbeltAction::Show),
        ) {
            (false, true) => {
                // Not open but we are holding the open button
                toolbelt_events.send(ToolbeltOpeningEvent::Opened { toolbelt_id });
                toolbelt.open = true;
            }
            (true, false) => {
                // Open but we are not holding the open button
                toolbelt_events.send(ToolbeltOpeningEvent::Closed { toolbelt_id });
                toolbelt.open = false;
            }
            _ => {}
        }
    }
}
