use windows::{Win32::Foundation::POINT, Win32::UI::WindowsAndMessaging::GetCursorPos};

pub fn get_cursor_position() -> Result<(i32,i32), windows::core::Error> {
    unsafe {
        let mut point = POINT::default();
        GetCursorPos(&mut point)?;
        Ok((point.x, point.y))
    }
}
