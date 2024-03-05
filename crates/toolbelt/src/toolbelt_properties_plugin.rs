use bevy::prelude::*;
use cursor_hero_toolbelt_types::toolbelt_types::ToolbeltPopulateEvent;
use cursor_hero_toolbelt_types::toolbelt_types::Toolbelt;

pub struct ToolbeltPropertiesPlugin;

impl Plugin for ToolbeltPropertiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, switch_layout);
    }
}
pub fn switch_layout(
    mut toolbelt_events: EventReader<ToolbeltPopulateEvent>,
    mut toolbelt_query: Query<&mut Toolbelt>,
) {
    for event in toolbelt_events.read() {
        let ToolbeltPopulateEvent { id, loadout } = event;
        if let Ok(mut toolbelt) = toolbelt_query.get_mut(*id) {
            toolbelt.loadout = loadout.clone();
            toolbelt.layout = loadout.layout();
        }
    }
}
