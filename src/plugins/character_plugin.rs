use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_xpbd_2d::{math::*, prelude::*};
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

use crate::plugins::damping_plugin::MovementDamping;

use super::damping_plugin::DampingSystemSet;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum CharacterSystemSet {
    Spawn,
    Position,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(
                Startup,
                (
                    spawn_character.in_set(CharacterSystemSet::Spawn),
                    apply_deferred,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    // player_mouse_look.run_if(in_state(ActiveInput::MouseKeyboard)),
                    apply_movement
                        .in_set(CharacterSystemSet::Position)
                        // .after(player_mouse_look)
                        .run_if(has_movement)
                        .run_if(is_character_physics_ready)
                        .after(DampingSystemSet::Dampen),
                ),
            )
            .register_type::<Character>();
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Look,
    Click,
}

// Exhaustively match `PlayerAction` and define the default binding to the input
impl PlayerAction {
    fn default_gamepad_binding(&self) -> UserInput {
        // Match against the provided action to get the correct default gamepad input
        match self {
            Self::Move => UserInput::Single(InputKind::DualAxis(DualAxis::left_stick())),
            Self::Look => UserInput::Single(InputKind::DualAxis(DualAxis::right_stick())),
            Self::Click => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::RightTrigger))
            }
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        // Match against the provided action to get the correct default gamepad input
        match self {
            Self::Move => UserInput::VirtualDPad(VirtualDPad::wasd()),
            Self::Look => UserInput::VirtualDPad(VirtualDPad::arrow_keys()),
            Self::Click => UserInput::Single(InputKind::Mouse(MouseButton::Left)),
        }
    }

    fn default_input_map() -> InputMap<PlayerAction> {
        let mut input_map = InputMap::default();

        for variant in PlayerAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Character {
    #[inspector(min = 0.0)]
    pub speed: f32,
}

fn spawn_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    commands.spawn((
        // SpriteBundle {
        //     sprite: Sprite {
        //         custom_size: Some(Vec2::new(100.0, 100.0)),
        //         ..default()
        //     },
        //     texture: asset_server.load("textures/cursor.png"),
        //     ..default()
        // },
        MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Capsule {
                        radius: 12.5,
                        depth: 20.0,
                        ..default()
                    }
                    .into(),
                )
                .into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.2, 0.7, 0.9))),
            transform: Transform::from_xyz(0.0, -100.0, 100.0),
            ..default()
        },
        Character { speed: 5000.0 },
        MovementDamping::default(),
        Name::new("Character"),
        InputManagerBundle::<PlayerAction> {
            input_map: PlayerAction::default_input_map(),
            action_state: ActionState::default(),
        },
        RigidBody::Kinematic,
        Collider::capsule(20.0, 12.5),
        SpatialListener::new(7.0),
    ));
    info!("Character spawn command issued");
}

fn is_character_physics_ready(query: Query<&LinearVelocity, With<Character>>) -> bool {
    query.get_single().is_ok()
}
fn has_movement(action_state: Query<&ActionState<PlayerAction>, With<Character>>) -> bool {
    action_state.single().pressed(PlayerAction::Move)
        || action_state.single().pressed(PlayerAction::Look)
}
fn apply_movement(
    time: Res<Time>,
    action_state: Query<&ActionState<PlayerAction>, With<Character>>,
    mut character_query: Query<(&mut LinearVelocity, &Character)>,
) {
    let (mut player_velocity, character) = character_query.single_mut();
    let delta_time = time.delta_seconds_f64().adjust_precision();

    if action_state.single().pressed(PlayerAction::Move) {
        // Note: In a real game we'd feed this into an actual player controller
        // and respects the camera extrinsics to ensure the direction is correct
        let move_delta = delta_time
            * action_state
                .single()
                .clamped_axis_pair(PlayerAction::Move)
                .unwrap()
                .xy();
        player_velocity.x += move_delta.x * character.speed;
        player_velocity.y += move_delta.y * character.speed;
    }
}
