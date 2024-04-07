use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_cursor_types::prelude::*;
use cursor_hero_environment_types::environment_types::TrackEnvironmentTag;
use leafwing_input_manager::prelude::*;

pub struct CursorSpawningPlugin;

impl Plugin for CursorSpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, insert_cursor);
    }
}

#[allow(clippy::type_complexity)]
fn insert_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    character: Query<(Entity, Option<&MainCharacter>, Option<&AgentCharacter>), Added<Character>>,
) {
    for character in character.iter() {
        let (character_id, is_main_character, is_agent_character) = character;
        info!("Creating cursor for character '{:?}'", character_id);
        commands.entity(character_id).with_children(|parent| {
            let mut p = parent.spawn((
                Name::new("Cursor"),
                SpriteBundle {
                    texture: asset_server.load("textures/cursor.png"),
                    transform: Transform::from_xyz(0.0, 0.0, 2.0),
                    sprite: Sprite {
                        color: Color::rgb(0.149, 0.549, 0.184),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    ..Default::default()
                },
                TrackEnvironmentTag,
                RigidBody::Dynamic,
                Collider::cuboid(10.0, 10.0),
                Sensor,
            ));
            match (is_main_character.is_some(), is_agent_character.is_some()) {
                (true, false) => {
                    p.insert((
                        MainCursor,
                        Cursor::new_host_cursor(),
                        InputManagerBundle::<CursorAction> {
                            input_map: CursorAction::default_input_map(),
                            action_state: ActionState::default(),
                        },
                    ));
                }
                (false, true) => {
                    p.insert((
                        Cursor::new_agent_cursor(),
                        InputManagerBundle::<CursorAction> {
                            input_map: InputMap::default(),
                            action_state: ActionState::default(),
                        },
                    ));
                }
                (is_main, is_agent) => {
                    error!(
                        "Character '{:?}' isn't exclusively main or agent: main: {:?}, agent: {:?}",
                        character_id, is_main, is_agent
                    );
                    p.insert((
                        Cursor::new_unknown_cursor(),
                        InputManagerBundle::<CursorAction> {
                            input_map: InputMap::default(),
                            action_state: ActionState::default(),
                        },
                    ));
                }
            }
        });
    }
}
