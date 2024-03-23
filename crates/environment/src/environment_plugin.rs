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

fn send_create_host_event(mut events: EventWriter<CreateEnvironmentRequestEvent>) {
    events.send(CreateEnvironmentRequestEvent {
        kind: EnvironmentKind::Host,
        origin: Vec2::new(0.0, 0.0),
        name: "Host Environment".to_string(),
    });
}

fn send_create_game_event(mut events: EventWriter<CreateEnvironmentRequestEvent>) {
    events.send(CreateEnvironmentRequestEvent {
        kind: EnvironmentKind::Game,
        origin: Vec2::new(0.0, -3000.0),
        name: "Game Environment".to_string(),
    });
}

#[allow(clippy::type_complexity)]
fn send_populate_events(
    environment_query: Query<Entity, Added<EnvironmentKind>>,
    mut populate_events: EventWriter<PopulateEnvironmentEvent>,
) {
    for environment in environment_query.iter() {
        let environment_id = environment;
        let event = PopulateEnvironmentEvent { environment_id };
        debug!("Sending populate event: {:?}", event);
        populate_events.send(event);
    }
}

fn handle_create_events(
    mut commands: Commands,
    mut create_events: EventReader<CreateEnvironmentRequestEvent>,
) {
    for event in create_events.read() {
        info!("Creating host environment at {:?}", event.origin);
        let mut c = commands.spawn((
            SpatialBundle {
                transform: Transform::from_translation(event.origin.extend(0.0)),
                ..default()
            },
            event.kind,
            Name::new(event.name.clone()),
        ));
        match event.kind {
            EnvironmentKind::Host => {
                c.insert(HostEnvironment);
            }
            EnvironmentKind::Game => {
                c.insert(GameEnvironment);
            }
            EnvironmentKind::HostUIWatcher => {
                c.insert(HostUIWatcherEnvironment);
            }
        }
    }
}
