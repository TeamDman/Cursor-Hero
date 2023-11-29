use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::{math::*, prelude::*};
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use leafwing_input_manager::user_input::InputKind;

use crate::active_input_state_plugin::ActiveInput;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Startup, spawn_character)
            .add_systems(
                Update,
                (
                    player_mouse_look
                        .run_if(in_state(ActiveInput::MouseKeyboard)),
                    update_character_velocity
                        .after(player_mouse_look)
                        .run_if(is_character_physics_ready),
                    apply_movement_damping
                        .before(update_character_velocity),
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
            input_map.insert(variant.default_mkb_binding(), variant.clone());
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
    #[inspector(min = 0.5, max = 0.999)]
    pub damping_factor: f32,
}

fn spawn_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        // SpriteBundle {
        //     sprite: Sprite {
        //         custom_size: Some(Vec2::new(100.0, 100.0)),
        //         ..default()
        //     },
        //     texture: asset_server.load("character.png"),
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
            transform: Transform::from_xyz(0.0, -100.0, 1.0),
            ..default()
        },
        Character {
            speed: 5000.0,
            damping_factor: 0.95,
        },
        Name::new("Cursor Character"),
        InputManagerBundle::<PlayerAction> {
            input_map: PlayerAction::default_input_map(),
            action_state: ActionState::default(),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::capsule(20.0, 12.5),
    ));
}

/// Note that we handle the action_state mutation differently here than in the `mouse_position`
/// example. Here we don't use an `ActionDriver`, but change the action data directly.
fn player_mouse_look(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    mut player_query: Query<(&Transform, &mut ActionState<PlayerAction>), With<Character>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Update each actionstate with the mouse position from the window
    // by using the referenced entities in ActionStateDriver and the stored action as
    // a key into the action data
    let (camera_transform, camera) = camera_query.get_single().expect("Need a single camera");
    let (player_transform, mut action_state) =
        player_query.get_single_mut().expect("Need a single player");
    let window = window_query
        .get_single()
        .expect("Need a single primary window");

    // Many steps can fail here, so we'll wrap in an option pipeline
    // First check if cursor is in window
    // Then check if the ray intersects the plane defined by the player
    // Then finally compute the point along the ray to look at
    if let Some(p) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .and_then(|ray| Some(ray).zip(ray.intersect_plane(player_transform.translation, Vec3::Y)))
        .map(|(ray, p)| ray.get_point(p))
    {
        let diff = (p - player_transform.translation).xz();
        if diff.length_squared() > 1e-3f32 {
            // Press the look action, so we can check that it is active
            action_state.press(PlayerAction::Look);
            // Modify the action data to set the axis
            let action_data = action_state.action_data_mut(PlayerAction::Look);
            // Flipping y sign here to be consistent with gamepad input. We could also invert the gamepad y axis
            action_data.axis_pair = Some(DualAxisData::from_xy(Vec2::new(diff.x, -diff.y)));
        }
    }
}

fn is_character_physics_ready(query: Query<&LinearVelocity, With<Character>>) -> bool {
    query.get_single().is_ok()
}

fn update_character_velocity(
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

    if action_state.single().pressed(PlayerAction::Look) {
        let look = action_state
            .single()
            .axis_pair(PlayerAction::Look)
            .unwrap()
            .xy()
            .normalize();
        println!("Player looking in direction: {}", look);
    }

    if action_state.single().just_pressed(PlayerAction::Click) {
        // println!("Click!")
    }
}

fn apply_movement_damping(
    mut query: Query<(&Character, &mut LinearVelocity, &mut AngularVelocity)>,
) {
    for (character, mut linear_velocity, mut angular_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= character.damping_factor;
        linear_velocity.y *= character.damping_factor;
        angular_velocity.0 *= character.damping_factor;
    }
}
