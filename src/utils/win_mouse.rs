use windows::{Win32::Foundation::POINT, Win32::UI::WindowsAndMessaging::{SetCursorPos, GetCursorPos}};

pub fn get_cursor_position() -> Result<(i32,i32), windows::core::Error> {
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
