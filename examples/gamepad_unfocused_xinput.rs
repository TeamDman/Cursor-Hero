use windows::core::Result;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::ERROR_DEVICE_NOT_CONNECTED;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::Gaming::XInput::XInputGetCapabilities;
use windows::Win32::Gaming::XInput::XInputGetState;
use windows::Win32::Gaming::XInput::XINPUT_CAPABILITIES;
use windows::Win32::Gaming::XInput::XINPUT_GAMEPAD;
use windows::Win32::Gaming::XInput::XINPUT_STATE;

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
