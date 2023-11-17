use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

pub struct CursorCharacterPlugin;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Character {
    #[inspector(min = 0.0)]
    pub speed: f32,
}


impl Plugin for CursorCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cursor_character)
            .add_systems(Update, cursor_movement_tick)
            .register_type::<Character>();
    }
}

fn spawn_cursor_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Character { speed: 400.0 },
        Name::new("Cursor Character")
    ));
}


fn cursor_movement_tick(
    mut characters: Query<(&mut Transform, &Character)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, char) in &mut characters {
        let movement_amount = char.speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount;
        }
    }
}
