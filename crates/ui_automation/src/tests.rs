
#[cfg(test)]
mod tests {
    use crate::prelude::get_taskbar;

    #[test]
    fn test_get_taskbar() {
        let taskbar = get_taskbar().unwrap();
        assert!(taskbar.entries.len() > 0);
        // print the entries
        for entry in taskbar.entries {
            println!("entry: {:?}", entry);
        }
    }

    #[test]
    fn test_gather_apps() {
        // TODO: test to call this 10,000 times to see if we can reproduce the slow down over time issue.
        // UI automation gets slower the longer the computer has gone without restarting.
        let start = std::time::Instant::now();
        let snapshot = super::take_snapshot().unwrap();
        assert!(snapshot.app_windows.len() > 0);
        println!("{}", snapshot);
        let end = std::time::Instant::now();
        println!("time: {:?}", end - start);
        assert!(end - start < std::time::Duration::from_secs(1));
    }
}
