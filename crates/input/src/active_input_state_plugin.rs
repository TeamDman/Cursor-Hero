use bevy::input::gamepad::GamepadEvent;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use cursor_hero_host_event_types::prelude::HostEvent;
use cursor_hero_cursor_types::cursor_action_types::CursorAction;
use cursor_hero_cursor_types::cursor_types::MainCursor;
use leafwing_input_manager::action_state::ActionState;

pub struct ActiveInputStatePlugin;

impl Plugin for ActiveInputStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<InputMethod>();
        app.insert_resource(InputMethod::MouseAndKeyboard);
        app.add_systems(Update, update_input_method);
    }
}

#[derive(Resource, Reflect, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
#[reflect(Resource)]
pub enum InputMethod {
    #[default]
    MouseAndKeyboard,
    Keyboard,
    Gamepad,
}

fn update_input_method(
    current_state: Res<State<InputMethod>>,
    mut next_state: ResMut<NextState<InputMethod>>,
    mut gamepad_events: EventReader<GamepadEvent>,
    mut host_events: EventReader<HostEvent>,
    mut keyboard_events: EventReader<KeyboardInput>,
    cursor_actions: Query<&ActionState<CursorAction>, With<MainCursor>>,
) {
    let current_input_method = *current_state.get();
    let keyboard_used = keyboard_events.read().count() > 0;
    let gamepad_used = gamepad_events
        .read()
        .filter(|e| match e {
            GamepadEvent::Button(_) => true,
            GamepadEvent::Axis(ax) => ax.value != 0.0,
            _ => false,
        })
        .count()
        > 0;
    let mouse_used = host_events
        .read()
        .filter(|e| **e == HostEvent::MousePhysicallyMoved)
        .count()
        > 0;
    let cursor_moved = cursor_actions.iter().any(|a| {
        a.axis_pair(CursorAction::Move)
            .map(|xy| !xy.x().is_nan() && !xy.y().is_nan() && xy.xy() != Vec2::ZERO)
            .unwrap_or(false)
    });

    #[derive(Debug)]
    struct DecisionInfo {
        current_input_method: InputMethod,
        keyboard_used: bool,
        gamepad_used: bool,
        mouse_used: bool,
        cursor_moved: bool,
    }
    let decision_info = DecisionInfo {
        current_input_method,
        keyboard_used,
        gamepad_used,
        mouse_used,
        cursor_moved,
    };
    let proposed_state = match decision_info {
        DecisionInfo {
            gamepad_used: true, ..
        } => InputMethod::Gamepad,
        DecisionInfo {
            current_input_method: InputMethod::MouseAndKeyboard,
            cursor_moved: true,
            ..
        } => InputMethod::Keyboard,
        DecisionInfo {
            mouse_used: true, ..
        }
        | DecisionInfo {
            current_input_method: InputMethod::Gamepad,
            keyboard_used: true,
            ..
        } => InputMethod::MouseAndKeyboard,
        DecisionInfo {
            current_input_method,
            ..
        } => current_input_method,
    };
    if proposed_state != current_input_method {
        next_state.set(proposed_state);
    }
}
