use bevy::prelude::*;
use cursor_hero_toolbelt_types::toolbelt_types::PopulateToolbeltEvent;
use cursor_hero_toolbelt_types::toolbelt_types::Toolbelt;

pub struct ToolbeltLayoutPlugin;

impl Plugin for ToolbeltLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, switch_layout);
    }
}
pub fn switch_layout(
    mut toolbelt_events: EventReader<PopulateToolbeltEvent>,
    mut toolbelt_query: Query<&mut Toolbelt>,
) {
    for event in toolbelt_events.read() {
        let PopulateToolbeltEvent { id, loadout } = event;
        if let Ok(mut toolbelt) = toolbelt_query.get_mut(*id) {
            toolbelt.layout = loadout.layout();
        }
    }
}
