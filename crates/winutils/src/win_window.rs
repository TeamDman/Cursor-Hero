use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::FindWindowA;
use windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics;
use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;
use windows::Win32::UI::WindowsAndMessaging::IsWindowVisible;
use windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow;
use windows::Win32::UI::WindowsAndMessaging::ShowWindow;
use windows::Win32::UI::WindowsAndMessaging::SM_CYCAPTION;
use windows::Win32::UI::WindowsAndMessaging::SM_CYFRAME;
use windows::Win32::UI::WindowsAndMessaging::SW_RESTORE;

#[derive(Debug)]
pub enum WindowBoundsError {
    WindowNotFound,
    WindowsError(windows::core::Error),
}

#[allow(dead_code)]
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

pub fn get_window_bounds(hwnd: isize) -> Result<RECT, WindowBoundsError> {
    unsafe {
        let mut rect = RECT::default();
        GetWindowRect(HWND(hwnd), &mut rect).map_err(WindowBoundsError::WindowsError)?;
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

pub fn focus_window(hwnd: isize) {
    unsafe {
        // Convert the isize to HWND
        let hwnd = HWND(hwnd);

        // If the window is minimized, restore it before setting it to the foreground.
        if !IsWindowVisible(hwnd).as_bool() {
            ShowWindow(hwnd, SW_RESTORE);
        }

        // Bring the window to the foreground
        SetForegroundWindow(hwnd);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bevy_inspector_egui::egui::TextBuffer;
    use windows::Win32::Foundation::BOOL;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Foundation::LPARAM;
    use windows::Win32::UI::WindowsAndMessaging::EnumWindows;
    use windows::Win32::UI::WindowsAndMessaging::GetWindowTextA;
    use windows::Win32::UI::WindowsAndMessaging::IsWindowVisible;

    unsafe extern "system" fn enum_windows_proc(hwnd: HWND, _lp: LPARAM) -> BOOL {
        if IsWindowVisible(hwnd).as_bool() {
            let mut title = [0u8; 256];
            let title_length = GetWindowTextA(hwnd, &mut title);

            if title_length > 0 {
                let title = String::from_utf8_lossy(&title[..title_length as usize]);
                println!("Window title: \"{}\"", &title);
                match get_window_bounds_from_title(title.as_str()) {
                    Ok(rect) => {
                        println!("Window bounds: {:?}", rect);
                    }
                    Err(err) => {
                        eprintln!("Error: {:?}", err);
                        // panic!("Error: {:?}", err);
                    }
                }
            }
        }
        BOOL::from(true) // Continue enumeration
    }

    #[test]
    fn enum_windows() {
        unsafe {
            EnumWindows(Some(enum_windows_proc), LPARAM(0)).unwrap();
        }
    }

    #[test]
    fn test_get_window_bounds_from_title() {
        let title = "Cursor Hero";
        let result = get_window_bounds_from_title(title);
        assert!(result.is_ok(), "Error: {:?}", result.err());
        let rect = result.unwrap();
        assert!(rect.left < rect.right);
        assert!(rect.top < rect.bottom);
    }
}
