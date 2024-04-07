#![feature(let_chains)]
use anyhow::Result;
use bevy::input::common_conditions::input_toggle_active;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use cursor_hero_memory::primary_window_memory_plugin::PrimaryWindowMemoryPlugin;
use cursor_hero_ui_automation::prelude::*;
use cursor_hero_worker::prelude::Sender;
use cursor_hero_worker::prelude::WorkerConfig;
use cursor_hero_worker::prelude::WorkerMessage;
use cursor_hero_worker::prelude::WorkerPlugin;
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
calculator_example=trace,
cursor_hero_worker=debug,
"
                .replace('\n', "")
                .trim()
                .into(),
            })
            .build(),
    );
    app.add_plugins(WorkerPlugin {
        config: WorkerConfig::<ThreadboundUISnapshotMessage, GameboundUISnapshotMessage> {
            name: "calculator".to_string(),
            is_ui_automation_thread: true,
            handle_threadbound_message: handle_threadbound_message,
            ..default()
        },
    });
    app.add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
    );
    app.add_plugins(PrimaryWindowMemoryPlugin);
    app.insert_resource(ClearColor(Color::rgb(0.992, 0.714, 0.69)));
    app.add_systems(Startup, spawn_camera);
    app.add_systems(
        Update,
        step_1_of_making_the_calculators_in_the_game_match_the_calculator_apps_running_in_the_host,
    );
    app.add_systems(
        Update,
        step_2_of_making_the_calculators_in_the_game_match_the_calculator_apps_running_in_the_host,
    );
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Reflect, Clone, Event)]
enum ThreadboundUISnapshotMessage {
    RequestUISnapshot,
}
impl WorkerMessage for ThreadboundUISnapshotMessage {}

#[derive(Debug, Reflect, Clone, Event)]
enum GameboundUISnapshotMessage {
    UISnapshot(UiSnapshot),
}
impl WorkerMessage for GameboundUISnapshotMessage {}

fn handle_threadbound_message(
    msg: &ThreadboundUISnapshotMessage,
    reply_tx: &Sender<GameboundUISnapshotMessage>,
) -> Result<()> {
    let ThreadboundUISnapshotMessage::RequestUISnapshot = msg;
    debug!("getting state of host calculators");
    let snapshot = take_snapshot()?;
    if let Err(e) = reply_tx.send(GameboundUISnapshotMessage::UISnapshot(snapshot)) {
        error!("Failed to send snapshot: {:?}", e);
    }
    Ok(())
}

fn step_1_of_making_the_calculators_in_the_game_match_the_calculator_apps_running_in_the_host(
    // mut data: ResMut<UIData>,
    mut cooldown: Local<Option<Timer>>,
    time: Res<Time>,
    mut events: EventWriter<ThreadboundUISnapshotMessage>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };
    if window.cursor_position().is_some() {
        return;
    }
    let cooldown_over = if let Some(cooldown) = cooldown.as_mut() {
        if cooldown.tick(time.delta()).just_finished() {
            cooldown.reset();
            true
        } else {
            false
        }
    } else {
        cooldown.replace(Timer::from_seconds(0.5, TimerMode::Repeating));
        true
    };
    if !cooldown_over {
        return;
    }

    // if data.in_flight {
    //     warn!("Too fast!");
    //     return;
    // }

    events.send(ThreadboundUISnapshotMessage::RequestUISnapshot);
    // data.in_flight = true;
}

fn step_2_of_making_the_calculators_in_the_game_match_the_calculator_apps_running_in_the_host(
    mut snapshot: EventReader<GameboundUISnapshotMessage>,
    calculator_query: Query<&Calculator>,
    mut commands: Commands,
) {
    for msg in snapshot.read() {
        let GameboundUISnapshotMessage::UISnapshot(snapshot) = msg;
        // debug!("Received snapshot: {:?}", snapshot);
        for app in snapshot.app_windows.iter() {
            let AppWindow::Calculator(calculator) = app else {
                continue;
            };
            debug!("Received calculator: {:?}", calculator);
        }
    }
}
