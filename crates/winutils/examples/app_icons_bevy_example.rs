use bevy::input::common_conditions::input_toggle_active;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use cursor_hero_winutils::win_errors::*;
use cursor_hero_winutils::win_icons::get_images_from_exe;
use cursor_hero_winutils::win_process::*;
use cursor_hero_worker::prelude::*;
use image::DynamicImage;
use image::RgbaImage;
use windows::core::PWSTR;
use windows::Win32::Foundation::E_ACCESSDENIED;
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
cursor_hero=debug,
app_icons_bevy_example=trace,
"
                .replace('\n', "")
                .trim()
                .into(),
            })
            .build(),
    );
    app.add_plugins(WorkerPlugin {
        config: WorkerConfig::<ThreadboundMessage, GameboundMessage> {
            name: "ui_snapshot".to_string(),
            is_ui_automation_thread: true,
            handle_threadbound_message: |msg, reply_tx| {
                handle_threadbound_message(msg, reply_tx).map_err(|e| Box::new(e) as _)
            },
            ..default()
        },
    });
    app.add_systems(Update, receive);
    app.add_systems(Startup, trigger);
    app.add_systems(Startup, camera_setup);
    app.add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
    );
    app.run();
}

#[derive(Debug, Reflect, Clone, Event)]
enum ThreadboundMessage {
    GatherRunningProcessIcons,
}
impl Message for ThreadboundMessage {}

#[derive(Debug, Clone, Event)]
enum GameboundMessage {
    RunningProcessIcons(HashMap<String, Vec<RgbaImage>>),
}
impl Message for GameboundMessage {}

fn handle_threadbound_message(
    msg: &ThreadboundMessage,
    reply_tx: &Sender<GameboundMessage>,
) -> Result<()> {
    let ThreadboundMessage::GatherRunningProcessIcons = msg;
    let process_iter = ProcessIterator::new()?;
    let mut result = HashMap::new();
    unsafe {
        for mut process in process_iter {
            let exe_name_pwstr = PWSTR(process.szExeFile.as_mut_ptr());
            let exe_name = exe_name_pwstr.to_string()?;
            let exe_path = match get_process_full_name(process.th32ProcessID) {
                Ok(s) => s,
                Err(e) => {
                    if matches!(
                        e,
                        Error::Windows(ref e) if e.code() == E_ACCESSDENIED
                    ) {
                        continue;
                    }
                    warn!(
                        "Failed to get full process name for PID {:05} ({}): {:?}",
                        process.th32ProcessID, exe_name, e
                    );
                    continue;
                }
            };
            if result.contains_key(&exe_path) {
                continue;
            }
            let icons = get_images_from_exe(exe_path.as_str())?;
            result.insert(exe_path, icons);
        }
    }
    if let Err(e) = reply_tx.send(GameboundMessage::RunningProcessIcons(result)) {
        error!("Failed to send snapshot: {:?}", e);
    }

    Ok(())
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // commands.spawn((
    //     SpriteBundle {
    //         transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    //         sprite: Sprite {
    //             color: Color::WHITE,
    //             custom_size: Some(Vec2::new(100.0, 100.0)),
    //             ..default()
    //         },
    //         ..default()
    //     },
    // ));
}

fn trigger(mut events: EventWriter<ThreadboundMessage>) {
    events.send(ThreadboundMessage::GatherRunningProcessIcons);
}

fn receive(
    mut commands: Commands,
    mut bridge: EventReader<GameboundMessage>,
    mut icons_so_far: Local<usize>,
    mut textures: ResMut<Assets<Image>>,
) {
    for msg in bridge.read() {
        match msg {
            GameboundMessage::RunningProcessIcons(icons) => {
                info!("Received icons: {:?}", icons.len());
                for (exe_path, images) in icons {
                    for image in images {
                        debug!("{}x{}", image.width(), image.height());
                        let dynamic = DynamicImage::ImageRgba8(image.clone());
                        let handle = textures.add(Image::from_dynamic(dynamic, true));
                        let icons_per_row = 5;
                        let icon_size = 100.0;
                        let margin = 10.0;
                        commands.spawn((
                            SpriteBundle {
                                texture: handle,
                                transform: Transform::from_translation(Vec3::new(
                                    (*icons_so_far % icons_per_row) as f32 * (icon_size + margin),
                                    (*icons_so_far / icons_per_row) as f32 * (icon_size + margin),
                                    0.0,
                                )),
                                sprite: Sprite {
                                    color: Color::hsl(
                                        *icons_so_far as f32 / icons.len() as f32 * 360.0,
                                        1.0,
                                        0.5,
                                    ),
                                    custom_size: Some(Vec2::splat(icon_size)),
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new(exe_path.clone()),
                        ));
                        *icons_so_far += 1;
                    }
                }
            }
        }
    }
}
