use bevy::input::gamepad::GamepadEvent;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

pub struct ActiveInputStatePlugin;

impl Plugin for ActiveInputStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ActiveInput>();
        app.insert_resource(ActiveInput::MouseAndKeyboard);
        app.add_systems(
            Update,
            (
                activate_gamepad.run_if(in_state(ActiveInput::MouseAndKeyboard)),
                activate_mkb.run_if(in_state(ActiveInput::Gamepad)),
            ),
        );
    }
}

#[derive(Resource, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ActiveInput {
    #[default]
    MouseAndKeyboard,
    Gamepad,
}

/// Switch the gamepad when any button is pressed or any axis input used
fn activate_gamepad(
    mut next_state: ResMut<NextState<ActiveInput>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
    mut active_input: ResMut<ActiveInput>,
) {
    let mut debounce = false;
    for ev in gamepad_evr.read() {
        match ev {
            GamepadEvent::Button(_) => {
                if !debounce {
                    info!("Switching to gamepad input because of {:?}", ev);
                    next_state.set(ActiveInput::Gamepad);
                    *active_input = ActiveInput::Gamepad;
                    debounce = true;
                }
            }
            GamepadEvent::Axis(ax) => {
                if ax.value != 0.0 && !debounce {
                    info!("Switching to gamepad input because of {:?}", ev);
                    next_state.set(ActiveInput::Gamepad);
                    *active_input = ActiveInput::Gamepad;
                    debounce = true;
                }
            }
            _ => (),
        }
    }
}

/// Switch to mouse and keyboard input when any keyboard button is pressed
fn activate_mkb(
    mut next_state: ResMut<NextState<ActiveInput>>,
    mut kb_reader: EventReader<KeyboardInput>,
    mut active_input: ResMut<ActiveInput>,
) {
    if kb_reader.read().count() > 0 {
        info!("Switching to mouse and keyboard input");
        next_state.set(ActiveInput::MouseAndKeyboard);
        *active_input = ActiveInput::MouseAndKeyboard;
    }
}
