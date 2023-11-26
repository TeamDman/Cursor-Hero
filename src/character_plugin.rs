use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::interaction_plugin::Interactor;
use crate::update_ordering::MovementSet;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CharacterAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Character {
    #[inspector(min = 0.0)]
    pub speed: f32,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CharacterAction>::default())
            .add_systems(Startup, spawn_character)
            .add_systems(Update, update_character_position.in_set(MovementSet::Input))
            .register_type::<Character>();
    }
}

fn spawn_character(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut input_map = InputMap::default();
    input_map.insert(KeyCode::W, CharacterAction::MoveUp);
    input_map.insert(KeyCode::S, CharacterAction::MoveDown);
    input_map.insert(KeyCode::A, CharacterAction::MoveLeft);
    input_map.insert(KeyCode::D, CharacterAction::MoveRight);

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
        Interactor,
        Name::new("Cursor Character"),
        InputManagerBundle::<CharacterAction> {
            input_map,
            action_state: ActionState::default(),
            ..default()
        },
    ));
}

fn update_character_position(
    mut characters: Query<(&mut Transform, &Character, &ActionState<CharacterAction>)>,
    time: Res<Time>,
) {
    for (mut transform, char, action_state) in &mut characters {
        let movement_amount = char.speed * time.delta_seconds();

        if action_state.pressed(CharacterAction::MoveUp) {
            transform.translation.y += movement_amount;
        }
        if action_state.pressed(CharacterAction::MoveDown) {
            transform.translation.y -= movement_amount;
        }
        if action_state.pressed(CharacterAction::MoveRight) {
            transform.translation.x += movement_amount;
        }
        if action_state.pressed(CharacterAction::MoveLeft) {
            transform.translation.x -= movement_amount;
        }
    }
}
