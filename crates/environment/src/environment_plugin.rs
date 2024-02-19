use bevy::prelude::*;
use cursor_hero_environment_types::prelude::*;

use crate::environment_tracker_plugin::EnvironmentTrackerPlugin;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnvironmentTrackerPlugin);
        app.add_systems(Startup, send_create_host_event);
        app.add_systems(Startup, send_create_game_event);
        app.add_systems(Update, handle_create_events);
        app.add_systems(Update, send_populate_events);
    }
}

fn send_create_host_event(mut events: EventWriter<CreateEnvironmentEvent>) {
    events.send(CreateEnvironmentEvent::Host {
        origin: Vec2::new(0.0, 0.0),
        name: "Host Environment".to_string(),
    });
}

fn send_create_game_event(mut events: EventWriter<CreateEnvironmentEvent>) {
    events.send(CreateEnvironmentEvent::Game {
        origin: Vec2::new(0.0, -3000.0),
        name: "Game Environment".to_string(),
    });
}

fn send_populate_events(
    environment_query: Query<
        (Entity, Option<&GameEnvironment>, Option<&HostEnvironment>),
        Added<Environment>,
    >,
    mut populate_events: EventWriter<PopulateEnvironmentEvent>,
) {
    for environment in environment_query.iter() {
        let (environment_id, is_game, is_host) = environment;
        if is_game.is_some() {
            let event = PopulateEnvironmentEvent::Game { environment_id };
            debug!("Sending populate event: {:?}", event);
            populate_events.send(event);
        }
        if is_host.is_some() {
            let event = PopulateEnvironmentEvent::Host { environment_id };
            debug!("Sending populate event: {:?}", event);
            populate_events.send(event);
        }
    }
}

fn handle_create_events(
    mut commands: Commands,
    mut create_events: EventReader<CreateEnvironmentEvent>,
    mut populate_events: EventWriter<PopulateEnvironmentEvent>,
) {
    for event in create_events.read() {
        match event {
            CreateEnvironmentEvent::Host { origin, name } => {
                info!("Creating host environment at {:?}", origin);
                commands.spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(origin.extend(0.0)),
                        ..default()
                    },
                    Environment,
                    HostEnvironment,
                    Name::new(name.clone()),
                ));
            }
            CreateEnvironmentEvent::Game { origin, name } => {
                info!("Creating game environment at {:?}", origin);
                commands.spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(origin.extend(0.0)),
                        ..default()
                    },
                    Environment,
                    GameEnvironment,
                    Name::new(name.clone()),
                ));
            }
        }
    }
}
