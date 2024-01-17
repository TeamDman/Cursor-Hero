use bevy::prelude::*;
use bevy_xpbd_2d::math::*;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_physics::damping_plugin::DampingSystemSet;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MovementAction>::default())
            .register_type::<Movement>()
            .add_event::<MovementEvent>()
            .add_systems(
                Update,
                (
                    apply_movement.after(DampingSystemSet::Dampen),
                    insert_movement,
                ),
            );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MovementAction {
    Move,
}
// Exhaustively match `Action` and define the default binding to the input
impl MovementAction {
    fn default_gamepad_binding(&self) -> UserInput {
        // Match against the provided action to get the correct default gamepad input
        match self {
            Self::Move => UserInput::Single(InputKind::DualAxis(DualAxis::left_stick())),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        // Match against the provided action to get the correct default gamepad input
        match self {
            Self::Move => UserInput::VirtualDPad(VirtualDPad::wasd()),
        }
    }

    fn default_input_map() -> InputMap<MovementAction> {
        let mut input_map = InputMap::default();

        for variant in MovementAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

#[derive(Event, Debug, Reflect)]
pub enum MovementEvent {
    AddMovement { target_id: Entity },
    RemoveMovement { target_id: Entity },
}

fn insert_movement(query: Query<Entity, Added<Movement>>, mut commands: Commands) {
    for entity in query.iter() {
        info!("Inserting movement for {:?}", entity);
        commands
            .entity(entity)
            .insert(InputManagerBundle::<MovementAction> {
                input_map: MovementAction::default_input_map(),
                action_state: ActionState::default(),
            });
    }
}

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Movement {
    #[inspector(min = 0.0)]
    pub speed: f32,
    #[inspector(min = 0.0)]
    pub default_speed: f32,
    #[inspector(min = 0.0)]
    pub sprint_speed: f32,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            speed: 800.0,
            default_speed: 800.0,
            sprint_speed: 80000.0,
        }
    }
}
fn apply_movement(
    time: Res<Time>,
    mut character_query: Query<(&Movement, &mut LinearVelocity, &ActionState<MovementAction>)>,
) {
    let delta_time = time.delta_seconds_f64().adjust_precision();
    for (c, mut c_vel, c_act) in character_query.iter_mut() {
        if c_act.pressed(MovementAction::Move) {
            let move_delta =
                delta_time * c_act.clamped_axis_pair(MovementAction::Move).unwrap().xy();
            c_vel.x += move_delta.x * c.speed;
            c_vel.y += move_delta.y * c.speed;
        }
    }
}

fn handle_events(
    mut commands: Commands,
    mut movement_events: EventReader<MovementEvent>,
    mut character_query: Query<(Entity, &mut Movement)>,
) {
    for event in movement_events.read() {
        match event {
            MovementEvent::AddMovement { target_id } => {
                info!("Adding movement for {:?}", target_id);
                commands.entity(*target_id).insert(Movement::default());
            }
            MovementEvent::RemoveMovement { target_id } => {
                info!("Removing movement for {:?}", target_id);
                commands.entity(*target_id).remove::<Movement>();
            }
        }
    }
}
