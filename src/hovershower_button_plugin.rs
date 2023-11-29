use bevy::prelude::*;
use bevy_xpbd_2d::{
    components::{Collider, ColliderParent, LinearVelocity, Position, RigidBody, Rotation, Sensor},
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
                    leave_interaction_range,
                    enter_interaction_range,
                    reset_interaction_color,
                    update_interaction_color,
                )
                    .chain(),
            )
            .register_type::<MyButton>()
            .register_type::<InInteractionRange>();
    }
}

#[derive(Component, Default, Reflect)]
struct MyButton;

#[derive(Component, Default, Reflect)]
struct InInteractionRange;

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
        Collider::cuboid(100.0, 100.0),
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

#[allow(clippy::type_complexity)]
fn enter_interaction_range(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut characters: Query<(&Character, &Name), Without<MyButton>>,
    mut buttons: Query<(&MyButton, &Name), Without<Character>>,
) {
    for CollisionStarted(a, b) in collision_event_reader.read() {
        if let Ok((character, character_name)) = characters.get(*a) {
            if let Ok((button, button_name)) = buttons.get(*b) {
                println!(
                    "{} entered interaction range of {}",
                    character_name, button_name
                );
                // mark the button as ready to interact
                commands.entity(*b).insert(InInteractionRange);
            }
        }

        if let Ok((character, character_name)) = characters.get(*b) {
            if let Ok((button, button_name)) = buttons.get(*a) {
                println!(
                    "{} entered interaction range of {}",
                    character_name, button_name
                );
                // mark the button as ready to interact
                commands.entity(*a).insert(InInteractionRange);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn leave_interaction_range(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionEnded>,
    mut characters: Query<(&Character, &Name), Without<MyButton>>,
    mut buttons: Query<(&MyButton, &Name), (Without<Character>, With<InInteractionRange>)>,
) {
    for CollisionEnded(a, b) in collision_event_reader.read() {
        if let Ok((character, character_name)) = characters.get(*a) {
            if let Ok((button, button_name)) = buttons.get(*b) {
                println!(
                    "{} left interaction range of {}",
                    character_name, button_name
                );
                // remove the ready to interact marker
                commands.entity(*b).remove::<InInteractionRange>();
            }
        }

        if let Ok((character, character_name)) = characters.get(*b) {
            if let Ok((button, button_name)) = buttons.get(*a) {
                println!(
                    "{} left interaction range of {}",
                    character_name, button_name
                );
                // remove the ready to interact marker
                commands.entity(*a).remove::<InInteractionRange>();
            }
        }
    }
}


fn reset_interaction_color(mut query: Query<&mut Sprite, With<MyButton>>) {
    for mut sprite in query.iter_mut() {
        sprite.color = Color::rgb(0.0, 0.0, 0.0);
    }
}

fn update_interaction_color(
    mut query: Query<&mut Sprite, (With<MyButton>, With<InInteractionRange>)>,
) {
    for mut sprite in query.iter_mut() {
        sprite.color = Color::rgb(0.0, 1.0, 0.0);
    }
}
