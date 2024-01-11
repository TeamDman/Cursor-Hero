use bevy::audio::AudioPlugin;
use bevy::audio::SpatialScale;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::math::Vector;
use bevy_xpbd_2d::plugins::setup::Physics;
use bevy_xpbd_2d::plugins::PhysicsPlugins;
use bevy_xpbd_2d::resources::Gravity;
use cursor_hero_plugins::MyPlugin;
use cursor_hero_version::version_plugin::VersionPlugin;

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
    app.add_plugins(VersionPlugin(env!("CARGO_PKG_VERSION").to_string()));
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
                    transparent: true,
                    title: format!("Cursor Hero v{}", env!("CARGO_PKG_VERSION")),
                    #[cfg(debug_assertions)]
                    resolution: (
                        // %BEGIN_RESOLUTION%
                        1129.0,
                        728.0
                        // %END_RESOLUTION%
                    )
                        .into(),
                    resizable: true,
                    #[cfg(debug_assertions)]
                    position: WindowPosition::At(
                        (
                            // %BEGIN_POSITION%
                        55,
                        52
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
    .insert_resource(Time::new_with(Physics::fixed_hz(144.0)))
    .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)))
    // .insert_resource(ClearColor(Color::NONE))
    .add_plugins((FrameTimeDiagnosticsPlugin,))
    .add_plugins(MyPlugin);
    app.run();
}
