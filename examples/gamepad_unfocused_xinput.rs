use windows::{
    core::Result,
    Win32::{
        Foundation::{BOOL, ERROR_DEVICE_NOT_CONNECTED, ERROR_SUCCESS},
        Gaming::XInput::{
            XInputGetCapabilities, XInputGetState, XINPUT_CAPABILITIES, XINPUT_GAMEPAD, XINPUT_STATE,
        },
    },
};

fn main() -> Result<()> {
    const MAX_CONTROLLERS: usize = 4; // XInput supports up to 4 controllers

    // Check each possible controller index
    for i in 0..MAX_CONTROLLERS {
        let mut capabilities: XINPUT_CAPABILITIES = Default::default();
        let result = unsafe { XInputGetCapabilities(i as u32, 0, &mut capabilities) };

        if result == ERROR_SUCCESS.0 {
            println!("Controller {} is connected", i);
        } else if result == ERROR_DEVICE_NOT_CONNECTED.0 {
            println!("Controller {} is not connected", i);
        } else {
            println!("Controller {} status unknown", i);
        }
    }

    loop {
        for i in 0..MAX_CONTROLLERS {
            let mut state: XINPUT_STATE = Default::default();
            if unsafe { XInputGetState(i as u32, &mut state) } == ERROR_SUCCESS.0 {
                let gamepad: XINPUT_GAMEPAD = state.Gamepad;
                let left_thumb_x = gamepad.sThumbLX;
                
                println!("Controller {}: Left Thumb X = {}", i, left_thumb_x);
            }
        }

        // Sleep for a short duration to avoid spamming the console
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
