use bevy::{prelude::*, window::PrimaryWindow};
use image::DynamicImage;
use rayon::prelude::*;
use screenshots::Screen as ScreenLib;
use windows::Win32::Foundation::RECT;
use std::{collections::VecDeque, rc::Rc};

pub struct ScreenBackgroundsPlugin;
use crate::windows_screen_capturing::{get_full_monitor_capturers, MonitorRegionCapturer, get_all_monitors, get_monitor_capturer};


impl Plugin for ScreenBackgroundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_screens)
            .add_systems(Update, (update_screens, cycle_capture_method, resize_capture_areas.before(update_screens)))
            .insert_non_send_resource(CapturerResource {
                inhouse_capturers: get_full_monitor_capturers().unwrap(),
            });
    }
}

pub struct CapturerResource {
    pub inhouse_capturers: Vec<MonitorRegionCapturer>,
}

#[derive(Debug, Clone, Copy, Default, Reflect)]
pub enum CaptureMethod {
    #[default]
    Inhouse,
    Screen,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Screen {
    id: u32,
    name: String,
    refresh_rate: Timer,
    capture_method: CaptureMethod,
}

#[derive(Component)]
pub struct ScreenParent;

fn spawn_screens(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    // mut capturer_resource: NonSendMut<CapturerResource>,
) {
    commands.spawn((
        SpatialBundle::default(),
        ScreenParent,
        Name::new("Screen Parent"),
    ));

    // create a Screen component for each screen
    let mut screen_names = get_all_monitors().unwrap().iter().map(|monitor| monitor.info.name.clone()).collect::<VecDeque<String>>();
    for screen in ScreenLib::all().unwrap().iter() {
        let image_buf = screen.capture().unwrap();
        let dynamic_image = DynamicImage::ImageRgba8(image_buf);
        let image = Image::from_dynamic(dynamic_image, true);
        let texture = textures.add(image);

        // // Assuming the index aligns with the capturer's expected screen index
        // let capturer = Capturer::new_with_timeout(index, Duration::from_millis(2000))
        // println!("Creating capturer for screen {}", index);
        // let capturer = Capturer::new(index)
        //     .expect(format!("Failed to create capturer for screen {}", index).as_str());
        let name = screen_names.pop_front().unwrap();

        // capturer_resource
        //     .capturers
        //     .insert(screen.display_info.id, capturer);

        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_xyz(
                    screen.display_info.x as f32,
                    screen.display_info.y as f32,
                    -1.0,
                ), // Position behind the character
                ..Default::default()
            },
            Screen {
                name,
                id: screen.display_info.id,
                refresh_rate: Timer::from_seconds(1.0, TimerMode::Repeating),
                capture_method: default(),
            },
            Name::new(format!("Screen {}", screen.display_info.id)),
        ));
    }
}

fn update_screens(
    mut query: Query<(&mut Screen, &Handle<Image>)>,
    mut textures: ResMut<Assets<Image>>,
    time: Res<Time>,
    mut capturer_resource: NonSendMut<CapturerResource>,
) {
    // Cache the screens
    let all_screens = ScreenLib::all().unwrap();

    // Filter and collect the screens you're interested in, you can parallelize this part
    let relevant_screens: Vec<_> = all_screens
        .par_iter()
        .filter(|&libscreen| {
            query
                .iter()
                .any(|(screen, _)| libscreen.display_info.id == screen.id)
        })
        .collect();

    for (mut screen, texture) in &mut query {
        screen.refresh_rate.tick(time.delta());
        if screen.refresh_rate.finished() {
            // Only consider the screens that were filtered before
            for libscreen in relevant_screens.iter() {
                if libscreen.display_info.id == screen.id {
                    match screen.capture_method {
                        CaptureMethod::Screen => {
                            let start = std::time::Instant::now();
                            let image_buf = libscreen.capture().unwrap();
                            println!("capture took {:?}", start.elapsed());

                            let dynamic_image = DynamicImage::ImageRgba8(image_buf);
                            let image = Image::from_dynamic(dynamic_image, true);
                            textures.get_mut(&texture).unwrap().data = image.data;
                        }
                        CaptureMethod::Inhouse => {
                            let start = std::time::Instant::now();
                            let capturer = capturer_resource
                                .inhouse_capturers
                                .iter_mut()
                                .find(|capturer| capturer.monitor.info.name == screen.name);
                            if capturer.is_none() {
                                println!("No capturer found for screen {}", screen.name);
                                continue;
                            }
                            let frame = capturer.unwrap().capture().unwrap();
                            println!(" | total screen update took {:?}", start.elapsed());
                            
                            // let dynamic_image = DynamicImage::ImageRgba8(frame);
                            // let image = Image::from_dynamic(dynamic_image, true);
                            // textures.get_mut(&texture).unwrap().data = image.data;
                            textures.get_mut(&texture).unwrap().data = frame.to_vec();
                        }
                    }
                }
            }
        }
    }
}

fn cycle_capture_method(mut query: Query<&mut Screen>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::M) {
        for mut screen in query.iter_mut() {
            screen.capture_method = match screen.capture_method {
                CaptureMethod::Screen => {
                    CaptureMethod::Inhouse
                }
                CaptureMethod::Inhouse => {
                    CaptureMethod::Screen
                }
            };
            println!("Switched to {:?} method", screen.capture_method);

        }
    }
}

fn resize_capture_areas(
    mut res: NonSendMut<CapturerResource>,
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

        println!("Resizing capture areas to {:?} {:?}", bottom_left_world, top_right_world);

        // Get monitors and prepare for new capturers
        let monitors = get_all_monitors().unwrap();
        let mut capturers = Vec::new();

        for monitor in monitors {
            let monitor_rect = monitor.info.rect.clone();

            // Compute the intersection between the visible world coordinates and the monitor's rectangle
            let capture_region = compute_capture_region(
                monitor_rect,
                bottom_left_world,
                top_right_world,
            );
            if capture_region.is_none() {
                continue;
            }
            let capturer = get_monitor_capturer(Rc::new(monitor), capture_region.unwrap());
            capturers.push(capturer);
        }
        if capturers.len() == 0 {
            eprintln!("No capturers exist after resize, aborting");
            return;
        }
        res.inhouse_capturers = capturers;
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