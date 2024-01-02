use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::screen_plugin::Screen;
use bevy::prelude::*;
use bevy::utils::HashMap;
use cursor_hero_metrics::Metrics;
use cursor_hero_winutils::win_screen_capture::get_full_monitor_capturers;
use cursor_hero_winutils::win_screen_capture::MonitorRegionCapturer;

pub struct CapturerHolderResource {
    pub capturers: Vec<MonitorRegionCapturer>,
}

// Define a struct to hold captured frames
struct CapturedFrame {
    data: Vec<u8>,
}

// Shared resource for captured frames
#[derive(Resource)]
struct FrameHolderResource {
    frames: Arc<Mutex<HashMap<u32, CapturedFrame>>>,
}

pub struct ScreenUpdatePlugin;
impl Plugin for ScreenUpdatePlugin {
    fn build(&self, app: &mut App) {
        // Create a shared resource for captured frames
        let captured_frames = Arc::new(Mutex::new(HashMap::new()));

        // Clone the Arc to move into the capture thread
        let captured_frames_clone = Arc::clone(&captured_frames);

        let capturer_holder = Arc::new(Mutex::new(CapturerHolderResource {
            capturers: get_full_monitor_capturers().unwrap(),
        }));

        let captured_frames = FrameHolderResource {
            frames: captured_frames,
        };

        // Spawn a separate thread for capturing frames
        let ch = Arc::clone(&capturer_holder);
        thread::spawn(move || {
            loop {
                // Capture logic here...
                let frames = capture_frames(ch.clone());
                let mut shared_frames = captured_frames_clone.lock().unwrap();
                *shared_frames = frames;
            }
        });

        app.add_systems(Update, update_screens)
            .insert_resource(captured_frames)
            .insert_non_send_resource(CapturerHolderResource {
                capturers: get_full_monitor_capturers().unwrap(),
            });
    }
}

fn capture_frames(capturers: Arc<Mutex<CapturerHolderResource>>) -> HashMap<u32, CapturedFrame> {
    capturers
        .lock()
        .unwrap()
        .capturers
        .iter_mut()
        .map(|capturer| {
            // let mut metrics = Metrics::default();
            // let frame = capturer.capture(&mut Some(metrics)).unwrap();
            let frame = capturer.capture(&mut None).unwrap();
            let frame = CapturedFrame {
                data: frame.to_vec(),
            };
            (capturer.monitor.info.id, frame)
        })
        .collect::<HashMap<u32, CapturedFrame>>()
}

fn update_screens(
    mut query: Query<(&mut Screen, &Handle<Image>)>,
    mut textures: ResMut<Assets<Image>>,
    time: Res<Time>,
    frames: Res<FrameHolderResource>,
) {
    let monitor_frames = frames.frames.lock().unwrap();
    for (mut screen, texture) in &mut query {
        // tick the refresh rate timer
        screen.refresh_rate.tick(time.delta());
        // skip if not time to refresh
        if !screen.refresh_rate.finished() {
            continue;
        }

        // find the frame captured in the other thread
        let mut metrics = Metrics::default();
        metrics.begin("lookup");
        let frame = monitor_frames.get(&screen.id).unwrap();
        metrics.end("lookup");

        // update the texture
        metrics.begin("texture");
        textures.get_mut(texture).unwrap().data = frame.data.clone();
        metrics.end("texture");

        // report metrics
        // println!("{}", metrics.report());
    }
}
