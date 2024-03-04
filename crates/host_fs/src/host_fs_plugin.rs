use bevy::prelude::*;
use cursor_hero_host_fs_types::host_fs_types::HostPathAction;

pub struct HostFsPlugin;

impl Plugin for HostFsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_host_path_events);
    }
}

fn handle_host_path_events(mut host_path_events: EventReader<HostPathAction>) {
    for event in host_path_events.read() {
        match event {
            HostPathAction::OpenWithCode { path } => {
                println!("Open with code: {:?}", path);
                let Some(x) = path.path.to_str() else {
                    continue;
                };
                if let Err(e) = std::process::Command::new("code.cmd").arg(x).spawn() {
                    error!("Failed to open with code: {:?}", e);
                }
            }
        }
    }
}
