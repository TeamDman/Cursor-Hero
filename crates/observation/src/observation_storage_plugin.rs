use bevy::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_environment_types::prelude::*;
use cursor_hero_observation_types::prelude::*;

/// Responsible for storing observations inside ObservationBuckets of those who are able to observe them.
pub struct ObservationStoragePlugin;

impl Plugin for ObservationStoragePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, track_observations);
    }
}

fn track_observations(
    mut observation_events: EventReader<ObservationEvent>,
    mut agent_query: Query<(Entity, &mut ObservationTimeline), With<AgentCharacter>>,
    environment_query: Query<(Option<&GameEnvironment>, Option<&Name>), With<Environment>>,
) {
    for event in observation_events.read() {
        // let ObservationEvent::SomethingHappened { observation } = event else {
        //     continue;
        // };
        // let Some(environment_id) = observation.environment_id;

        // for agent in agent_query.iter_mut() {
        //     let (agent_entity, mut observation_timeline) = agent;
        //     let mut agent_can_see = false;
        //     let Ok(environment) = environment_query.get(observation.environment_id) else {
        //         warn!(
        //             "Observation event for unknown environment? environment_id {:?}",
        //             observation.environment_id
        //         );
        //         continue;
        //     };
        //     let (game_environment, environment_name) = environment;
        //     if !game_environment.is_some() {
        //         continue;
        //     }
        //     let Some(name) = environment_name else {
        //         warn!(
        //             "Observation event for environment with no name? environment_id {:?}",
        //             observation.environment_id
        //         );
        //         continue;
        //     };

        //     let entry = (Instant::now(), observation.clone());
        //     debug!("Agent {:?} tracking observation: {:?}", agent_entity, entry);
        //     observation_timeline.observations.push(entry);
        // }
    }
}
