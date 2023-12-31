use bevy::audio::AudioPlugin;
use bevy::audio::SpatialScale;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::math::Vector;
use bevy_xpbd_2d::plugins::PhysicsPlugins;
use bevy_xpbd_2d::resources::Gravity;
use cursor_hero_plugins::MyPlugin;

const AUDIO_SCALE: f32 = 1. / 100.0;

fn main() {
    use bevy::log::LogPlugin;

    // this code is compiled only if debug assertions are enabled (debug mode)
    #[cfg(debug_assertions)]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_ecs=info,cursor_hero=debug".into(),
        // filter: "debug,wgpu_core=warn,wgpu_hal=warn,bevy_ecs=info,cursor_hero=debug".into(),
    };

    // this code is compiled only if debug assertions are disabled (release mode)
    #[cfg(not(debug_assertions))]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::INFO,
        filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
    };

    let mut app = App::new();
    app.add_plugins(EmbeddedAssetPlugin {
        mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
    })
    .add_plugins(
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
                        // %BEGIN_RESOLUTION%
                        834.0,
                        797.0
                        // %END_RESOLUTION%
                    )
                        .into(),
                    resizable: true,
                    position: WindowPosition::At(
                        (
                            // %BEGIN_POSITION%
                        903,
                        105
                        // %END_POSITION%
                        )
                            .into(),
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
