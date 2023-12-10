use bevy::audio::{AudioPlugin, SpatialScale};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::math::Vector;
use bevy_xpbd_2d::plugins::PhysicsPlugins;
use bevy_xpbd_2d::resources::Gravity;
use crate::utils::win_screen_capture::{get_monitor_infos, MonitorInfo};

use crate::plugins::MyPlugin;

mod data;
mod utils;
mod plugins;

const AUDIO_SCALE: f32 = 1. / 100.0;

fn main() {
    let left_monitor = get_monitor_infos()
        .expect("Monitor info not found")
        .into_iter()
        .fold(None, |acc: Option<MonitorInfo>, elem| {
            if let Some(acc) = acc {
                if elem.rect.left < acc.rect.left {
                    Some(elem)
                } else {
                    Some(acc)
                }
            } else {
                Some(elem)
            }
        })
        .expect("Left monitor not found");

    use bevy::log::LogPlugin;

    // this code is compiled only if debug assertions are enabled (debug mode)
    #[cfg(debug_assertions)]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_ecs=info,cursor_hero=debug".into(),
    };

    // this code is compiled only if debug assertions are disabled (release mode)
    #[cfg(not(debug_assertions))]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::INFO,
        filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
    };

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(AudioPlugin {
                spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Cursor Hero".into(),
                    resolution: (
                        (left_monitor.work_area.right - left_monitor.work_area.left - 100) as f32,
                        (left_monitor.work_area.bottom - left_monitor.work_area.top - 100) as f32,
                    )
                        .into(),
                    resizable: true,
                    position: WindowPosition::At(
                        (left_monitor.rect.left, left_monitor.rect.top + 10).into(),
                    ),
                    ..default()
                }),
                ..default()
            })
            .set(log_plugin)
            .build(),
    )
    .add_plugins(PhysicsPlugins::default())
    .insert_resource(Gravity(Vector::ZERO))
    .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)))
    .add_plugins((FrameTimeDiagnosticsPlugin,))
    .add_plugins(MyPlugin);
    app.run();
}
