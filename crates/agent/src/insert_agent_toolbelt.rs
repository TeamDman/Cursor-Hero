use bevy::prelude::*;
use cursor_hero_agent_types::agent_types::Agent;
use cursor_hero_character_types::prelude::*;
use cursor_hero_toolbelt_types::toolbelt_types::*;

pub struct InsertAgentToolbeltPlugin;

impl Plugin for InsertAgentToolbeltPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, insert_agent_toolbelt);
    }
}

pub fn insert_agent_toolbelt(
    mut commands: Commands,
    mut writer: EventWriter<PopulateToolbeltEvent>,
    fresh_characters: Query<Entity, (Added<Agent>, With<Character>)>,
) {
    for character in fresh_characters.iter() {
        let character_id = character;
        commands.entity(character_id).with_children(|c_commands| {
            let toolbelt = c_commands.spawn(ToolbeltBundle::default());
            writer.send(PopulateToolbeltEvent {
                id: toolbelt.id(),
                loadout: ToolbeltLoadout::Agent,
            });
            info!(
                "Sent populate agent toolbelt event for agent {:?}",
                character_id
            );
        });
    }
}
