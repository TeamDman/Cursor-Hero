use bevy::prelude::*;

use bevy_xpbd_2d::prelude::*;

use cursor_hero_bevy::NegativeYVec2;
use cursor_hero_camera::camera_plugin::CameraEvent;
use cursor_hero_character_types::prelude::*;
use cursor_hero_physics::damping_plugin::MovementDamping;
use cursor_hero_toolbelt_types::types::PopulateToolbeltEvent;
use cursor_hero_toolbelt_types::types::ToolbeltBundle;
use cursor_hero_winutils::win_mouse::get_cursor_position;
use cursor_hero_environment_types::prelude::*;
pub struct CharacterSpawningPlugin;

impl Plugin for CharacterSpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character);
    }
}

fn spawn_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_events: EventWriter<CameraEvent>,
    mut writer: EventWriter<PopulateToolbeltEvent>,
) {
    let os_cursor_pos = match get_cursor_position() {
        Ok(pos) => pos,
        Err(e) => {
            error!(
                "Failed to get cursor position, spawning character at (0,0): {}",
                e
            );
            IVec2::ZERO
        }
    };
    let mut character = commands.spawn((
        SpriteBundle {
            texture: asset_server.load(CharacterAppearance::Focused.get_texture_path()),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(os_cursor_pos.as_vec2().neg_y().extend(100.0)),
            ..default()
        },
        Character,
        MainCharacter,
        MovementDamping { factor: 0.90 },
        Name::new("John Doe"),
        RigidBody::Kinematic,
        Collider::capsule(15.0, 12.5),
        SpatialListener::new(-7.0),
        TrackEnvironmentTag,
    ));
    camera_events.send(CameraEvent::BeginFollowing {
        target_id: character.id(),
    });
    let character_id = character.id();
    character.with_children(|c_commands| {
        let toolbelt = c_commands.spawn(ToolbeltBundle::default());
        writer.send(PopulateToolbeltEvent::Default {
            toolbelt_id: toolbelt.id(),
        });
        info!(
            "Sent populate default toolbelt event to fresh main character {:?}",
            character_id
        );
    });

    info!("Spawned character");
}
