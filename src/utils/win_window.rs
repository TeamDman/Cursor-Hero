use windows::core::PCSTR;
use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, GetWindowRect};
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CYCAPTION, SM_CYFRAME};

#[derive(Debug)]
pub enum WindowBoundsError {
    WindowNotFound,
    WindowsError(windows::core::Error),
}
pub fn get_window_bounds_from_title(title: &str) -> Result<RECT, WindowBoundsError> {
    unsafe {
        let hwnd = FindWindowA(PCSTR::null(), PCSTR(title.as_ptr() as _));
        if hwnd.0 == 0 {
            return Err(WindowBoundsError::WindowNotFound);
        }
        let mut rect = RECT::default();
        GetWindowRect(hwnd, &mut rect).map_err(WindowBoundsError::WindowsError)?;
        Ok(rect)
    }
}

pub fn get_window_inner_offset() -> (i32, i32) {
    unsafe {
        // SM_CYCAPTION includes the height of the title bar
        let caption_height = GetSystemMetrics(SM_CYCAPTION);

        // SM_CYFRAME includes the height of the window frame (border)
        let frame_height = GetSystemMetrics(SM_CYFRAME);

        (frame_height, caption_height + frame_height * 2)
    }
}
