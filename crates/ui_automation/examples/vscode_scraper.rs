use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::ExitCondition;
use cursor_hero_memory_types::prelude::get_persist_file;
use cursor_hero_memory_types::prelude::Usage;
use cursor_hero_ui_automation::prelude::*;
use std::io::Write;
fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(LogPlugin {
                level: bevy::log::Level::DEBUG,
                filter: "
info,
wgpu_core=warn,
wgpu_hal=warn,
vscode_scraper=trace
"
                .replace('\n', "")
                .trim()
                .into(),
            })
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                ..default()
            })
            .build(),
    );
    app.add_systems(Update, write_vscode_ui_info);
    app.run();
}

fn write_vscode_ui_info(mut cooldown: Local<Option<Timer>>, time: Res<Time>) {
    let should_tick = if let Some(cooldown) = cooldown.as_mut() {
        if cooldown.tick(time.delta()).just_finished() {
            cooldown.reset();
            true
        } else {
            false
        }
    } else {
        cooldown.replace(Timer::from_seconds(1.0, TimerMode::Repeating));
        true
    };
    if !should_tick {
        return;
    }
    debug!("taking snapshot");
    println!("!!!!!!!!!!!!!!! taking snapshot !!!!!!!!!!!!!!!");
    let snapshot = take_snapshot().unwrap();
    match get_persist_file(file!(), "vscode.txt", Usage::Persist) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(snapshot.to_string().as_bytes()) {
                debug!("Failed to write to file: {:?}", e);
            }
        }
        Err(e) => {
            error!("Failed to open file: {:?}", e);
        }
    }
}
