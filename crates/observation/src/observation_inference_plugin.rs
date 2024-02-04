use bevy::prelude::*;
use cursor_hero_inference_types::inference_types::InferenceEvent;
use cursor_hero_inference_types::inference_types::InferenceSession;
use cursor_hero_observation_types::observation_types::ObservationEvent;

pub struct ObservationInferencePlugin;

impl Plugin for ObservationInferencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, observation_inference_pipeline);
    }
}

fn observation_inference_pipeline(
    mut commands: Commands,
    mut observation_events: EventReader<ObservationEvent>,
    mut inference_events: EventWriter<InferenceEvent>,
) {
    for event in observation_events.read() {
        let ObservationEvent::ObservationToolResponse {
            observation,
            character_id: _,
        } = event;

        let session_id = commands
            .spawn((InferenceSession, Name::new("ObservationInferenceSession")))
            .id();
        let prompt = observation.clone();
        inference_events.send(InferenceEvent::Request { session_id, prompt });
        debug!(
            "ObservationInferencePlugin: Sent inference request for session {:?}",
            session_id
        );
    }
}
