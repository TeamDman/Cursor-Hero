use bevy::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_chat_types::prelude::*;
use rand::prelude::SliceRandom;
pub struct ChatSfxPlugin;

impl Plugin for ChatSfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, play_sound_for_new_chat_messages);
        app.add_systems(Update, play_sound_for_keystrokes);
    }
}
fn play_sound_for_new_chat_messages(
    mut commands: Commands,
    mut events: EventReader<ChatEvent>,
    character_query: Query<&Transform, With<Character>>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        let ChatEvent::Chat { character_id, .. } = event;
        let Ok(character) = character_query.get(*character_id) else {
            warn!("Character not found for event {:?}", event);
            continue;
        };
        let character_transform = character;
        commands.spawn((
            AudioBundle {
                source: asset_server.load("sounds/kenny_bong_001.ogg"),
                settings: PlaybackSettings::DESPAWN.with_spatial(true),
            },
            SpatialBundle {
                transform: character_transform.clone(),
                ..default()
            },
        ));
    }
}

fn play_sound_for_keystrokes(
    mut commands: Commands,
    mut events: EventReader<ChatInputEvent>,
    character_query: Query<&Transform, With<Character>>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        let ChatInputEvent::TextChanged { character_id, .. } = event else {
            continue;
        };
        let Ok(character) = character_query.get(*character_id) else {
            warn!("Character not found for event {:?}", event);
            continue;
        };
        let character_transform = character;

        // optimization opportunity: avoid unnecessary allocations
        let choices = vec![
            "sounds/kenny_click_002.ogg",
            "sounds/kenny_click_003.ogg",
        ];
        let Some(choice) = choices.choose(&mut rand::thread_rng()) else {
            continue;
        };

        commands.spawn((
            AudioBundle {
                source: asset_server.load(*choice),
                settings: PlaybackSettings::DESPAWN.with_spatial(true),
            },
            SpatialBundle {
                transform: character_transform.clone(),
                ..default()
            },
        ));
    }
}
