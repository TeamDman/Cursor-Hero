use bevy::{
    input::gamepad::GamepadEvent, input::keyboard::KeyboardInput, prelude::*,
};


pub struct ActiveInputStatePlugin;

impl Plugin for ActiveInputStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ActiveInput>()
            .add_systems(
                Update,
                activate_gamepad.run_if(in_state(ActiveInput::MouseKeyboard)),
            )
            .add_systems(Update, activate_mkb.run_if(in_state(ActiveInput::Gamepad)));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ActiveInput {
    #[default]
    MouseKeyboard,
    Gamepad,
}

/// Switch the gamepad when any button is pressed or any axis input used
fn activate_gamepad(
    mut next_state: ResMut<NextState<ActiveInput>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for ev in gamepad_evr.read() {
        match ev {
            GamepadEvent::Button(_) | GamepadEvent::Axis(_) => {
                info!("Switching to gamepad input");
                next_state.set(ActiveInput::Gamepad);
                return;
            }
            _ => (),
        }
    }
}

/// Switch to mouse and keyboard input when any keyboard button is pressed
fn activate_mkb(
    mut next_state: ResMut<NextState<ActiveInput>>,
    mut kb_evr: EventReader<KeyboardInput>,
) {
    for _ev in kb_evr.read() {
        info!("Switching to mouse and keyboard input");
        next_state.set(ActiveInput::MouseKeyboard);
    }
}
