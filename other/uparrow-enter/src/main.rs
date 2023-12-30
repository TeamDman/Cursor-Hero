extern crate enigo;

use enigo::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut enigo = Enigo::new();

    // Simulate pressing the up arrow key
    enigo.key_down(Key::UpArrow);
    thread::sleep(Duration::from_millis(100));
    enigo.key_up(Key::UpArrow);

    // Wait a bit between keys
    thread::sleep(Duration::from_millis(100));

    // Simulate pressing the enter key
    enigo.key_down(Key::Return);
    thread::sleep(Duration::from_millis(100));
    enigo.key_up(Key::Return);
}
