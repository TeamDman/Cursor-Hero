use bevy::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_chat_types::prelude::*;
use cursor_hero_environment_types::environment_types::TrackedEnvironment;
use cursor_hero_observation_types::prelude::*;
pub struct ObserveChatPlugin;

impl Plugin for ObserveChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, observe_chat);
    }
}

fn observe_chat(
    mut chat_events: EventReader<ChatEvent>,
    mut observation_events: EventWriter<SomethingObservableHappenedEvent>,
    character_query: Query<(Option<&Name>, Option<&TrackedEnvironment>), With<Character>>,
) {
    for event in chat_events.read() {
        let ChatEvent::Chat {
            character_id,
            message,
        } = event;
        let Ok(character) = character_query.get(*character_id) else {
            warn!(
                "Chat event for unknown character? character_id {:?}",
                character_id
            );
            continue;
        };
        let (character_name, character_environment_tag) = character;

        let Some(character_name) = character_name else {
            warn!(
                "Chat event for character with no name? character_id {:?}",
                character_id
            );
            continue;
        };

        let environment_id = character_environment_tag.map(|tag| tag.environment_id);

        let event = SomethingObservableHappenedEvent::Chat {
            environment_id,
            character_id: *character_id,
            character_name: character_name.to_string(),
            message: message.clone(),
        };
        debug!("Sending event: {:?}", event);
        observation_events.send(event);
    }
}
