use bevy::math::IRect;
use bevy::math::IVec2;
use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::RECT;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows::Win32::UI::WindowsAndMessaging::FindWindowA;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
use windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics;
use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;
use windows::Win32::UI::WindowsAndMessaging::IsWindowVisible;
use windows::Win32::UI::WindowsAndMessaging::SendMessageW;
use windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow;
use windows::Win32::UI::WindowsAndMessaging::ShowWindow;
use windows::Win32::UI::WindowsAndMessaging::HTCAPTION;
use windows::Win32::UI::WindowsAndMessaging::SM_CYCAPTION;
use windows::Win32::UI::WindowsAndMessaging::SM_CYFRAME;
use windows::Win32::UI::WindowsAndMessaging::SW_RESTORE;
use windows::Win32::UI::WindowsAndMessaging::WM_NCLBUTTONDOWN;

use crate::ToBevyIRect;

impl ToBevyIRect for RECT {
    fn to_bevy_irect(&self) -> IRect {
        IRect {
            min: IVec2::new(self.left, self.top),
            max: IVec2::new(self.right, self.bottom),
        }
    }
}

#[derive(Debug)]
pub enum WindowBoundsError {
    WindowNotFound,
    WindowsError(windows::core::Error),
}

#[allow(dead_code)]
pub fn get_window_bounds_from_title(title: &str) -> Result<IRect, WindowBoundsError> {
    unsafe {
        let hwnd = FindWindowA(PCSTR::null(), PCSTR(title.as_ptr() as _));
        if hwnd.0 == 0 {
            return Err(WindowBoundsError::WindowNotFound);
        }
        let mut rect = RECT::default();
        GetWindowRect(hwnd, &mut rect).map_err(WindowBoundsError::WindowsError)?;
        Ok(rect.to_bevy_irect())
    }
}

pub fn get_window_bounds(hwnd: isize) -> Result<IRect, WindowBoundsError> {
    unsafe {
        let mut rect = RECT::default();
        GetWindowRect(HWND(hwnd), &mut rect).map_err(WindowBoundsError::WindowsError)?;
        Ok(rect.to_bevy_irect())
    }
}

pub fn get_window_inner_bounds(hwnd: isize) -> Result<IRect, WindowBoundsError> {
    unsafe {
        let hwnd = HWND(hwnd);
        let mut rect = RECT::default();
        if GetClientRect(hwnd, &mut rect).is_ok() {
            Ok(rect.to_bevy_irect())
        } else {
            Err(WindowBoundsError::WindowsError(
                windows::core::Error::from_win32(),
            ))
        }
    }
}

pub fn begin_dragging(hwnd: isize) -> Result<(), windows::core::Error> {
    unsafe {
        let _join_handle = std::thread::Builder::new()
            .name("Begin move".to_string())
            .spawn(move || {
                if let Err(e) = ReleaseCapture() {
                    eprintln!("Failed to release capture: {:?}", e);
                }
                let hwnd = HWND(hwnd);
                SendMessageW(
                    hwnd,
                    WM_NCLBUTTONDOWN,
                    WPARAM(HTCAPTION as usize),
                    LPARAM(0),
                )
                .0;
                println!("Sent message");
            });
        Ok(())
    }
}

pub fn get_window_title_bar_center_position(hwnd: isize) -> Result<IVec2, WindowBoundsError> {
    unsafe {
        let bounds = get_window_bounds(hwnd)?;

        // SM_CYCAPTION includes the height of the title bar
        let caption_height = GetSystemMetrics(SM_CYCAPTION);

        // SM_CYFRAME includes the height of the window frame (border)
        let frame_height = GetSystemMetrics(SM_CYFRAME);

        let pos =
            bounds.min + IVec2::new((bounds.width()) / 2, (caption_height / 2) + frame_height);
        Ok(pos)
    }
}

pub fn get_window_inner_offset() -> IVec2 {
    unsafe {
        // SM_CYCAPTION includes the height of the title bar
        let caption_height = GetSystemMetrics(SM_CYCAPTION);

        // SM_CYFRAME includes the height of the window frame (border)
        let frame_height = GetSystemMetrics(SM_CYFRAME);

        IVec2::new(frame_height, caption_height + frame_height * 2)
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

pub fn note_window_info(hwnd: isize) -> Result<IRect, WindowBoundsError> {
    unsafe {
        let hwnd = HWND(hwnd);

        // Get the window's size and location
        let mut rect = RECT::default();
        GetWindowRect(hwnd, &mut rect).map_err(WindowBoundsError::WindowsError)?;
        Ok(rect.to_bevy_irect())
    }
}

pub fn is_window_focused(hwnd: HWND) -> bool {
    unsafe {
        // Get the handle to the currently focused (foreground) window.
        let foreground_hwnd = GetForegroundWindow();

        // Compare it with the provided hwnd.
        foreground_hwnd == hwnd
    }
}
