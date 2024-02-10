use bevy::prelude::*;
use bevy::utils::Instant;
use cursor_hero_environment_types::prelude::*;
use cursor_hero_observation_types::prelude::*;

/// Responsible for storing observations inside ObservationBuckets of those who are able to observe them.
pub struct ObservationBufferPlugin;

impl Plugin for ObservationBufferPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_buffers);
    }
}

fn update_buffers(
    mut observation_events: EventReader<ObservationEvent>,
    mut buffer_query: Query<(Entity, &mut ObservationBuffer, Option<&EnvironmentTag>)>,
    mut buffer_events: EventWriter<ObservationBufferEvent>,
) {
    for event in observation_events.read() {
        for buffer in buffer_query.iter_mut() {
            let (buffer_id, mut buffer, buffer_environment_tag) = buffer;

            // Extract environment id from the event
            let event_environment_id = match event {
                ObservationEvent::Chat { environment_id, .. } => *environment_id,
            };

            // Assert the buffer can see the event
            let can_see = match (buffer_environment_tag, event_environment_id) {
                // A buffer in an environment can only see events from that environment
                (
                    Some(EnvironmentTag {
                        environment_id: buffer_environment_id,
                    }),
                    Some(event_environment_id),
                ) => *buffer_environment_id == event_environment_id,
                // A buffer outside all environments will observe all environments
                (None, _) => true,
                _ => false,
            };
            if !can_see {
                if buffer.log_level == ObservationLogLevel::All {
                    debug!("Buffer {:?} cannot see event {:?}", buffer_id, event)
                }
                continue;
            }

            let entry = ObservationBufferEntry {
                timestamp: Instant::now(),
                observation: event.to_string(),
                origin: event.clone(),
            };
            if buffer.log_level == ObservationLogLevel::All {
                debug!("Buffer {:?} observed event {:?}", buffer_id, &entry)
            }
            buffer.observations.push(entry);

            let event = ObservationBufferEvent::Updated { buffer_id };
            buffer_events.send(event);
        }
    }
}
