use std::io::Write;

use bevy::app::App;
use bevy::log::LogPlugin;
use cursor_hero_memory_types::prelude::get_persist_file;
use cursor_hero_memory_types::prelude::Usage;
use cursor_hero_ui_automation::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.add_plugins(LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "
info,
wgpu_core=warn,
wgpu_hal=warn,
bevy_ecs=info,
cursor_hero=debug,
        "
        .replace('\n', "")
        .trim()
        .into(),
    });
    app.add_systems(Update, write_vscode_ui_info);
    app.run();
}

fn write_vscode_ui_info(
    mut cooldown: Local<Cooldown>,
) {
    let snapshot = take_snapshot()?;
    // println!("{}", snapshot);

    match get_persist_file(file!(), "vscode.txt", Usage::Persist) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(snapshot.to_string().as_bytes()) {
                eprintln!("Failed to write to file: {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to open file: {:?}", e);
        }
    }
}
