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
        icon_info.cbSize = std::mem::size_of::<ICONINFOEXW>() as u32; // thank you valve https://github.com/ValveSoftware/wine/blob/941279cf95abce8c59ad350e6345734c9a75f0f2/dlls/winemac.drv/mouse.c#L775
        GetIconInfoExW(*hicon, &mut icon_info).ok_with_description(format!(
            "icon_info := hicon • GetIconInfoExW: {} {}:{}",
            file!(),
            line!(),
            column!()
        ))?;
        if icon_info.hbmColor.is_invalid() {
            return Err(Error::from_win32().with_description(format!(
                "icon • GetIconInfoExW: {} {}:{}",
                file!(),
                line!(),
                column!()
            )));
        }

        let mut bmp = BITMAP::default();
        let bytes_stored = GetObjectW(
            icon_info.hbmColor,
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bmp as *mut _ as _),
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
        let hbitmap = CreateCompatibleBitmap(hdc_screen, bmp.bmWidth, bmp.bmHeight);
        let hbm_old = SelectObject(hdc_mem, hbitmap);

        // Draw the icon onto the memory device context
        DrawIconEx(
            hdc_mem,
            0,
            0,
            *hicon,
            bmp.bmWidth,
            bmp.bmHeight,
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

        let mut bitmap_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: bmp.bmWidth,
                biHeight: bmp.bmHeight,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: DIB_RGB_COLORS.0,
                ..Default::default()
            },
            bmiColors: [RGBQUAD::default(); 1],
        };

        // Allocate a buffer and get the bitmap bits
        let mut buffer = vec![0u8; bmp.bmWidth as usize * bmp.bmHeight as usize * 4];
        if GetDIBits(
            hdc_mem,
            hbitmap,
            0,
            bmp.bmHeight as u32,
            Some(buffer.as_mut_ptr() as *mut _),
            &mut bitmap_info,
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
        DeleteObject(hbitmap).ok()?;
        DeleteDC(hdc_mem).ok()?;
        DeleteDC(hdc_screen).ok()?;
        DeleteObject(icon_info.hbmColor).ok()?;
        DeleteObject(icon_info.hbmMask).ok()?;

        // Create an image from the buffer
        ImageBuffer::from_raw(bmp.bmWidth as u32, bmp.bmHeight as u32, buffer)
            .map(RgbaImage::from)
            .ok_or_else(|| Error::ImageContainerNotBigEnough)
    }
}
