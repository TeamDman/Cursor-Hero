use cursor_hero_toolbelt_types::types::*;

use bevy::audio::Volume;
use bevy::audio::VolumeLevel;
use bevy::prelude::*;

#[derive(Component)]
pub struct WheelAudio;

pub fn wheel_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut toolbelt_events: EventReader<ToolbeltStateEvent>,
    toolbelt_query: Query<&GlobalTransform>,
) {
    for event in toolbelt_events.read() {
        match event {
            ToolbeltStateEvent::Opened {
                toolbelt_id,
                character_id: _,
            } => {
                let Ok(toolbelt_transform) = toolbelt_query.get(*toolbelt_id) else {
                    continue;
                };
                commands.spawn((
                    AudioBundle {
                        source: asset_server.load("sounds/plastic toy snapping shut 1.ogg"),
                        settings: PlaybackSettings::DESPAWN
                            .with_spatial(true)
                            .with_volume(Volume::Relative(VolumeLevel::new(0.5))),
                    },
                    SpatialBundle {
                        transform: toolbelt_transform.compute_transform(),
                        ..default()
                    },
                    WheelAudio,
                    Name::new("opening sound"),
                ));
            }
            ToolbeltStateEvent::Closed {
                toolbelt_id,
                character_id: _,
            } => {
                let Ok(toolbelt_transform) = toolbelt_query.get(*toolbelt_id) else {
                    continue;
                };
                commands.spawn((
                    AudioBundle {
                        source: asset_server
                            .load("sounds/plastic toy snapping shut 1 reversed.ogg"),
                        settings: PlaybackSettings::DESPAWN
                            .with_spatial(true)
                            .with_volume(Volume::Relative(VolumeLevel::new(0.5))),
                    },
                    SpatialBundle {
                        transform: toolbelt_transform.compute_transform(),
                        ..default()
                    },
                    WheelAudio,
                    Name::new("closing sound"),
                ));
            }
        }
    }
}
