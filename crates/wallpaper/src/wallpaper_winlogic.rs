use std::ffi::OsString;
use std::os::raw::c_void;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::UI::WindowsAndMessaging::FindWindowExA;
use windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW;
use windows::Win32::UI::WindowsAndMessaging::SPIF_UPDATEINIFILE;
use windows::Win32::UI::WindowsAndMessaging::SPI_GETDESKWALLPAPER;

fn get_wallpaper_path() -> Result<OsString, windows::core::Error> {
    let mut buffer = vec![0u16; 32768]; // theoretical max path after increase
    unsafe {
        SystemParametersInfoW(
            SPI_GETDESKWALLPAPER,
            buffer.len() as u32,
            Some(buffer.as_mut_ptr() as *mut c_void),
            SPIF_UPDATEINIFILE,
        )
    }?;

    // Find the position of the first null character
    let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());

    // Create a slice of the buffer up to the found position
    let trimmed_buffer = &buffer[..len];

    Ok(OsString::from_wide(trimmed_buffer))
}

use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::keybd_event;
use windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_LWIN;
use windows::Win32::UI::WindowsAndMessaging::FindWindowA;
use windows::Win32::UI::WindowsAndMessaging::ShowWindow;
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOW;

fn unobstructed_desktop() {
    // Minimize all windows
    show_desktop();

    // Hide all taskbars
    hide_taskbars();

    // Hide desktop icons
    let desktop_icons_handle = find_desktop_icons_window();
    if let Some(hwnd) = desktop_icons_handle {
        unsafe { ShowWindow(hwnd, SW_HIDE) };
    }

    // Wait for a while to view the desktop
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Restore desktop icons
    if let Some(hwnd) = desktop_icons_handle {
        unsafe { ShowWindow(hwnd, SW_SHOW) };
    }

    // Restore taskbars
    show_taskbars();

    // Restore all windows
    show_desktop();
}

fn show_desktop() {
    unsafe {
        keybd_event(VK_LWIN.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(0x44, 0, KEYBD_EVENT_FLAGS(0), 0); // 'D' key
        keybd_event(VK_LWIN.0 as u8, 0, KEYBD_EVENT_FLAGS(2), 0); // KEYEVENTF_KEYUP
        keybd_event(0x44, 0, KEYBD_EVENT_FLAGS(2), 0); // 'D' key up
    }
}

fn hide_taskbars() {
    hide_window("Shell_TrayWnd");
    hide_window("Shell_SecondaryTrayWnd");
}

fn show_taskbars() {
    show_window("Shell_TrayWnd");
    show_window("Shell_SecondaryTrayWnd");
}

fn hide_window(class_name: &str) {
    unsafe {
        let hwnd = FindWindowA(PCSTR(class_name.as_ptr()), PCSTR::null());
        if hwnd.0 != 0 {
            ShowWindow(hwnd, SW_HIDE);
        }
    }
}

fn show_window(class_name: &str) {
    unsafe {
        let hwnd = FindWindowA(PCSTR(class_name.as_ptr()), PCSTR::null());
        if hwnd.0 != 0 {
            ShowWindow(hwnd, SW_SHOW);
        }
    }
}

fn find_desktop_icons_window() -> Option<HWND> {
    let progman = unsafe { FindWindowA(PCSTR("Progman".as_ptr()), PCSTR::null()) };
    let shell_dll_def_view = unsafe {
        FindWindowExA(
            progman,
            HWND::default(),
            PCSTR("SHELLDLL_DefView".as_ptr()),
            PCSTR::null(),
        )
    };
    let hwnd = unsafe {
        FindWindowExA(
            shell_dll_def_view,
            HWND::default(),
            PCSTR("SysListView32".as_ptr()),
            PCSTR::null(),
        )
    };

    if hwnd.0 == 0 {
        None
    } else {
        Some(hwnd)
    }
}

fn find_taskbar_window() -> Option<HWND> {
    let taskbar_class = "Shell_TrayWnd";
    unsafe {
        let hwnd = FindWindowA(PCSTR(taskbar_class.as_ptr()), PCSTR::null());
        if hwnd.0 == 0 {
            None
        } else {
            Some(hwnd)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_wallpaper_path_test() {
        match get_wallpaper_path() {
            Ok(path) => println!("Wallpaper path: {:?}", path),
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    #[test]
    fn unobstructed_desktop_test() {
        unobstructed_desktop();
    }

    #[test]
    fn hide_taskbars_test() {
        hide_taskbars();
        std::thread::sleep(std::time::Duration::from_secs(3));
        show_taskbars();
    }

    #[test]
    fn hide_taskbar_test() {
        // Hide the taskbar
        let taskbar_handle = find_taskbar_window();
        if let Some(hwnd) = taskbar_handle {
            unsafe { ShowWindow(hwnd, SW_HIDE) };
        } else {
            panic!("Taskbar not found");
        }

        // Wait for a while to view the desktop
        std::thread::sleep(std::time::Duration::from_secs(5));

        // Restore the taskbar
        if let Some(hwnd) = taskbar_handle {
            unsafe { ShowWindow(hwnd, SW_SHOW) };
        }
    }
}
