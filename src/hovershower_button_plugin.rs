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
            .add_systems(
                Update,
                (
                    can_interact_update,
                    reset_interaction_color,
                    update_interaction_color,
                )
                    .chain(),
            )
            .register_type::<MyButton>()
            .register_type::<CanInteract>()
            .register_type::<HashSet<Entity>>();
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
        Collider::cuboid(150.0, 150.0), // make the interaction range larger than the button itself
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

#[derive(Component, Default, Reflect)]
struct CanInteract(pub HashSet<Entity>);

#[allow(clippy::type_complexity)]
fn can_interact_update(
    mut commands: Commands,
    mut button_query: Query<(Entity, &CollidingEntities), With<MyButton>>,
    mut started: EventReader<CollisionStarted>,
    mut ended: EventReader<CollisionEnded>,
) {
    // print out the started and ended events
    for event in started.read() {
        println!("CollisionStarted: {:?}", event);
    }
    for event in ended.read() {
        println!("CollisionEnded: {:?}", event);
    }
    for (button_entity, button_touchers) in button_query.iter_mut() {
        // println!("button_touchers: {:?}", button_touchers.0);
        if button_touchers.0.is_empty() {
            commands.entity(button_entity).remove::<CanInteract>();
        } else {
            commands
                .entity(button_entity)
                .insert(CanInteract(button_touchers.0.clone()));
        }
    }
}

fn reset_interaction_color(mut query: Query<&mut Sprite, With<MyButton>>) {
    for mut sprite in query.iter_mut() {
        sprite.color = Color::rgb(0.0, 0.0, 0.0);
    }
}

fn update_interaction_color(mut query: Query<&mut Sprite, (With<MyButton>, With<CanInteract>)>) {
    for mut sprite in query.iter_mut() {
        sprite.color = Color::rgb(0.0, 1.0, 0.0);
    }
}
