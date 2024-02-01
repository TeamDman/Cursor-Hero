use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_xpbd_2d::prelude::*;

use cursor_hero_bevy::NegativeYVec2;
use cursor_hero_camera::camera_plugin::CameraEvent;
use cursor_hero_physics::damping_plugin::MovementDamping;
use cursor_hero_winutils::win_mouse::get_cursor_position;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character)
            .add_systems(Update, update_character_appearance_from_camera_events)
            .register_type::<Character>();
    }
}

#[derive(Component, InspectorOptions, Reflect, Default)]
#[reflect(Component, InspectorOptions)]
pub struct Character;

#[derive(Component)]
pub struct MainCharacter;

#[derive(Component, Reflect, Eq, PartialEq, Debug)]
pub enum CharacterAppearance {
    Focused,
    Unfocused,
}

impl CharacterAppearance {
    pub fn get_texture_path(&self) -> &'static str {
        match self {
            Self::Focused => "textures/character/default_character_focused.png",
            Self::Unfocused => "textures/character/default_character_unfocused.png",
        }
    }
}

fn spawn_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_events: EventWriter<CameraEvent>,
) {
    let os_cursor_pos = match get_cursor_position() {
        Ok(pos) => pos,
        Err(e) => {
            error!("Failed to get cursor position, spawning character at (0,0): {}", e);
            IVec2::ZERO
        }
    };
    let character = commands.spawn((
        SpriteBundle {
            texture: asset_server.load(CharacterAppearance::Focused.get_texture_path()),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(os_cursor_pos.as_vec2().neg_y().extend(100.0)),
            ..default()
        },
        Character::default(),
        MainCharacter,
        MovementDamping { factor: 0.90 },
        Name::new("Character"),
        RigidBody::Kinematic,
        Collider::capsule(15.0, 12.5),
        SpatialListener::new(-7.0),
    ));
    camera_events.send(CameraEvent::BeginFollowing {
        target_id: character.id(),
    });
    info!("Spawned character");
}

fn update_character_appearance_from_camera_events(
    mut camera_events: EventReader<CameraEvent>,
    asset_server: Res<AssetServer>,
    mut character_query: Query<&mut Handle<Image>, With<Character>>,
) {
    for event in camera_events.read() {
        match event {
            CameraEvent::BeginFollowing { target_id } => {
                if let Ok(mut texture) = character_query.get_mut(*target_id) {
                    *texture = asset_server.load(CharacterAppearance::Focused.get_texture_path());
                    info!("Updated character appearance to focused");
                }
            }
            CameraEvent::StopFollowing { target_id } => {
                if let Ok(mut texture) = character_query.get_mut(*target_id) {
                    *texture = asset_server.load(CharacterAppearance::Unfocused.get_texture_path());
                    info!("Updated character appearance to unfocused");
                }
            }
        }
    }
}
