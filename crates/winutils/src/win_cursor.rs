use crate::win_errors::*;
use crate::win_icons::convert_hcursor_to_rgba_image;
use image::RgbaImage;
use windows::Win32::UI::WindowsAndMessaging::LoadCursorW;
use windows::Win32::UI::WindowsAndMessaging::IDC_APPSTARTING;
use windows::Win32::UI::WindowsAndMessaging::IDC_ARROW;
use windows::Win32::UI::WindowsAndMessaging::IDC_CROSS;
use windows::Win32::UI::WindowsAndMessaging::IDC_HAND;
use windows::Win32::UI::WindowsAndMessaging::IDC_HELP;
use windows::Win32::UI::WindowsAndMessaging::IDC_IBEAM;
use windows::Win32::UI::WindowsAndMessaging::IDC_NO;
use windows::Win32::UI::WindowsAndMessaging::IDC_SIZEALL;
use windows::Win32::UI::WindowsAndMessaging::IDC_SIZENESW;
use windows::Win32::UI::WindowsAndMessaging::IDC_SIZENS;
use windows::Win32::UI::WindowsAndMessaging::IDC_SIZENWSE;
use windows::Win32::UI::WindowsAndMessaging::IDC_SIZEWE;
use windows::Win32::UI::WindowsAndMessaging::IDC_UPARROW;
use windows::Win32::UI::WindowsAndMessaging::IDC_WAIT;

pub fn get_all_cursor_icons() -> Result<Vec<RgbaImage>> {
    let mut icons = Vec::new();

    // Load each cursor and convert it to an RgbaImage
    for cursor_id in [
        IDC_ARROW,
        IDC_IBEAM,
        IDC_WAIT,
        IDC_CROSS,
        IDC_UPARROW,
        IDC_SIZEALL,
        IDC_SIZENESW,
        IDC_SIZENS,
        IDC_SIZENWSE,
        IDC_SIZEWE,
        IDC_HAND,
        IDC_HELP,
        IDC_NO,
        IDC_APPSTARTING,
    ] {
        let hcursor = unsafe { LoadCursorW(None, cursor_id)? };
        if hcursor.is_invalid() {
            return Err(Error::from_win32()
                .with_description(format!("Failed to load cursor with ID {:?}", cursor_id.0)));
        }
        let image = convert_hcursor_to_rgba_image(&hcursor)?;
        icons.push(image);
    }

    Ok(icons)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_get_all_cursor_icons() {
        let icons = super::get_all_cursor_icons().unwrap();

        // Ensure the expected amount is present
        assert_eq!(icons.len(), 14);

        // Save icons
        let mut path = PathBuf::from("target/cursor_icons");
        std::fs::create_dir_all(&path).unwrap();
        for (i, icon) in icons.iter().enumerate() {
            let mut icon_path = path.clone();
            icon_path.push(format!("{}.png", i));
            icon.save(icon_path).unwrap();
        }
    }
}
