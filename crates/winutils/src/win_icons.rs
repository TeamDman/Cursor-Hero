use crate::win_errors::*;
use image::ImageBuffer;
use image::RgbaImage;
use windows::Win32::Graphics::Gdi::CreateCompatibleBitmap;
use windows::Win32::Graphics::Gdi::CreateCompatibleDC;
use windows::Win32::Graphics::Gdi::DeleteDC;
use windows::Win32::Graphics::Gdi::DeleteObject;
use windows::Win32::Graphics::Gdi::GetDIBits;
use windows::Win32::Graphics::Gdi::GetObjectW;
use windows::Win32::Graphics::Gdi::SelectObject;
use windows::Win32::Graphics::Gdi::BITMAP;
use windows::Win32::Graphics::Gdi::BITMAPINFO;
use windows::Win32::Graphics::Gdi::BITMAPINFOHEADER;
use windows::Win32::Graphics::Gdi::DIB_RGB_COLORS;
use windows::Win32::Graphics::Gdi::RGBQUAD;
use windows::Win32::UI::WindowsAndMessaging::DrawIconEx;
use windows::Win32::UI::WindowsAndMessaging::GetIconInfoExW;
use windows::Win32::UI::WindowsAndMessaging::DI_NORMAL;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::Win32::UI::WindowsAndMessaging::ICONINFOEXW;

pub fn convert_hicon_to_rgba_image(hicon: &HICON) -> Result<RgbaImage> {
    unsafe {
        let mut icon_info = ICONINFOEXW::default();

        // thank you valve https://github.com/ValveSoftware/wine/blob/941279cf95abce8c59ad350e6345734c9a75f0f2/dlls/winemac.drv/mouse.c#L775
        icon_info.cbSize = std::mem::size_of::<ICONINFOEXW>() as u32;

        GetIconInfoExW(*hicon, &mut icon_info).ok_with_description(format!(
            "icon_info := hicon • GetIconInfoExW: {} {}:{}",
            file!(),
            line!(),
            column!()
        ))?;
        if icon_info.hbmColor.is_invalid() || icon_info.hbmMask.is_invalid() {
            return Err(Error::from_win32().with_description(format!(
                "icon • GetIconInfoExW: {} {}:{}",
                file!(),
                line!(),
                column!()
            )));
        }

        // Create the colour bitmap
        let mut color_bmp = BITMAP::default();
        let bytes_stored = GetObjectW(
            icon_info.hbmColor,
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut color_bmp as *mut _ as _),
        );
        if bytes_stored == 0 {
            return Err(Error::from_win32().with_description(format!(
                "icon_info::hbmColor • GetObjectW: {} {}:{}",
                file!(),
                line!(),
                column!()
            )));
        }

        // Create a compatible device context
        let hdc_screen = CreateCompatibleDC(None);
        let hdc_mem = CreateCompatibleDC(hdc_screen);

        // Create a compatible bitmap
        let hbitmap = CreateCompatibleBitmap(hdc_screen, color_bmp.bmWidth, color_bmp.bmHeight);
        let hbm_old = SelectObject(hdc_mem, hbitmap);

        // Draw the icon onto the memory device context
        DrawIconEx(
            hdc_mem,
            0,
            0,
            *hicon,
            color_bmp.bmWidth,
            color_bmp.bmHeight,
            0,
            None,
            DI_NORMAL,
        )
        .with_description(format!(
            "hdc_mem, hicon • DrawIconEx: {} {}:{}",
            file!(),
            line!(),
            column!()
        ))?;

        let mut color_bitmap_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: color_bmp.bmWidth,
                biHeight: -color_bmp.bmHeight,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: DIB_RGB_COLORS.0,
                ..Default::default()
            },
            bmiColors: [RGBQUAD::default(); 1],
        };

        // Allocate a buffer and get the bitmap bits
        let mut color_buffer =
            vec![0u8; color_bmp.bmWidth as usize * color_bmp.bmHeight as usize * 4];
        if GetDIBits(
            hdc_mem,
            hbitmap,
            0,
            color_bmp.bmHeight as u32,
            Some(color_buffer.as_mut_ptr() as *mut _),
            &mut color_bitmap_info,
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

        
        // let mut mask_bmp = BITMAP::default();
        // let bytes_stored = GetObjectW(
        //     icon_info.hbmMask,
        //     std::mem::size_of::<BITMAP>() as i32,
        //     Some(&mut mask_bmp as *mut _ as _),
        // );
        // if bytes_stored == 0 {
        //     return Err(Error::from_win32().with_description(format!(
        //         "icon_info::hbmMask • GetObjectW: {} {}:{}",
        //         file!(),
        //         line!(),
        //         column!()
        //     )));
        // }
        
        // let mut mask_bitmap_info = BITMAPINFO {
        //     bmiHeader: BITMAPINFOHEADER {
        //         biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
        //         biWidth: mask_bmp.bmWidth,
        //         biHeight: mask_bmp.bmHeight,
        //         biPlanes: 1,
        //         biBitCount: 32,
        //         biCompression: DIB_RGB_COLORS.0,
        //         ..Default::default()
        //     },
        //     bmiColors: [RGBQUAD::default(); 1],
        // };
        // let mut mask_buffer = vec![0u8; mask_bmp.bmWidth as usize * mask_bmp.bmHeight as usize * 1];
        // if GetDIBits(
        //     hdc_mem,
        //     hbitmap,
        //     0,
        //     mask_bmp.bmHeight as u32,
        //     Some(mask_buffer.as_mut_ptr() as *mut _),
        //     &mut mask_bitmap_info,
        //     DIB_RGB_COLORS,
        // ) == 0
        // {
        //     return Err(Error::from_win32().with_description(format!(
        //         "GetDIBits: {} {}:{}",
        //         file!(),
        //         line!(),
        //         column!()
        //     )));
        // }

        // Combine the color and mask bitmaps
        let width = color_bmp.bmWidth as usize;
        let height = color_bmp.bmHeight as usize;

        let combined_buffer = color_buffer;
        // let mut combined_buffer: Vec<u8> = Vec::with_capacity(width * height * 4);
        // for y in 0..height {
        //     for x in 0..width {
        //         let color_index = (y * width + x) * 4;
        //         let mask_index = y * width + x; // Assuming mask bitmap is 1 byte per pixel

        //         let alpha = !mask_buffer[mask_index]; // Inverting the mask value since white is opaque and black is transparent

        //         combined_buffer.push(color_buffer[color_index]); // Red
        //         combined_buffer.push(color_buffer[color_index + 1]); // Green
        //         combined_buffer.push(color_buffer[color_index + 2]); // Blue
        //         combined_buffer.push(alpha); // Alpha
        //     }
        // }

        // Create an image from the buffer
        // This might be able to be done after cleanup since buffer is populated by then
        let image = ImageBuffer::from_raw(width as u32, height as u32, combined_buffer)
            .map(RgbaImage::from)
            .ok_or_else(|| Error::ImageContainerNotBigEnough);

        // Clean up
        SelectObject(hdc_mem, hbm_old);
        DeleteObject(hbitmap).ok()?;
        DeleteDC(hdc_mem).ok()?;
        DeleteDC(hdc_screen).ok()?;
        DeleteObject(icon_info.hbmColor).ok()?;
        DeleteObject(icon_info.hbmMask).ok()?;

        image
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::IVec4;

    use crate::win_process::get_images_for_process;

    use std::path::PathBuf;

    #[test]
    fn test_convert_hicon_to_rgba_image() {
        let exe_path = r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe";
        let icons = get_images_for_process(exe_path).unwrap();

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
