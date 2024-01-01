extern crate enigo;

use enigo::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut enigo = Enigo::new();
    
    enigo.key_down(Key::LControl);
    thread::sleep(Duration::from_millis(10));
    enigo.key_down(Key::C);
    thread::sleep(Duration::from_millis(50));
    enigo.key_up(Key::C);
    enigo.key_up(Key::LControl);

    thread::sleep(Duration::from_millis(400));

    enigo.key_down(Key::UpArrow);
    thread::sleep(Duration::from_millis(50));
    enigo.key_up(Key::UpArrow);

    thread::sleep(Duration::from_millis(50));

    enigo.key_down(Key::Return);
    thread::sleep(Duration::from_millis(100));
    enigo.key_up(Key::Return);
}
