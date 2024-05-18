extern crate gilrs;

use gilrs::Axis;
use gilrs::Event;
use gilrs::Gilrs;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!(
            "Connected: {} is {:?}",
            gamepad.name(),
            gamepad.power_info()
        );
    }

    loop {
        // Poll for events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
        }

        // You can also use cached gamepad state
        for (id, gamepad) in gilrs.gamepads() {
            let left_stick_x = gamepad.value(Axis::LeftStickX);
            let left_stick_y = gamepad.value(Axis::LeftStickY);
            println!(
                "{:?} Left Stick X: {}, Left Stick Y: {}",
                id, left_stick_x, left_stick_y
            );
        }

        // Sleep for a short duration to avoid spamming
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
