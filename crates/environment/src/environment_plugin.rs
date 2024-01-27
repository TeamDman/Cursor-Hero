use bevy::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Environment>()
            .add_event::<CreateEnvironmentEvent>()
            .add_event::<PopulateEnvironmentEvent>()
            .add_systems(Startup, send_create_host_event)
            .add_systems(Update, handle_create_events);
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Environment;

#[derive(Event, Debug, Reflect)]
pub enum CreateEnvironmentEvent {
    Host { origin: Vec2, name: String },
    Game { origin: Vec2, name: String },
}

#[derive(Event, Debug, Reflect)]
pub enum PopulateEnvironmentEvent {
    Host { environment_id: Entity },
    Game { environment_id: Entity },
}

fn send_create_host_event(mut events: EventWriter<CreateEnvironmentEvent>) {
    events.send(CreateEnvironmentEvent::Host {
        origin: Vec2::new(0.0, 0.0),
        name: "Host Environment".to_string(),
    });
}

fn handle_create_events(mut commands: Commands, mut create_events: EventReader<CreateEnvironmentEvent>, mut populate_events: EventWriter<PopulateEnvironmentEvent>) {
    for event in create_events.read() {
        match event {
            CreateEnvironmentEvent::Host { origin, name } => {
                info!("Creating host environment at {:?}", origin);
                let environment_id = commands.spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(origin.extend(0.0)),
                        ..default()
                    },
                    Environment,
                    Name::new(name.clone()),
                )).id();
                info!("Broadcasting environment population event");
                populate_events.send(PopulateEnvironmentEvent::Host { environment_id })
            }
            CreateEnvironmentEvent::Game { origin, name } => {
                info!("Creating game environment at {:?}", origin);
                let environment_id = commands.spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(origin.extend(0.0)),
                        ..default()
                    },
                    Environment,
                    Name::new(name.clone()),
                )).id();
                info!("Broadcasting environment population event");
                populate_events.send(PopulateEnvironmentEvent::Game { environment_id })
            }
        }
    }
}
