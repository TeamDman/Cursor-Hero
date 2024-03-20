use crate::win_errors::*;
use cursor_hero_math::prelude::bgra_to_rgba;
use image::ImageBuffer;
use image::RgbaImage;
use itertools::Itertools;
use widestring::U16CString;
use windows::core::PCWSTR;
use windows::Win32::Graphics::Gdi::CreateCompatibleDC;
use windows::Win32::Graphics::Gdi::DeleteDC;
use windows::Win32::Graphics::Gdi::DeleteObject;
use windows::Win32::Graphics::Gdi::GetDIBits;
use windows::Win32::Graphics::Gdi::SelectObject;
use windows::Win32::Graphics::Gdi::BITMAPINFO;
use windows::Win32::Graphics::Gdi::BITMAPINFOHEADER;
use windows::Win32::Graphics::Gdi::DIB_RGB_COLORS;
use windows::Win32::UI::Shell::ExtractIconExW;
use windows::Win32::UI::WindowsAndMessaging::DestroyIcon;
use windows::Win32::UI::WindowsAndMessaging::GetIconInfoExW;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::Win32::UI::WindowsAndMessaging::ICONINFOEXW;

pub fn get_images_from_exe(executable_path: &str) -> Result<Vec<RgbaImage>> {
    unsafe {
        let path_cstr = U16CString::from_str(executable_path)?;
        let path_pcwstr = PCWSTR(path_cstr.as_ptr());
        let num_icons_total = ExtractIconExW(path_pcwstr, -1, None, None, 0);
        if num_icons_total == 0 {
            return Ok(Vec::new()); // No icons extracted
        }

        let mut large_icons = vec![HICON::default(); num_icons_total as usize];
        let mut small_icons = vec![HICON::default(); num_icons_total as usize];
        let num_icons_fetched = ExtractIconExW(
            path_pcwstr,
            0,
            Some(large_icons.as_mut_ptr()),
            Some(small_icons.as_mut_ptr()),
            num_icons_total,
        );

        if num_icons_fetched == 0 {
            return Ok(Vec::new()); // No icons extracted
        }

        let images = large_icons
            .iter()
            .chain(small_icons.iter())
            // .map(|icon| convert_hicon_to_rgba_image(icon))
            .map(|icon| convert_hicon_to_rgba_image(icon))
            .filter_map(|r| match r {
                Ok(img) => Some(img),
                Err(e) => {
                    eprintln!("Failed to convert HICON to RgbaImage: {:?}", e);
                    None
                }
            })
            .collect_vec();

        large_icons
            .iter()
            .chain(small_icons.iter())
            .filter(|icon| !icon.is_invalid())
            .map(|icon| DestroyIcon(*icon))
            .filter_map(|r| r.err())
            .for_each(|e| eprintln!("Failed to destroy icon: {:?}", e));

        Ok(images)
    }
}

// https://stackoverflow.com/a/23390460/11141271 -- How to extract 128x128 icon bitmap data from EXE in python
// https://stackoverflow.com/a/22885412/11141271 -- Save HICON as a png (Java reference)
pub fn convert_hicon_to_rgba_image(hicon: &HICON) -> Result<RgbaImage> {
    unsafe {
        let mut icon_info = ICONINFOEXW::default();
        icon_info.cbSize = std::mem::size_of::<ICONINFOEXW>() as u32;

        if !GetIconInfoExW(*hicon, &mut icon_info).as_bool() {
            return Err(Error::from_win32().with_description(format!(
                "icon • GetIconInfoExW: {} {}:{}",
                file!(),
                line!(),
                column!()
            )));
        }
        let hdc_screen = CreateCompatibleDC(None);
        let hdc_mem = CreateCompatibleDC(hdc_screen);
        let hbm_old = SelectObject(hdc_mem, icon_info.hbmColor);

        let mut bmp_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: icon_info.xHotspot as i32 * 2,
                // https://discord.com/channels/1071140253181673492/1219071553929740328/1219359761389064273
                // Rafael: ImageBuffer::from_raw expects top-down bitmap, so your biHeight in the BITMAPINFOHEADER should be negative.
                biHeight: -(icon_info.yHotspot as i32 * 2),
                biPlanes: 1,
                biBitCount: 32,
                biCompression: DIB_RGB_COLORS.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut buffer: Vec<u8> =
            vec![0; (icon_info.xHotspot * 2 * icon_info.yHotspot * 2 * 4) as usize];

        if GetDIBits(
            hdc_mem,
            icon_info.hbmColor,
            0,
            icon_info.yHotspot * 2,
            Some(buffer.as_mut_ptr() as *mut _),
            &mut bmp_info,
            DIB_RGB_COLORS,
        ) == 0
        {
            return Err(Error::from_win32().with_description(format!(
                "GetDIBits: {} {}:{}",
                file!(),
                line!(),
                column!()
            )));
        }
        // Clean up
        SelectObject(hdc_mem, hbm_old);
        DeleteDC(hdc_mem);
        DeleteDC(hdc_screen);
        DeleteObject(icon_info.hbmColor);
        DeleteObject(icon_info.hbmMask);

        bgra_to_rgba(buffer.as_mut_slice());

        let image = ImageBuffer::from_raw(icon_info.xHotspot * 2, icon_info.yHotspot * 2, buffer)
            .ok_or_else(|| Error::ImageContainerNotBigEnough)?;
        return Ok(image);
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::IVec4;
    use std::path::PathBuf;

    #[test]
    fn test_convert_hicon_to_rgba_image() {
        let exe_path = r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe";
        let icons = super::get_images_from_exe(exe_path).unwrap();

        // Ensure the expected amount is present
        assert_eq!(icons.len(), 30);

        // Save icons
        let mut path = PathBuf::from("target/app_icons");
        path.push("msedge.exe");
        std::fs::create_dir_all(&path).unwrap();
        for (i, icon) in icons.iter().enumerate() {
            let mut icon_path = path.clone();
            icon_path.push(format!("{}.png", i));
            icon.save(icon_path).unwrap();
        }

        // Assert all icons are more than just transparent images
        // Also count rgb totals
        let mut passed = vec![false; icons.len()];
        for (i, icon) in icons.iter().enumerate() {
            let mut rgb_count = IVec4::ZERO;
            for pixel in icon.pixels() {
                let pixel = IVec4::new(
                    pixel[0] as i32,
                    pixel[1] as i32,
                    pixel[2] as i32,
                    pixel[3] as i32,
                );
                rgb_count += pixel;
                // if pixel[3] != 0 {
                //     non_transparent_pixel_present = true;
                //     break;
                // }
            }
            if rgb_count != IVec4::ZERO {
                passed[i] = true;
            }
        }
        println!("{:?}", passed);
        assert!(passed.iter().all(|&x| x));
    }
}
