use std::rc::Rc;

use bevy::utils::HashSet;
use cursor_hero_winutils::win_errors::*;
use cursor_hero_winutils::win_icons::get_images_from_exe;
use cursor_hero_winutils::win_process::*;
use windows::core::PWSTR;
use windows::Win32::Foundation::E_ACCESSDENIED;

fn main() -> Result<()> {
    unsafe {
        let process_iter = ProcessIterator::new()?;
        let mut done = HashSet::new();
        for mut process in process_iter {
            let exe_name_pwstr = PWSTR(process.szExeFile.as_mut_ptr());
            let exe_name = exe_name_pwstr.to_string()?;
            let exe_path = match get_process_full_name(process.th32ProcessID) {
                Ok(s) => s,
                Err(e) => {
                    if matches!(
                        e,
                        Error::Windows(ref e) if e.code() == E_ACCESSDENIED
                    ) {
                        continue;
                    }
                    eprintln!(
                        "Failed to get full process name for PID {:05} ({}): {:?}",
                        process.th32ProcessID, exe_name, e
                    );
                    continue;
                }
            };
            if done.contains(&exe_path) {
                continue;
            }
            done.insert(exe_path.clone());
            let icons = get_images_from_exe(exe_path.as_str())?;
            println!(
                "Process ID: {:05}, name: {}, icon count: {}",
                process.th32ProcessID,
                exe_name,
                icons.len()
            );

            if icons.is_empty() {
                continue;
            }

            // write the image to target/app_icons/{exe_name}/{icon_index}.png
            let mut path = std::path::PathBuf::from("target/app_icons");
            path.push(exe_name);
            std::fs::create_dir_all(&path).map_err(|e| Error::Other(Rc::new(e)))?;
            for (i, icon) in icons.iter().enumerate() {
                let mut icon_path = path.clone();
                icon_path.push(format!("{}.png", i));
                icon.save(icon_path).map_err(|e| Error::Other(Rc::new(e)))?;
            }
        }
    }
    Ok(())
}
