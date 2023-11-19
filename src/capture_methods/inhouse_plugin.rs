use std::{
    sync::{Arc, Mutex},
    thread, borrow::BorrowMut,
};

use crate::{
    capture_methods::inhouse::{
        get_all_monitors, get_full_monitor_capturers, get_monitor_capturer, MonitorRegionCapturer,
    },
    screens::{InhouseCaptureTag, Screen},
    metrics::Metrics,
};
use bevy::prelude::*;
use windows::Win32::Foundation::RECT;

pub struct CapturerHolderResource {
    pub capturers: Vec<MonitorRegionCapturer>,
}

// Define a struct to hold captured frames
struct CapturedFrame {
    data: Vec<u8>,
    // other fields as necessary
}

// Shared resource for captured frames
#[derive(Resource)]
struct FrameHolderResource {
    frames: Arc<Mutex<Vec<CapturedFrame>>>,
}

pub struct InhouseCapturePlugin;
impl Plugin for InhouseCapturePlugin {
    fn build(&self, app: &mut App) {
        // Create a shared resource for captured frames
        let captured_frames = Arc::new(Mutex::new(Vec::new()));

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

        app.add_systems(
            Update,
            (update_screens, resize_capture_areas.before(update_screens)),
            // update_screens,
        )
        .insert_non_send_resource(captured_frames)
        .insert_non_send_resource(CapturerHolderResource {
            capturers: get_full_monitor_capturers().unwrap(),
        });
    }
}

fn capture_frames(capturers: Arc<Mutex<CapturerHolderResource>>) -> Vec<CapturedFrame> {
    
    capturers.lock().unwrap()
        .capturers
        .iter_mut()
        .map(|capturer| {
            let mut metrics = Metrics::default();
            let frame = capturer.capture(&mut metrics).unwrap();
            CapturedFrame {
                data: frame.to_vec(),
            }
        })
        .collect::<Vec<CapturedFrame>>()
}

fn update_screens(
    mut query: Query<(&mut Screen, &Handle<Image>), With<InhouseCaptureTag>>,
    mut textures: ResMut<Assets<Image>>,
    time: Res<Time>,
    mut capturer_resource: NonSendMut<CapturerHolderResource>,
) {
    for (mut screen, texture) in &mut query {
        // tick the refresh rate timer
        screen.refresh_rate.tick(time.delta());
        // skip if not time to refresh
        if !screen.refresh_rate.finished() {
            continue;
        }

        let start = std::time::Instant::now();

        // find the capturer for this screen
        let capturer = capturer_resource
            .capturers
            .iter_mut()
            .find(|capturer| capturer.monitor.info.name == screen.name);
        if capturer.is_none() {
            println!("No capturer found for screen {}", screen.name);
            continue;
        }

        let mut metrics = Metrics::default();

        // capture the screen
        metrics.begin("capture");
        let frame = capturer.unwrap().capture(&mut metrics).unwrap();
        metrics.end("capture");

        // update the texture
        metrics.begin("texture");
        textures.get_mut(texture).unwrap().data = frame.to_vec();
        metrics.end("texture");

        // report metrics
        println!("{}", metrics.report());
    }
}

fn resize_capture_areas(
    mut res: NonSendMut<CapturerHolderResource>,
    keyboard_input: Res<Input<KeyCode>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if keyboard_input.pressed(KeyCode::R) {
        // println!("Resizing capture areas");
        // Get camera and window information
        let (camera, camera_transform) = q_camera.single();

        // Convert the corners of the viewport to world coordinates
        let bottom_left_world = camera
            .viewport_to_world(camera_transform, Vec2::new(0.0, 0.0))
            .map(|ray| ray.origin.truncate())
            .unwrap_or_default();

        let top_right_world = camera
            .viewport_to_world(camera_transform, Vec2::new(1.0, 1.0))
            .map(|ray| ray.origin.truncate())
            .unwrap_or_default();

        println!(
            "Resizing capture areas to {:?} {:?}",
            bottom_left_world, top_right_world
        );

        // Get monitors and prepare for new capturers
        let monitors = get_all_monitors().unwrap();
        let mut capturers = Vec::new();

        for monitor in monitors {
            let monitor_rect = monitor.info.rect.clone();

            // Compute the intersection between the visible world coordinates and the monitor's rectangle
            let capture_region =
                compute_capture_region(monitor_rect, bottom_left_world, top_right_world);
            if capture_region.is_none() {
                continue;
            }
            let capturer = get_monitor_capturer(Arc::new(monitor), capture_region.unwrap());
            capturers.push(capturer);
        }
        if capturers.len() == 0 {
            eprintln!("No capturers exist after resize, aborting");
            return;
        }
        res.capturers = capturers;
    }
}

fn compute_capture_region(
    monitor_rect: RECT,
    bottom_left_world: Vec2,
    top_right_world: Vec2,
) -> Option<RECT> {
    // Convert Vec2 to i32 for comparison
    let bl_x = bottom_left_world.x as i32;
    let bl_y = bottom_left_world.y as i32;
    let tr_x = top_right_world.x as i32;
    let tr_y = top_right_world.y as i32;

    // Calculate the overlapping region
    let overlap_left = std::cmp::max(monitor_rect.left, bl_x);
    let overlap_right = std::cmp::min(monitor_rect.right, tr_x);
    let overlap_top = std::cmp::max(monitor_rect.top, bl_y);
    let overlap_bottom = std::cmp::min(monitor_rect.bottom, tr_y);

    // Check if the regions actually overlap
    if overlap_left < overlap_right && overlap_top < overlap_bottom {
        Some(RECT {
            left: overlap_left,
            top: overlap_top,
            right: overlap_right,
            bottom: overlap_bottom,
        })
    } else {
        None
    }
}
