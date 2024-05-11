use bevy::prelude::*;
use windows::Win32::Foundation::POINT;
use windows::Win32::UI::Input::KeyboardAndMouse::SendInput;
use windows::Win32::UI::Input::KeyboardAndMouse::INPUT;
use windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0;
use windows::Win32::UI::Input::KeyboardAndMouse::INPUT_MOUSE;
use windows::Win32::UI::Input::KeyboardAndMouse::INPUT_TYPE;
use windows::Win32::UI::Input::KeyboardAndMouse::KEYBDINPUT;
use windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS;
use windows::Win32::UI::Input::KeyboardAndMouse::MOUSEEVENTF_LEFTDOWN;
use windows::Win32::UI::Input::KeyboardAndMouse::MOUSEEVENTF_LEFTUP;
use windows::Win32::UI::Input::KeyboardAndMouse::MOUSEEVENTF_RIGHTDOWN;
use windows::Win32::UI::Input::KeyboardAndMouse::MOUSEEVENTF_RIGHTUP;
use windows::Win32::UI::Input::KeyboardAndMouse::MOUSEEVENTF_WHEEL;
use windows::Win32::UI::Input::KeyboardAndMouse::MOUSEINPUT;
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;
use windows::Win32::UI::WindowsAndMessaging::WHEEL_DELTA;

pub fn get_host_cursor_position() -> Result<IVec2, windows::core::Error> {
    unsafe {
        let mut point = POINT::default();
        GetCursorPos(&mut point)?;
        Ok(IVec2::new(point.x, point.y))
    }
}

pub fn set_host_cursor_position(position: IVec2) -> Result<(), windows::core::Error> {
    unsafe {
        SetCursorPos(position.x, position.y)?;
        Ok(())
    }
}

pub fn left_mouse_down() -> Result<(), windows::core::Error> {
    // Prepare a mouse input for left button down
    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_LEFTDOWN,
        time: 0,
        dwExtraInfo: 0,
    };

    // Prepare an INPUT structure for the down event
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: mouse_input },
    };

    // Send the input for button down
    unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) };

    Ok(())
}

pub fn left_mouse_up() -> Result<(), windows::core::Error> {
    // Prepare a mouse input for left button up
    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_LEFTUP,
        time: 0,
        dwExtraInfo: 0,
    };

    // Prepare an INPUT structure for the up event
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: mouse_input },
    };

    // Send the input for button up
    unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) };

    Ok(())
}

#[allow(dead_code)]
pub fn left_click() -> Result<(), windows::core::Error> {
    left_mouse_down()?;
    left_mouse_up()?;
    Ok(())
}

pub fn right_mouse_down() -> Result<(), windows::core::Error> {
    // Prepare a mouse input for right button down
    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_RIGHTDOWN,
        time: 0,
        dwExtraInfo: 0,
    };

    // Prepare an INPUT structure for the down event
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: mouse_input },
    };

    // Send the input for button down
    unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) };

    Ok(())
}

pub fn right_mouse_up() -> Result<(), windows::core::Error> {
    // Prepare a mouse input for right button up
    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_RIGHTUP,
        time: 0,
        dwExtraInfo: 0,
    };

    // Prepare an INPUT structure for the up event
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: mouse_input },
    };

    // Send the input for button up
    unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) };

    Ok(())
}

// Constants
const INPUT_KEYBOARD: u32 = 1;
const VK_F23: u16 = 0x86;
const KEYEVENTF_KEYUP: u32 = 0x0002;

pub fn press_f23_key() -> Result<(), windows::core::Error> {
    let keyboard_input = KEYBDINPUT {
        wVk: VIRTUAL_KEY(VK_F23),
        wScan: 0,
        dwFlags: KEYBD_EVENT_FLAGS(0),
        time: 0,
        dwExtraInfo: 0,
    };

    let input = INPUT {
        r#type: INPUT_TYPE(INPUT_KEYBOARD),
        Anonymous: INPUT_0 { ki: keyboard_input },
    };

    unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) };

    Ok(())
}

pub fn release_f23_key() -> Result<(), windows::core::Error> {
    let keyboard_input = KEYBDINPUT {
        wVk: VIRTUAL_KEY(VK_F23),
        wScan: 0,
        dwFlags: KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP),
        time: 0,
        dwExtraInfo: 0,
    };

    let input = INPUT {
        r#type: INPUT_TYPE(INPUT_KEYBOARD),
        Anonymous: INPUT_0 { ki: keyboard_input },
    };

    unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) };

    Ok(())
}

pub fn scroll_wheel(scale: f32) -> Result<(), windows::core::Error> {
    let mouse_input = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: (WHEEL_DELTA as f32 * scale) as i32 as u32, // https://github.com/microsoft/windows-rs/issues/2708
        dwFlags: MOUSEEVENTF_WHEEL,
        time: 0,
        dwExtraInfo: 0,
    };

    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: mouse_input },
    };

    unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) };

    Ok(())
}
