use windows::core::PCSTR;
use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, GetWindowRect};
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CYFRAME, SM_CYCAPTION};

pub fn get_window_bounds_from_title(title: &str) -> Result<RECT, windows::core::Error> {
    unsafe {
        let hwnd = FindWindowA(PCSTR::null(), PCSTR(title.as_ptr() as _));
        let mut rect = RECT::default();
        GetWindowRect(hwnd, &mut rect)?;
        Ok(rect)
    }
}

pub fn get_title_bar_height() -> i32 {
    unsafe {
        // SM_CYCAPTION includes the height of the title bar
        let caption_height = GetSystemMetrics(SM_CYCAPTION);

        // SM_CYFRAME includes the height of the window frame (border)
        let frame_height = GetSystemMetrics(SM_CYFRAME);

        // The total height of the window decorations above the client area
        caption_height + (2 * frame_height)
    }
}
