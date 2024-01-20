use super::types::*;
use bevy::audio::Volume;
use bevy::audio::VolumeLevel;
use bevy::prelude::*;

#[derive(Component)]
pub struct WheelAudio;

pub fn wheel_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut toolbelt_events: EventReader<ToolbeltEvent>,
) {
    for event in toolbelt_events.iter() {
        match event {
            ToolbeltEvent::Opened { toolbelt_id } => {
                commands.entity(*toolbelt_id).with_children(|parent| {
                    parent.spawn((
                        AudioBundle {
                            source: asset_server.load("sounds/plastic toy snapping shut 1.ogg"),
                            settings: PlaybackSettings::ONCE
                                .with_spatial(true)
                                .with_volume(Volume::Relative(VolumeLevel::new(0.5))),
                        },
                        SpatialBundle::default(),
                        WheelAudio,
                        Name::new("opening sound"),
                    ));
                });
            }
            ToolbeltEvent::Closed { toolbelt_id } => {
                commands.entity(*toolbelt_id).with_children(|parent| {
                    parent.spawn((
                        AudioBundle {
                            source: asset_server
                                .load("sounds/plastic toy snapping shut 1 reversed.ogg"),
                            settings: PlaybackSettings::ONCE
                                .with_spatial(true)
                                .with_volume(Volume::Relative(VolumeLevel::new(0.5))),
                        },
                        SpatialBundle::default(),
                        WheelAudio,
                        Name::new("closing sound"),
                    ));
                });
            }
            _ => {}
        }
    }
}

pub fn wheel_audio_cleanup(
    mut commands: Commands,
    wheel_audio_query: Query<(Entity, &SpatialAudioSink, &Parent), With<WheelAudio>>,
    parent_query: Query<Entity, With<Toolbelt>>,
) {
    for (entity, sink, parent) in wheel_audio_query.iter() {
        if sink.empty() {
            if let Ok(parent) = parent_query.get(parent.get()) {
                commands.entity(parent).remove_children(&[entity]);
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}
