use bevy::input::common_conditions::input_toggle_active;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use cursor_hero_camera::camera_plugin::CameraPlugin;
use cursor_hero_memory::prelude::MemoryPlugin;
use cursor_hero_memory::primary_window_memory_plugin::restore_window;
use cursor_hero_memory_types::prelude::MemoryConfig;
use cursor_hero_memory_types::prelude::MemoryPluginBuildConfig;
use cursor_hero_memory_types::prelude::MemoryTypesPlugin;
use cursor_hero_ui_automation::prelude::UiAutomationPlugin;
use cursor_hero_ui_automation::prelude::UiAutomationTypesPlugin;
use cursor_hero_version::version_plugin::Version;
pub struct InspectLaunchModePlugin;

impl Plugin for InspectLaunchModePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MemoryTypesPlugin);

        let memory_config = MemoryConfig {
            save_dir: "Cursor Hero Memory (inspect mode)".to_string(),
        };
        app.add_plugins(MemoryPlugin {
            config: memory_config.clone(),
            build_config: MemoryPluginBuildConfig {
                primary_window_memory_enabled: true,
                ..default()
            },
        });
        app.add_plugins(UiAutomationTypesPlugin);
        app.add_plugins(UiAutomationPlugin);
        app.add_plugins(CameraPlugin);

        let version = match app.world.get_resource::<Version>() {
            Some(version) => version.0.clone(),
            None => {
                warn!("Version resource not found");
                "Unknown".to_string()
            }
        };
        let mut window = Window {
            title: format!("Cursor Hero Inspector v{}", version),
            resizable: true,
            ..default()
        };
        if let Err(e) = restore_window(&memory_config, &mut window) {
            error!("Failed to restore window: {:?}", e);
        }
        app.add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: bevy::log::Level::DEBUG,
                    filter: "
info,
wgpu_core=warn,
wgpu_hal=warn,
ui_hover_example=trace,
cursor_hero_worker=debug,
"
                    .replace('\n', "")
                    .trim()
                    .into(),
                })
                .set(WindowPlugin {
                    primary_window: Some(window),
                    ..default()
                })
                .build(),
        );
        // app.add_plugins(WorkerPlugin {
        //     config: WorkerConfig::<ThreadboundUISnapshotMessage, GameboundUISnapshotMessage> {
        //         name: "ui_hover".to_string(),
        //         is_ui_automation_thread: true,
        //         handle_threadbound_message: handle_threadbound_message,
        //         handle_threadbound_message_error_handler: handle_threadbound_message_error_handler,
        //         ..default()
        //     },
        // });
        app.add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        );
        app.insert_resource(ClearColor(Color::rgb(0.992, 0.714, 0.69)));
    }
}
