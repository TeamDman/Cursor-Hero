use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_xpbd_2d::math::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

use cursor_hero_camera::camera_plugin::FollowWithCamera;
use cursor_hero_physics::damping_plugin::DampingSystemSet;
use cursor_hero_physics::damping_plugin::MovementDamping;
use cursor_hero_winutils::win_mouse::get_cursor_position;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum CharacterSystemSet {
    Spawn,
    Position,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .configure_sets(Startup, CharacterSystemSet::Spawn)
            .configure_sets(Update, CharacterSystemSet::Position)
            .add_systems(Startup, spawn_character.in_set(CharacterSystemSet::Spawn))
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

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Character {
    #[inspector(min = 0.0)]
    pub speed: f32,
    #[inspector(min = 0.0)]
    pub default_speed: f32,
    #[inspector(min = 0.0)]
    pub sprint_speed: f32,
    #[inspector(min = 0.0)]
    pub reach: f32,
    #[inspector(min = 0.0)]
    pub default_reach: f32,
    #[inspector(min = 0.0)]
    pub sprint_reach: f32,
}
impl Default for Character {
    fn default() -> Self {
        Self {
            speed: 5000.0,
            default_speed: 5000.0,
            sprint_speed: 1000.0,
            reach: 200.0,
            default_reach: 200.0,
            sprint_reach: 50.0,
        }
    }
}

#[derive(Component, Reflect, Eq, PartialEq, Debug)]
pub enum CharacterColor {
    Unfocused,
    FocusedWithCamera,
}
impl Default for CharacterColor {
    fn default() -> Self {
        Self::FocusedWithCamera
    }
}

impl CharacterColor {
    pub fn as_color(self) -> Color {
        match self {
            Self::Unfocused => Color::rgb(0.2, 0.7, 0.9),
            Self::FocusedWithCamera => Color::rgb(0.149, 0.549, 0.184),
        }
    }
    pub fn as_material(self) -> ColorMaterial {
        ColorMaterial::from(self.as_color())
    }
}

fn spawn_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let default_material = materials.add(CharacterColor::FocusedWithCamera.as_material());
    let os_cursor_pos = get_cursor_position().expect("Should be able to fetch cursor pos from OS");
    let mut character = commands.spawn((
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
            material: default_material,
            transform: Transform::from_xyz(os_cursor_pos.x, -os_cursor_pos.y, 100.0),
            ..default()
        },
        Character::default(),
        MovementDamping { factor: 0.90 },
        Name::new("Character"),
        InputManagerBundle::<PlayerAction> {
            input_map: PlayerAction::default_input_map(),
            action_state: ActionState::default(),
        },
        RigidBody::Kinematic,
        Collider::capsule(20.0, 12.5),
        SpatialListener::new(7.0),
    ));
    if CharacterColor::default() == CharacterColor::FocusedWithCamera {
        character.insert(FollowWithCamera);
    }
    info!("Character spawn command issued");
}

fn is_character_physics_ready(query: Query<&LinearVelocity, With<Character>>) -> bool {
    query.get_single().is_ok()
}
fn has_movement(action_state: Query<(&ActionState<PlayerAction>, &Character)>) -> bool {
    let (act, character) = action_state.single();
    act.pressed(PlayerAction::Move) || character.speed > 5000.0
}

fn apply_movement(
    time: Res<Time>,
    action_state: Query<&ActionState<PlayerAction>, With<Character>>,
    mut character_query: Query<(&mut LinearVelocity, &mut Character)>,
) {
    let (mut player_velocity, mut character) = character_query.single_mut();
    let delta_time = time.delta_seconds_f64().adjust_precision();
    if action_state.single().pressed(PlayerAction::Move) {
        let move_delta = delta_time
            * action_state
                .single()
                .clamped_axis_pair(PlayerAction::Move)
                .unwrap()
                .xy();

        if move_delta.x == 0.0 && move_delta.y == 0.0 {
            debug!("No movement detected");
            debug!("Resetting to base speed");
            character.speed = 5000.0;
        } else {
            // debug!("Movement detected, {:?}", move_delta);
        }

        // Increment speed if continuously moving
        // character.speed += character.speed_increment;

        character.speed += 1000.0;
        player_velocity.x += move_delta.x * character.speed;
        player_velocity.y += move_delta.y * character.speed;
    } else {
        // Reset speed if not moving
        // character.speed = character.base_speed;
        debug!("Resetting to base speed");
        character.speed = 5000.0;
    }
}
