use bevy::prelude::*;

use super::{character_plugin::Character, screen_plugin::Screen};

pub struct LevelBoundsPlugin;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum LevelBoundsSystemSet {
    Spawn,
    Enforce,
}

impl Plugin for LevelBoundsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LevelBounds>()
            .add_systems(Startup, (spawn_parent.in_set(LevelBoundsSystemSet::Spawn), apply_deferred).chain())
            .add_systems(Update, enforce.in_set(LevelBoundsSystemSet::Enforce));
    }
}

#[derive(Component, Reflect)]
pub struct LevelBoundsParent;
#[derive(Component, Reflect)]
pub struct LevelBounds;

fn spawn_parent(mut commands: Commands) {
    info!("Spawning level bounds");
    commands.spawn((
        SpatialBundle::default(),
        LevelBoundsParent,
        Name::new("Level Bounds"),
    ));
}

fn enforce(
    mut character_query: Query<&mut Transform, (With<Character>, Without<Screen>)>,
    screen_query: Query<(&Transform, &Handle<Image>), (With<Screen>, Without<Character>)>,
    images: Res<Assets<Image>>,
) {
    let threshold_distance: f32 = 100000.0;

    for mut character_transform in character_query.iter_mut() {
        let character_pos = character_transform.translation;
        let mut closest_distance = f32::MAX;
        let mut target_position = character_pos;

        for (screen_transform, image_handle) in screen_query.iter() {
            if let Some(image) = images.get(image_handle) {
                let screen_size = Vec2::new(
                    image.texture_descriptor.size.width as f32,
                    image.texture_descriptor.size.height as f32,
                );
                let screen_pos = screen_transform.translation;

                let left_edge = screen_pos.x;
                let right_edge = screen_pos.x + screen_size.x;
                let bottom_edge = screen_pos.y;
                let top_edge = screen_pos.y + screen_size.y;

                let distances = [
                    (
                        character_pos.x - left_edge,
                        Vec3::new(
                            left_edge + threshold_distance,
                            character_pos.y,
                            character_pos.z,
                        ),
                    ),
                    (
                        right_edge - character_pos.x,
                        Vec3::new(
                            right_edge - threshold_distance,
                            character_pos.y,
                            character_pos.z,
                        ),
                    ),
                    (
                        character_pos.y - bottom_edge,
                        Vec3::new(
                            character_pos.x,
                            bottom_edge + threshold_distance,
                            character_pos.z,
                        ),
                    ),
                    (
                        top_edge - character_pos.y,
                        Vec3::new(
                            character_pos.x,
                            top_edge - threshold_distance,
                            character_pos.z,
                        ),
                    ),
                ];

                for (distance, pos) in distances {
                    if distance < closest_distance && distance > threshold_distance {
                        closest_distance = distance;
                        target_position = pos;
                    }
                }
            }
        }

        if closest_distance > threshold_distance {
            character_transform.translation = target_position;
        }
    }
}
