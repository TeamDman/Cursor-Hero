use std::thread;

use uiautomation::UIAutomation;
use uiautomation::UIElement;
use windows::{
    Win32::Foundation::POINT,
    Win32::UI::{
        Input::KeyboardAndMouse::{
            SendInput, INPUT, INPUT_0, INPUT_MOUSE, INPUT_TYPE, MOUSEEVENTF_LEFTDOWN,
            MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEINPUT,
        },
        WindowsAndMessaging::{GetCursorPos, SetCursorPos},
    },
};

pub fn get_cursor_position() -> Result<(i32, i32), windows::core::Error> {
    unsafe {
        let mut point = POINT::default();
        GetCursorPos(&mut point)?;
        Ok((point.x, point.y))
    }
}

pub fn set_cursor_position(x: i32, y: i32) -> Result<(), windows::core::Error> {
    unsafe {
        SetCursorPos(x, y)?;
        Ok(())
    }
}

pub fn left_click() -> Result<(), windows::core::Error> {
    // Prepare a mouse input for left button down
    let mouse_input_down = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_LEFTDOWN,
        time: 0,
        dwExtraInfo: 0,
    };

    // Prepare an INPUT structure for the down event
    let input_down = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: mouse_input_down,
        },
    };

    // Send the input for button down
    unsafe { SendInput(&[input_down], std::mem::size_of::<INPUT>() as i32) };

    // Prepare a mouse input for left button up
    let mouse_input_up = MOUSEINPUT {
        dwFlags: MOUSEEVENTF_LEFTUP,
        ..mouse_input_down
    };

    // Prepare an INPUT structure for the up event
    let input_up = INPUT {
        Anonymous: INPUT_0 { mi: mouse_input_up },
        ..input_down
    };

    // Send the input for button up
    unsafe { SendInput(&[input_up], std::mem::size_of::<INPUT>() as i32) };

    Ok(())
}

pub fn right_click() -> Result<(), windows::core::Error> {
    // Prepare a mouse input for right button down
    let mouse_input_down = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_RIGHTDOWN,
        time: 0,
        dwExtraInfo: 0,
    };

    // Prepare an INPUT structure for the down event
    let input_down = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: mouse_input_down,
        },
    };

    // Send the input for button down
    unsafe { SendInput(&[input_down], std::mem::size_of::<INPUT>() as i32) };

    // Prepare a mouse input for right button up
    let mouse_input_up = MOUSEINPUT {
        dwFlags: MOUSEEVENTF_RIGHTUP,
        ..mouse_input_down
    };

    // Prepare an INPUT structure for the up event
    let input_up = INPUT {
        Anonymous: INPUT_0 { mi: mouse_input_up },
        ..input_down
    };

    // Send the input for button up
    unsafe { SendInput(&[input_up], std::mem::size_of::<INPUT>() as i32) };

    Ok(())
}

pub fn ui_left_click(x: i32, y: i32) -> Result<(), uiautomation::Error> {
    let automation = UIAutomation::new().unwrap();
    if let Ok(root) = automation.element_from_point(uiautomation::types::Point::new(x, y)) {
        root.click().unwrap();
    }
    Ok(())
}

pub fn ui_right_click(x: i32, y: i32) -> Result<(), uiautomation::Error> {
    let automation = UIAutomation::new()?;
    if let Ok(root) = automation.element_from_point(uiautomation::types::Point::new(x, y)) {
        root.right_click()?;
    }
    Ok(())
}
