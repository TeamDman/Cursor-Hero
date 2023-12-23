// mostly from 2d kinematic character example from https://github.com/Jondolf/bevy_xpbd

use bevy::input::common_conditions::input_toggle_active;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::{math::*, prelude::*};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cursor Hero Example - Character Movement".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins((PhysicsPlugins::default(), CharacterControllerPlugin))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        )
        .insert_resource(Gravity(Vector::ZERO))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
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
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(20.0, 12.5))
            .with_movement(12500.0, 0.92, 400.0),
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
    ));

    // Camera
    commands.spawn(Camera2dBundle::default());
}

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>()
            .add_systems(
                Update,
                (
                    keyboard_input,
                    gamepad_input,
                    apply_deferred,
                    movement,
                    apply_movement_damping,
                )
                    .chain(),
            )
            .register_type::<CharacterController>()
            .register_type::<MovementAcceleration>()
            .register_type::<MovementDampingFactor>()
            .register_type::<JumpImpulse>();
    }
}

/// An event sent for a movement input action.
#[derive(Event, Debug, Reflect)]
pub enum MovementAction {
    Move(Vec2),
    Jump,
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component, Reflect)]
pub struct CharacterController;

/// The acceleration used for character movement.
#[derive(Component, Reflect)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component, Reflect)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component, Reflect)]
pub struct JumpImpulse(Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle, Reflect)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
}

impl MovementBundle {
    pub const fn new(acceleration: Scalar, damping: Scalar, jump_impulse: Scalar) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Vector::NEG_Y)
                .with_max_time_of_impact(10.0),
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse);
        self
    }
}

/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);
    let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
    let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vec2::new(horizontal as Scalar, vertical as Scalar);
    if direction != Vec2::ZERO {
        movement_event_writer.send(MovementAction::Move(direction));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement_event_writer.send(MovementAction::Jump);
    }
}

/// Sends [`MovementAction`] events based on gamepad input.
fn gamepad_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
) {
    for gamepad in gamepads.iter() {
        let axis_lx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX,
        };
        let axis_ly = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickY,
        };

        let x = axes.get(axis_lx).unwrap_or_default();
        let y = axes.get(axis_ly).unwrap_or_default();
        if x != 0.0 || y != 0.0 {
            movement_event_writer.send(MovementAction::Move(Vec2::new(x, y)));
        }

        let jump_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South,
        };

        if buttons.just_pressed(jump_button) {
            movement_event_writer.send(MovementAction::Jump);
        }
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(&MovementAcceleration, &JumpImpulse, &mut LinearVelocity)>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for event in movement_event_reader.read() {
        for (movement_acceleration, jump_impulse, mut linear_velocity) in &mut controllers {
            match event {
                MovementAction::Move(direction) => {
                    linear_velocity.x += direction.x * movement_acceleration.0 * delta_time;
                    linear_velocity.y += direction.y * movement_acceleration.0 * delta_time;
                }
                MovementAction::Jump => {
                    linear_velocity.y += jump_impulse.0;
                }
            }
        }
    }
}

/// Slows down movement in the X direction.
fn apply_movement_damping(
    mut query: Query<(
        &MovementDampingFactor,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
) {
    for (damping_factor, mut linear_velocity, mut angular_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
        linear_velocity.y *= damping_factor.0;
        angular_velocity.0 *= damping_factor.0;
    }
}
