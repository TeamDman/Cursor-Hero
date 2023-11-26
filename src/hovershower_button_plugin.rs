use bevy::prelude::*;

use crate::{interaction_plugin::{Interactable, WithinInteractionRange}, update_ordering::InteractionSet};

pub struct HoverShowerButtonPlugin;
impl Plugin for HoverShowerButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_button).add_systems(
            Update,
            (reset_interaction_color, update_interaction_color)
                .chain()
                .in_set(InteractionSet::Response),
        );
    }
}

#[derive(Component, Default, Reflect)]
struct MyButton;

fn spawn_button(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 150.0, 0.0),
                scale: Vec3::new(100.0, 100.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        MyButton,
        Interactable,
        Name::new("HoverShower Button"),
    ));
}

fn reset_interaction_color(mut query: Query<&mut Sprite, With<MyButton>>) {
    for mut sprite in query.iter_mut() {
        sprite.color = Color::rgb(0.0, 0.0, 0.0);
    }
}

fn update_interaction_color(
    mut query: Query<&mut Sprite, (With<MyButton>, With<WithinInteractionRange>)>,
) {
    for mut sprite in query.iter_mut() {
        sprite.color = Color::rgb(0.0, 1.0, 0.0);
    }
}
