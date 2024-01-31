use bevy::prelude::*;
use bevy::sprite::Anchor;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_pointer_types::prelude::*;
use bevy_xpbd_2d::prelude::*;
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
    character: Query<Entity, Added<Character>>,
) {
    for character_id in character.iter() {
        info!("Creating pointer for character '{:?}'", character_id);
        commands.entity(character_id).with_children(|parent| {
            parent.spawn((
                Pointer::default(),
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
                InputManagerBundle::<PointerAction> {
                    input_map: PointerAction::default_input_map(),
                    action_state: ActionState::default(),
                },
                RigidBody::Dynamic,
                Collider::cuboid(10.0, 10.0),
                Sensor,
            ));
        });
    }
}
