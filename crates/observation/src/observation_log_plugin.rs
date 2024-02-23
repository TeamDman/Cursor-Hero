use bevy::prelude::*;
use cursor_hero_observation_types::prelude::*;

pub struct ObservationLogPlugin;

impl Plugin for ObservationLogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, log_observations);
    }
}

fn log_observations(mut observation_event_reader: EventReader<SomethingObservableHappenedEvent>) {
    for event in observation_event_reader.read() {
        info!("{:?}", event);
    }
}
