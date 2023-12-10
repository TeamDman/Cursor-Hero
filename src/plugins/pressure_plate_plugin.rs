use bevy::prelude::*;
use bevy_xpbd_2d::components::CollidingEntities;

pub struct PressurePlatePlugin;
impl Plugin for PressurePlatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_plate)
            .add_event::<PressurePlateActivationEvent>()
            .register_type::<PressurePlate>();
    }
}

#[derive(Event)]
pub struct PressurePlateActivationEvent(pub Entity);

#[derive(Component, Reflect)]
pub struct PressurePlate {
    active_time: f32,
    debounce: bool,
    indicator: Entity,
}

#[derive(Component, Default, Reflect)]
pub struct PressurePlateProgressIndicator {
    visual_progress: f32,
}

impl PressurePlate {
    pub fn new(indicator: Entity) -> Self {
        Self {
            active_time: 0.0,
            debounce: false,
            indicator,
        }
    }
}

fn update_plate(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut PressurePlate,
            &mut Sprite,
            &CollidingEntities,
            Option<&SpatialAudioSink>,
        ),
        Without<PressurePlateProgressIndicator>,
    >,
    mut indicator_query: Query<
        (&mut PressurePlateProgressIndicator, &mut Sprite),
        Without<PressurePlate>,
    >,
    mut activation_writer: EventWriter<PressurePlateActivationEvent>,
) {
    for (entity, mut plate, mut sprite, colliding_entities, sink) in &mut query {
        if colliding_entities.0.is_empty() {
            sprite.color = Color::rgb(0.2, 0.7, 0.9);
            plate.active_time = 0.0;
            sink.map(SpatialAudioSink::stop);
            plate.debounce = false;
        } else {
            if plate.debounce {
                continue;
            }
            sprite.color = Color::rgb(0.9, 0.7, 0.2);
            if plate.active_time == 0.0 {
                let bundle = AudioBundle {
                    source: asset_server.load("sounds/pressure plate activation.ogg"),
                    settings: PlaybackSettings::REMOVE.with_spatial(true),
                    ..default()
                };
                commands.entity(entity).insert(bundle);
                plate.active_time += time.delta_seconds();
            } else {
                plate.active_time += time.delta_seconds();
                if plate.active_time > crate::data::sounds::PRESSURE_PLATE_ACTIVATION_DURATION {
                    plate.active_time = 0.0;
                    plate.debounce = true;
                    activation_writer.send(PressurePlateActivationEvent(entity));
                }
            }
        }
        if let Ok((mut indicator, mut indicator_sprite)) = indicator_query.get_mut(plate.indicator)
        {
            indicator.visual_progress =
                plate.active_time / crate::data::sounds::PRESSURE_PLATE_ACTIVATION_DURATION;
            indicator_sprite.color = Color::rgb(0.2, 0.7, 0.9) * indicator.visual_progress;
        }
    }
}
