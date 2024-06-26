use bevy::prelude::*;

use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use cursor_hero_agent_types::prelude::*;
use cursor_hero_bevy::prelude::NegativeYVec2;
use cursor_hero_character_types::prelude::*;
use cursor_hero_environment_types::prelude::*;
use cursor_hero_floaty_nametag_types::prelude::*;
use cursor_hero_observation_types::prelude::*;
use cursor_hero_physics::damping_plugin::MovementDamping;

pub struct AgentSpawningPlugin;

impl Plugin for AgentSpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_agent);
    }
}

fn spawn_agent(
    mut commands: Commands,
    mut environment_events: EventReader<PopulateEnvironmentEvent>,
    environment_query: Query<&AgentEnvironment>,
    asset_server: Res<AssetServer>,
) {
    for event in environment_events.read() {
        if !environment_query.contains(event.environment_id) {
            continue;
        }
        info!("Spawning agent for environment {:?}", event.environment_id);
        commands
            .entity(event.environment_id)
            .with_children(|parent| {
                let spawn_position = Vec2::new(1920.0, 1080.0).neg_y() / 2.0;
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(64.0, 64.0)),
                            ..default()
                        },
                        texture: asset_server.load(AgentAppearance::Default.get_texture_path()),
                        transform: Transform::from_translation(spawn_position.extend(80.0)),
                        ..default()
                    },
                    Character,
                    AgentCharacter,
                    TrackedEnvironment {
                        environment_id: event.environment_id,
                    },
                    Name::new("Character - (Agent) Ithia Tig"),
                    FloatyName {
                        text: "Ithia Tig".to_string(),
                        vertical_offset: 40.0,
                        appearance: NametagAppearance::Character,
                    },
                    Agent,
                    RigidBody::Dynamic,
                    ObservationBuffer {
                        log_level: ObservationLogLevel::All,
                        ..default()
                    },
                    Collider::capsule(25.0, 12.5),
                    MovementDamping { factor: 0.90 },
                ));
            });
    }
}
