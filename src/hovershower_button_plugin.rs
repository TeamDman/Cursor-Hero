use bevy::{prelude::*, utils::HashSet};
use bevy_xpbd_2d::{
    components::{
        Collider, ColliderParent, CollidingEntities, LinearVelocity, Position, RigidBody, Rotation,
        Sensor,
    },
    plugins::collision::{
        contact_reporting::{Collision, CollisionEnded, CollisionStarted},
        Collisions,
    },
};

use crate::character_plugin::Character;

pub struct HoverShowerButtonPlugin;
impl Plugin for HoverShowerButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_button)
            .add_systems(Update, update_colour)
            .register_type::<MyButton>();
    }
}

#[derive(Component, Default, Reflect)]
struct MyButton;

fn spawn_button(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 150.0, 0.0),
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            ..default()
        },
        MyButton,
        Sensor,
        RigidBody::Static,
        Collider::cuboid(100.0, 100.0), // make the interaction range larger than the button itself
        Name::new("HoverShower Button"),
    ));

    // A cube to move around
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.4, 0.7),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_xyz(250.0, -100.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(30.0, 30.0),
        Name::new("A cube to push around"),
    ));
}

fn update_colour(mut query: Query<(&mut Sprite, &CollidingEntities), With<MyButton>>) {
    for (mut sprite, colliding_entities) in &mut query {
        if colliding_entities.0.is_empty() {
            sprite.color = Color::rgb(0.2, 0.7, 0.9);
        } else {
            sprite.color = Color::rgb(0.9, 0.7, 0.2);
        }
    }
}
