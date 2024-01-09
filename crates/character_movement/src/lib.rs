use bevy::prelude::*;
use bevy_xpbd_2d::math::*;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_physics::damping_plugin::DampingSystemSet;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum CharacterMovementSystemSet {
    Move,
}
pub struct CharacterMovementPlugin;

impl Plugin for CharacterMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .configure_sets(Update, CharacterMovementSystemSet::Move)
            .add_systems(
                Update,
                (
                    apply_movement
                        .in_set(CharacterMovementSystemSet::Move)
                        .after(DampingSystemSet::Dampen),
                    insert_movement,
                ),
            );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    Move,
}
// Exhaustively match `Action` and define the default binding to the input
impl Action {
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

    fn default_input_map() -> InputMap<Action> {
        let mut input_map = InputMap::default();

        for variant in Action::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

fn insert_movement(query: Query<Entity, Added<Character>>, mut commands: Commands) {
    for entity in query.iter() {
        info!("Inserting movement for {:?}", entity);
        commands
            .entity(entity)
            .insert(InputManagerBundle::<Action> {
                input_map: Action::default_input_map(),
                action_state: ActionState::default(),
            });
    }
}

fn apply_movement(
    time: Res<Time>,
    mut character_query: Query<(&Character, &mut LinearVelocity, &ActionState<Action>)>,
) {
    let delta_time = time.delta_seconds_f64().adjust_precision();
    for (c, mut c_vel, c_act) in character_query.iter_mut() {
        if c_act.pressed(Action::Move) {
            let move_delta = delta_time * c_act.clamped_axis_pair(Action::Move).unwrap().xy();
            c_vel.x += move_delta.x * c.speed;
            c_vel.y += move_delta.y * c.speed;
        }
    }
}
