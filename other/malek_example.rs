// https://discord.com/channels/691052431525675048/1034547742262951966/1190443687990284368
use bevy::app::{App, PreUpdate};
use bevy::prelude::{
    Commands, DetectChangesMut, Input, IntoSystemConfigs, KeyCode, NonSendMut, Plugin, ResMut,
    Resource, Startup, World,
};
use hookmap_core::button::ButtonKind::Key;
use hookmap_core::button::{Button, ButtonAction};
use hookmap_core::event::Event;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use willhook::{InputEvent, KeyPress, KeyboardEvent, KeyboardKey};

pub struct GlobalInputPlugin;
impl Plugin for GlobalInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_input);
        app.add_systems(PreUpdate, keyboard_input_system);
    }
}

#[derive(PartialEq, Hash, Debug, Copy, Clone, Default)]
pub struct InputState {
    pub active: bool,
    pub just_changed: bool,
}

impl InputState {
    pub fn new(active: bool, just_changed: bool) -> Self {
        Self {
            active,
            just_changed,
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct KeyboardState {
    keys: Arc<Mutex<HashMap<KeyCode, InputState>>>,
}

pub struct KeyboardChannel(Receiver<(KeyCode, ButtonAction)>);

/*pub fn keyboard_press(key: KeyCode, key_press: KeyPress, keys: &mut MutexGuard<HashMap<KeyCode, InputState>>) {
    match key_press {
        KeyPress::Down(_) => { keys.insert(key, InputState::new(true, true)); }
        KeyPress::Up(_) => { keys.insert(key, InputState::new(false, true)); }
        _ => {}
    };
}*/

// pub fn keyboard_press(key: KeyCode, key_press: KeyPress, keys: Sender<(KeyCode, KeyPress)>) {
//     keys.send((key, key_press)).unwrap();
// }

pub fn keyboard_press2(
    key: KeyCode,
    button_action: ButtonAction,
    keys: Sender<(KeyCode, ButtonAction)>,
) {
    keys.send((key, button_action)).unwrap();
}

pub fn keyboard_input_system(
    mut key_input: ResMut<Input<KeyCode>>,
    mut keyboard_channel: NonSendMut<KeyboardChannel>,
) {
    // Avoid clearing if it's not empty to ensure change detection is not triggered.
    key_input.bypass_change_detection().clear();
    for (keycode, press) in keyboard_channel.0.try_iter() {
        println!("keycode: {:?}", keycode);
        match press {
            ButtonAction::Press => key_input.press(keycode),
            ButtonAction::Release => key_input.release(keycode),
        };
    }
}

pub fn setup_input(world: &mut World) {
    //let keyboard_state = KeyboardState::default();
    //commands.insert_resource(keyboard_state.clone());
    let (mut tx, rx) = std::sync::mpsc::channel();
    world.insert_non_send_resource(KeyboardChannel(rx));

    thread::spawn(move || {
        let rx = hookmap_core::install_hook();
        let tx = tx.clone();
        while let Ok((event, native_handler)) = rx.recv() {
            let tx = tx.clone();
            match event {
                Event::Button(button) => {
                    native_handler.dispatch();
                    match button.target {
                        Button::RightArrow => keyboard_press2(KeyCode::Right, button.action, tx),
                        Button::LeftArrow => keyboard_press2(KeyCode::Left, button.action, tx),
                        Button::UpArrow => keyboard_press2(KeyCode::Up, button.action, tx),
                        Button::DownArrow => keyboard_press2(KeyCode::Down, button.action, tx),
                        Button::Q => keyboard_press2(KeyCode::Q, button.action, tx),
                        Button::LCtrl => keyboard_press2(KeyCode::ControlLeft, button.action, tx),
                        Button::LShift => keyboard_press2(KeyCode::ShiftLeft, button.action, tx),
                        Button::LAlt => keyboard_press2(KeyCode::AltLeft, button.action, tx),
                        Button::Esc => keyboard_press2(KeyCode::Escape, button.action, tx),
                        _ => {}
                    }
                }
                Event::Wheel(_) => {}
                Event::Cursor(_) => {}
            }
        }
    });
    return;
}