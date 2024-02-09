use bevy::prelude::*;
use cursor_hero_environment_types::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, send_create_host_event);
        app.add_systems(Startup, send_create_game_event);
        app.add_systems(Update, handle_create_events);
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

fn handle_create_events(
    mut commands: Commands,
    mut create_events: EventReader<CreateEnvironmentEvent>,
    mut populate_events: EventWriter<PopulateEnvironmentEvent>,
) {
    for event in create_events.read() {
        match event {
            CreateEnvironmentEvent::Host { origin, name } => {
                info!("Creating host environment at {:?}", origin);
                let environment_id = commands
                    .spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(origin.extend(0.0)),
                            ..default()
                        },
                        Environment,
                        HostEnvironment,
                        Name::new(name.clone()),
                    ))
                    .id();
                info!("Broadcasting environment population event");
                populate_events.send(PopulateEnvironmentEvent::Host { environment_id })
            }
            CreateEnvironmentEvent::Game { origin, name } => {
                info!("Creating game environment at {:?}", origin);
                let environment_id = commands
                    .spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(origin.extend(0.0)),
                            ..default()
                        },
                        Environment,
                        GameEnvironment,
                        Name::new(name.clone()),
                    ))
                    .id();
                info!("Broadcasting environment population event");
                populate_events.send(PopulateEnvironmentEvent::Game { environment_id })
            }
        }
    }
}
