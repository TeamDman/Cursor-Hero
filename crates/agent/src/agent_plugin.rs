use bevy::prelude::*;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_environment::environment_plugin::PopulateEnvironmentEvent;
use cursor_hero_physics::damping_plugin::MovementDamping;

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Agent>();
        app.add_systems(Update, spawn_agent);
    }
}

#[derive(Component, Reflect, Debug)]
pub struct Agent;

#[derive(Component, Reflect, Eq, PartialEq, Debug)]
pub enum AgentAppearance {
    Default,
}

impl AgentAppearance {
    pub fn get_texture_path(&self) -> &'static str {
        match self {
            Self::Default => "textures/agent/default_agent.png",
        }
    }
}

fn spawn_agent(
    mut commands: Commands,
    mut environment_events: EventReader<PopulateEnvironmentEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in environment_events.read() {
        let PopulateEnvironmentEvent::Game { environment_id } = event else {
            continue;
        };
        info!("Spawning agent for game environment {:?}", environment_id);
        commands.entity(*environment_id).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(64.0, 64.0)),
                        ..default()
                    },
                    texture: asset_server.load(AgentAppearance::Default.get_texture_path()),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 80.0)),
                    ..default()
                },
                Character::default(),
                Name::new("Agent"),
                Agent,
                RigidBody::Kinematic,
                Collider::capsule(15.0, 12.5),
                MovementDamping { factor: 0.90 },
            ));
        });
    }
}
