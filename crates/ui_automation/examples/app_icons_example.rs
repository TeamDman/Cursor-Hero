use cursor_hero_winutils::widestring::U16CString;
use cursor_hero_winutils::win_errors::Error;
use cursor_hero_winutils::win_errors::Result;
use image::ImageBuffer;
use image::RgbaImage;
use itertools::Itertools;
use windows::core::PCWSTR;
use windows::core::PWSTR;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Foundation::HANDLE;
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
use windows::Win32::System::Diagnostics::ToolHelp::CreateToolhelp32Snapshot;
use windows::Win32::System::Diagnostics::ToolHelp::Process32FirstW;
use windows::Win32::System::Diagnostics::ToolHelp::Process32NextW;
use windows::Win32::System::Diagnostics::ToolHelp::PROCESSENTRY32W;
use windows::Win32::System::Diagnostics::ToolHelp::TH32CS_SNAPPROCESS;
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::QueryFullProcessImageNameW;
use windows::Win32::System::Threading::PROCESS_NAME_FORMAT;
use windows::Win32::System::Threading::PROCESS_QUERY_INFORMATION;
use windows::Win32::UI::Shell::ExtractIconExW;
use windows::Win32::UI::WindowsAndMessaging::DestroyIcon;
use windows::Win32::UI::WindowsAndMessaging::DrawIconEx;
use windows::Win32::UI::WindowsAndMessaging::GetIconInfoExW;
use windows::Win32::UI::WindowsAndMessaging::DI_IMAGE;
use windows::Win32::UI::WindowsAndMessaging::DI_MASK;
use windows::Win32::UI::WindowsAndMessaging::DI_NORMAL;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::Win32::UI::WindowsAndMessaging::ICONINFOEXW;

struct ProcessIterator {
    snapshot: HANDLE,
    process: PROCESSENTRY32W,
    first_done: bool,
}

impl ProcessIterator {
    fn new() -> Result<Self> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
            Ok(ProcessIterator {
                snapshot,
                process: PROCESSENTRY32W::default(),
                first_done: false,
            })
        }
    }
}

impl Iterator for ProcessIterator {
    type Item = PROCESSENTRY32W;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.process.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
            if !self.first_done {
                self.first_done = true;
                return match Process32FirstW(self.snapshot, &mut self.process) {
                    Ok(()) => Some(self.process),
                    Err(e) => {
                        eprintln!("Failed to get first process: {:?}", e);
                        None
                    }
                };
            } else {
                return match Process32NextW(self.snapshot, &mut self.process) {
                    Ok(()) => Some(self.process),
                    Err(e) => {
                        if e.message() == "There are no more files." {
                            return None;
                        }
                        eprintln!("Failed to get next process: {:?}", e);
                        None
                    }
                };
            }
        }
    }
}

impl Drop for ProcessIterator {
    fn drop(&mut self) {
        unsafe {
            if let Err(e) = CloseHandle(self.snapshot) {
                eprintln!("Failed to close snapshot: {:?}", e);
            }
        };
    }
}

fn get_process_full_name(process_id: u32) -> Result<String> {
    unsafe {
        let process_handle: HANDLE = OpenProcess(PROCESS_QUERY_INFORMATION, false, process_id)?;
        let result = (|| {
            if process_handle.is_invalid() {
                eprintln!("Failed to open process handle");
                return Err(windows::core::Error::from_win32());
            }

            let mut buffer: Vec<u16> = Vec::with_capacity(512);
            let mut buffer_size = buffer.capacity() as u32;
            let full_name_pwstr = PWSTR(buffer.as_mut_ptr());

            QueryFullProcessImageNameW(
                process_handle,
                PROCESS_NAME_FORMAT(0),
                full_name_pwstr,
                &mut buffer_size,
            )?;
            buffer.set_len(buffer_size as usize);

            Ok(String::from_utf16_lossy(&buffer))
        })();
        if let Err(e) = CloseHandle(process_handle) {
            eprintln!("Failed to close process handle: {:?}", e);
        }
        Ok(result?)
    }
}

fn get_images_for_process(executable_path: &str) -> Result<Vec<RgbaImage>> {
    unsafe {
        let path_cstr = U16CString::from_str(executable_path)?;
        let path_pcwstr = PCWSTR(path_cstr.as_ptr());
        let num_icons_total = ExtractIconExW(path_pcwstr, -1, None, None, 0);
        println!("num_icons_total: {}", num_icons_total);
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

        println!(
            "got: {}, large={:?}, small={:?}",
            num_icons_total, large_icons, small_icons
        );

        if num_icons_total == 0 {
            return Ok(Vec::new()); // No icons extracted
        }

        let images = large_icons
            .iter()
            .chain(small_icons.iter())
            .map(|icon| convert_hicon_to_rgba_image(icon))
            .filter_map(|r| match r {
                Ok(img) => Some(img),
                Err(e) => {
                    eprintln!("Failed to convert HICON to RgbaImage: {:?}", e);
                    None
                }
            })
            .collect_vec();

        // Convert HICONs to RgbaImage
        // This will involve creating a compatible bitmap, drawing the icon on it, and then reading the pixel data
        // For simplicity, this is left as a placeholder
        // images.push(convert_hicon_to_rgba_image(hicon)?);

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

fn convert_hicon_to_rgba_image(hicon: &HICON) -> Result<RgbaImage> {
    unsafe {
        let mut icon_info = ICONINFOEXW::default();
        GetIconInfoExW(*hicon, &mut icon_info).ok()?;
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
        )?;

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

fn main() -> Result<()> {
    unsafe {
        let process_iter = ProcessIterator::new()?;

        for mut process in process_iter {
            let exe_name_pwstr = PWSTR(process.szExeFile.as_mut_ptr());
            let exe_name = exe_name_pwstr.to_string()?;
            if exe_name != "msedge.exe" {
                continue;
            }
            let exe_path = match get_process_full_name(process.th32ProcessID) {
                Ok(s) => s,
                Err(e) => {
                    // if e.code() == E_ACCESSDENIED {
                    //     eprintln!(
                    //         "Access denied for process: {} ({})",
                    //         process.th32ProcessID, exe_name
                    //     );
                    //     continue;
                    // }
                    eprintln!(
                        "Failed to get full process name for PID {:05} ({}): {:?}",
                        process.th32ProcessID, exe_name, e
                    );
                    continue;
                }
            };

            let icons = get_images_for_process(exe_path.as_str())?;
            println!("Icons for {}: {:?}", exe_path, icons.len());

            println!(
                "Process ID: {:05}, Full Name: {}",
                process.th32ProcessID, exe_path
            );
            break;
        }
    }
    Ok(())
}
