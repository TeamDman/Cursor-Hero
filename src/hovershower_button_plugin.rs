use bevy::prelude::*;

pub struct HoverShowerButtonPlugin;
impl Plugin for HoverShowerButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_button);
    }
}

#[derive(Component, Default, Reflect)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

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
        Collider,
        Name::new("HoverShower Button"),
    ));
}
