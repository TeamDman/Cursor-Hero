use crate::gather_root_children::gather_root_children;
use crate::resolve_app::resolve_app;
use cursor_hero_ui_automation_types::prelude::*;
use itertools::Itertools;
use uiautomation::UIAutomation;

pub fn take_snapshot() -> Result<UISnapshot, GatherAppsError> {
    let automation = UIAutomation::new()?;
    let walker = automation.create_tree_walker()?;
    // let walker = automation.get_raw_view_walker()?;
    let top_level_children = gather_root_children(&automation, &walker)?;

    let focused = automation.get_focused_element()?;
    let focused_app = walker.normalize(&focused)?;


    let mut apps = vec![];
    let mut errors = vec![];
    for elem in top_level_children {
        let focused = elem.get_runtime_id() == focused_app.get_runtime_id();
        match resolve_app(&elem, &automation, focused) {
            Ok(app) => {
                apps.push((elem, app));
            }
            Err(e) => errors.push(e),
        }
    }
    let bad_errors = errors
        .into_iter()
        .filter(|e| !matches!(e, AppResolveError::NoMatch))
        .collect_vec();
    if !bad_errors.is_empty() {
        return Err(GatherAppsError::ResolveFailed(bad_errors));
    }

    let snapshot = UISnapshot {
        app_windows: apps.into_iter().map(|(_elem, app)| app).collect(),
    };
    Ok(snapshot)
}

#[cfg(test)]
mod tests {
    use windows::Win32::System::Com::CoInitializeEx;
    use windows::Win32::System::Com::COINIT_MULTITHREADED;

    use crate::prelude::take_snapshot;

    #[test]
    fn test_take_snapshot() {
        //todo: put this in thread initialzed for COM like below
        let snapshot = take_snapshot().unwrap();
        assert!(snapshot.app_windows.len() > 0);
    }

    #[test]
    fn test_take_snapshot_many() {
        let handle = std::thread::spawn(move || -> windows::core::Result<()> {
            unsafe {
                // Initialize COM in MTA mode
                // https://learn.microsoft.com/en-us/windows/win32/com/multithreaded-apartments
                CoInitializeEx(None, COINIT_MULTITHREADED)?;

                println!("COM initialized in MTA mode.");

                for i in 0..100 {
                    let start = std::time::Instant::now();
                    let snapshot = take_snapshot().unwrap();
                    assert!(snapshot.app_windows.len() > 0);
                    if i == 0 {
                        println!("{}", snapshot);
                    }
                    let end = std::time::Instant::now();
                    println!("time: {:?}", end - start);
                    assert!(end - start < std::time::Duration::from_secs(1));
                }

                Ok(())
            }
        });

        // Wait for the thread to complete its tasks and handle any errors
        let _ = handle.join().unwrap();
    }
}
