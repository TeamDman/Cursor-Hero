use crate::win_errors::*;
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

pub struct ProcessIterator {
    snapshot: HANDLE,
    process: PROCESSENTRY32W,
    first_done: bool,
}

impl ProcessIterator {
    pub fn new() -> Result<Self> {
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
                match Process32FirstW(self.snapshot, &mut self.process) {
                    Ok(()) => Some(self.process),
                    Err(e) => {
                        eprintln!("Failed to get first process: {:?}", e);
                        None
                    }
                }
            } else {
                match Process32NextW(self.snapshot, &mut self.process) {
                    Ok(()) => Some(self.process),
                    Err(e) => {
                        if e.message() == "There are no more files." {
                            return None;
                        }
                        eprintln!("Failed to get next process: {:?}", e);
                        None
                    }
                }
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

pub fn get_process_full_name(process_id: u32) -> Result<String> {
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
