use crate::screen_plugin::Screen;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use cursor_hero_metrics::Metrics;
use cursor_hero_winutils::win_screen_capture::get_full_monitor_capturers;
use cursor_hero_winutils::win_screen_capture::MonitorId;
use cursor_hero_winutils::win_screen_capture::MonitorRegionCapturer;
use cursor_hero_worker::prelude::anyhow::Error;
use cursor_hero_worker::prelude::anyhow::Result;
use cursor_hero_worker::prelude::Sender;
use cursor_hero_worker::prelude::WorkerConfig;
use cursor_hero_worker::prelude::WorkerMessage;
use cursor_hero_worker::prelude::WorkerPlugin;
use cursor_hero_worker::prelude::WorkerState;

pub struct ScreenCaptureAndUpdatePlugin;

impl Plugin for ScreenCaptureAndUpdatePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        let refresh_fps = 10;
        #[cfg(not(debug_assertions))]
        let refresh_fps = 144;

        app.add_plugins(WorkerPlugin {
            config: WorkerConfig::<ThreadboundMessage, GameboundMessage, ThreadState> {
                name: "screen_update_plugin".to_string(),
                threadbound_message_receiver: |thread_rx, _state| {
                    // Continuously capture frames when no messages present
                    match thread_rx.try_recv() {
                        Ok(x) => Ok(x),
                        _ => Ok(ThreadboundMessage::CaptureFrames),
                    }
                },
                handle_threadbound_message,
                sleep_duration: std::time::Duration::from_nanos(1_000_000_000 / refresh_fps),
                ..default()
            },
        });
        app.add_systems(Update, update_screens);
    }
}

#[derive(Debug, Clone)]
struct CapturedFrame {
    data: Vec<u8>,
}

struct ThreadState {
    capturers: Vec<MonitorRegionCapturer>,
    enabled: bool,
}
impl WorkerState for ThreadState {
    fn try_default() -> Result<Self> {
        Ok(ThreadState {
            capturers: get_full_monitor_capturers()?,
            enabled: true,
        })
    }
}

#[derive(Debug, Reflect, Clone, Event)]
enum ThreadboundMessage {
    SetEnabled(bool),
    CaptureFrames,
}
impl WorkerMessage for ThreadboundMessage {}

#[derive(Clone, Event)]
enum GameboundMessage {
    Frames(HashMap<MonitorId, CapturedFrame>),
}
impl WorkerMessage for GameboundMessage {}
impl std::fmt::Debug for GameboundMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameboundMessage::Frames(frames) => {
                write!(f, "GameboundMessage::Frames(len={})", frames.len())
            }
        }
    }
}

fn handle_threadbound_message(
    msg: &ThreadboundMessage,
    reply_tx: &Sender<GameboundMessage>,
    state: &mut ThreadState,
) -> Result<()> {
    match msg {
        ThreadboundMessage::SetEnabled(enabled) => {
            state.enabled = *enabled;
            info!("Screen capture enabled: {}", enabled)
        }
        ThreadboundMessage::CaptureFrames => {
            if !state.enabled {
                return Ok(());
            }
            let frames = state
                .capturers
                .iter_mut()
                .map(|capturer| {
                    // let mut metrics = Metrics::default();
                    // let frame = capturer.capture(&mut Some(metrics)).unwrap();
                    let frame = capturer.capture(&mut None)?;
                    let frame = CapturedFrame {
                        data: frame.to_vec(),
                    };
                    Ok::<(u32, CapturedFrame), Error>((capturer.monitor.info.id, frame))
                })
                .filter_map(Result::ok)
                .collect::<HashMap<u32, CapturedFrame>>();
            reply_tx.send(GameboundMessage::Frames(frames))?;
        }
    }
    Ok(())
}

fn update_screens(
    mut query: Query<(&mut Screen, &Handle<Image>)>,
    mut textures: ResMut<Assets<Image>>,
    mut gamebound_messages: EventReader<GameboundMessage>,
    mut threadbound_messages: EventWriter<ThreadboundMessage>,
    window_query: Query<Entity, With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
    mut sent_disabled: Local<bool>,
) {
    let window_id = window_query.single();
    let Some(winit_window) = winit_windows.get_window(window_id) else {
        error!("Window not found");
        return;
    };
    if winit_window.is_minimized().unwrap_or(false) {
        if !*sent_disabled {
            threadbound_messages.send(ThreadboundMessage::SetEnabled(false));
            *sent_disabled = true;
        }
        return;
    } else if *sent_disabled {
        threadbound_messages.send(ThreadboundMessage::SetEnabled(true));
        *sent_disabled = false;
    }
    let Some(msg) = gamebound_messages.read().last() else {
        return;
    };
    let GameboundMessage::Frames(frames) = msg;
    for screen in &mut query {
        let (screen, texture) = screen;
        // find the frame captured in the other thread
        let mut metrics = Metrics::default();
        metrics.begin("lookup");
        if let Some(frame) = frames.get(&screen.id) {
            // update the texture
            metrics.begin("texture");
            if let Some(t) = textures.get_mut(texture) {
                t.data = frame.data.clone();
            }
            metrics.end("texture");
        }
        metrics.end("lookup");

        // report metrics
        // println!("{}", metrics.report());
    }
}
