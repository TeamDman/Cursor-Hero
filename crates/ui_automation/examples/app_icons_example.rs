use cursor_hero_winutils::widestring::U16CString;
use cursor_hero_winutils::win_errors::Result;
use image::RgbaImage;
use windows::core::PCWSTR;
use windows::core::PWSTR;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Foundation::HANDLE;
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
use windows::Win32::UI::WindowsAndMessaging::HICON;

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
        let mut large_icons: Vec<HICON> = Vec::new();
        let mut small_icons: Vec<HICON> = Vec::new();

        let path_cstr = U16CString::from_str(executable_path)?;
        let path_pcwstr = PCWSTR(path_cstr.as_ptr());
        let total_icons = ExtractIconExW(
            path_pcwstr,
            0,
            Some(large_icons.as_mut_ptr()),
            Some(small_icons.as_mut_ptr()),
            1, // You might want to adjust this based on how many icons you expect to extract
        );

        println!("large={:?}, small={:?}", large_icons, small_icons);

        if total_icons == 0 {
            return Ok(Vec::new()); // No icons extracted
        }

        let mut images = Vec::new();

        // Assuming we are working with the first large icon for demonstration
        if !large_icons.is_empty() {
            let hicon = large_icons[0];
            // Convert HICON to RgbaImage
            // This will involve creating a compatible bitmap, drawing the icon on it, and then reading the pixel data
            // For simplicity, this is left as a placeholder
            // images.push(convert_hicon_to_rgba_image(hicon)?);

            // Remember to free the icon when done
            if let Err(e) = DestroyIcon(hicon) {
                eprintln!("Failed to destroy icon: {:?}", e);
            }
        }

        Ok(images)
    }
}

fn main() -> Result<()> {
    unsafe {
        let process_iter = ProcessIterator::new()?;

        for mut process in process_iter {
            
            let exe_name_pwstr = PWSTR(process.szExeFile.as_mut_ptr());
            let exe_name = exe_name_pwstr.to_string()?;
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

            // let icons = get_images_for_process(exe_path.as_str())?;
            // println!("Icons for {}: {:?}", exe_path, icons.len());

            println!(
                "Process ID: {:05}, Full Name: {}",
                process.th32ProcessID, exe_path
            );
            // Here, you would also retrieve the image path for the process if available via other API calls
        }
    }
    Ok(())
}
