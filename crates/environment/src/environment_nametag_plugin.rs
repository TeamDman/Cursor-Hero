use bevy::prelude::*;

use crate::environment_plugin::Environment;
use crate::environment_plugin::PopulateEnvironmentEvent;

pub struct EnvironmentNametagPlugin;

impl Plugin for EnvironmentNametagPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_nametags_in_new_environments)
            .register_type::<Nametag>();
    }
}

#[derive(Component, Default, Reflect)]
pub struct Nametag;

fn spawn_nametags_in_new_environments(
    mut environment_reader: EventReader<PopulateEnvironmentEvent>,
    mut commands: Commands,
    environment_query: Query<&Name, With<Environment>>,
) {
    for environment_event in environment_reader.read() {
        match environment_event {
            PopulateEnvironmentEvent::Host { environment_id }
            | PopulateEnvironmentEvent::Game { environment_id } => {
                let Ok(environment_name) = environment_query.get(*environment_id) else {
                    continue;
                };
                info!(
                    "Spawning nametags for environment {:?} ({})",
                    environment_id, environment_name
                );
                commands.entity(*environment_id).with_children(|parent| {
                    parent.spawn((
                        Text2dBundle {
                            text: Text::from_section(
                                environment_name.to_string(),
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            )
                            .with_alignment(TextAlignment::Center),
                            transform: Transform::from_xyz(0.0, 30.0, 1.0),
                            ..default()
                        },
                        Nametag,
                        Name::new("Nametag"),
                    ));
                });
            }
        }
    }
}
