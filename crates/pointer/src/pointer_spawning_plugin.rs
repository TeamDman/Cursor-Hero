use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_pointer_types::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PointerSpawningPlugin;

impl Plugin for PointerSpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, insert_pointer);
    }
}
fn insert_pointer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    character: Query<(Entity, Option<&MainCharacter>, Option<&AgentCharacter>), Added<Character>>,
) {
    for character in character.iter() {
        let (character_id, is_main_character, is_agent_character) = character;
        info!("Creating pointer for character '{:?}'", character_id);
        commands.entity(character_id).with_children(|parent| {
            parent.spawn((
                match (is_main_character.is_some(), is_agent_character.is_some()) {
                    (true, false) => (
                        Pointer::new_host_pointer(),
                        InputManagerBundle::<PointerAction> {
                            input_map: PointerAction::default_input_map(),
                            action_state: ActionState::default(),
                        }
                    ),
                    (false, true) => (
                        Pointer::new_agent_pointer(),
                        InputManagerBundle::<PointerAction> {
                            input_map: InputMap::default(),
                            action_state: ActionState::default(),
                        }
                    ),
                    (is_main,is_agent) => {
                        error!("Character '{:?}' isn't exclusively main or agent: main: {:?}, agent: {:?}", character_id, is_main, is_agent);
                        (
                            Pointer::new_unknown_pointer(),
                            InputManagerBundle::<PointerAction> {
                                input_map: InputMap::default(),
                                action_state: ActionState::default(),
                            }
                        )
                    }
                },
                Name::new("Pointer"),
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
                RigidBody::Dynamic,
                Collider::cuboid(10.0, 10.0),
                Sensor,
            ));
        });
    }
}
