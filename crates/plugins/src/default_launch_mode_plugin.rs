use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use cursor_hero_input::active_input_state_plugin::InputMethod;

use bevy::audio::AudioPlugin;
use bevy::audio::SpatialScale;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::log::LogPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::StateInspectorPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use cursor_hero_agent::agent_plugin::AgentPlugin;
use cursor_hero_camera::camera_plugin::CameraPlugin;
use cursor_hero_character::character_plugin::CharacterPlugin;
use cursor_hero_cursor::cursor_plugin::CursorPlugin;
use cursor_hero_cursor_types::cursor_types_plugin::CursorTypesPlugin;
use cursor_hero_environment::environment_plugin::EnvironmentPlugin;
use cursor_hero_environment_nametag::environment_nametag_plugin::EnvironmentNametagPlugin;
use cursor_hero_hover::hover_tool::HoverToolPlugin;
use cursor_hero_hover::hover_ui_automation_plugin::HoverUiAutomationPlugin;
use cursor_hero_hover::screenshot_tool::ScreenshotToolPlugin;
use cursor_hero_icon::IconPlugin;
use cursor_hero_input::InputPlugin;
use cursor_hero_level_bounds::level_bounds_plugin::LevelBoundsPlugin;
use cursor_hero_math::prelude::MathPlugin;
use cursor_hero_memory::primary_window_memory_plugin::restore_window;
use cursor_hero_physics::damping_plugin::DampingPlugin;
use cursor_hero_physics::physics_plugin::PhysicsPlugin;
use cursor_hero_physics_debug::physics_debug_plugin::PhysicsDebugPlugin;
use cursor_hero_pressure_plate::pressure_plate_plugin::PressurePlatePlugin;
use cursor_hero_screen::screen_capture_and_update_plugin::ScreenCaptureAndUpdatePlugin;
use cursor_hero_screen::screen_plugin::ScreenPlugin;
use cursor_hero_sprint_tool::sprint_tool_plugin::SprintToolPlugin;
use cursor_hero_sprint_tool_types::sprint_tool_types_plugin::SprintToolTypesPlugin;
use cursor_hero_taskbar::taskbar_plugin::TaskbarPlugin;
use cursor_hero_toolbelt::toolbelt_plugin::ToolbeltPlugin;
use cursor_hero_toolbelt_types::toolbelt_types_plugin::ToolbeltTypesPlugin;
use cursor_hero_tools::ToolPlugin;
use cursor_hero_ui::about_text_plugin::AboutTextPlugin;
use cursor_hero_ui::fps_text_plugin::FpsTextPlugin;
use cursor_hero_version::version_plugin::Version;
use cursor_hero_wallpaper::wallpaper_plugin::WallpaperPlugin;

use cursor_hero_agent_types::agent_types_plugin::AgentTypesPlugin;
use cursor_hero_app::prelude::*;
use cursor_hero_app_types::prelude::*;
use cursor_hero_brick::prelude::*;
use cursor_hero_brick_types::prelude::*;
use cursor_hero_calculator_app::prelude::*;
use cursor_hero_calculator_app_types::prelude::*;
use cursor_hero_character_types::character_types_plugin::CharacterTypesPlugin;
use cursor_hero_chat::chat_plugin::ChatPlugin;
use cursor_hero_chat_types::chat_types_plugin::ChatTypesPlugin;
use cursor_hero_environment_types::environment_types_plugin::EnvironmentTypesPlugin;
use cursor_hero_explorer_app::prelude::*;
use cursor_hero_explorer_app_types::prelude::*;
use cursor_hero_explorer_tool::prelude::*;
use cursor_hero_explorer_tool_types::prelude::*;
use cursor_hero_floaty_nametag::prelude::*;
use cursor_hero_floaty_nametag_types::prelude::*;
use cursor_hero_fullscreen_tool::prelude::*;
use cursor_hero_fullscreen_tool_types::prelude::*;
use cursor_hero_glados_tts::prelude::*;
use cursor_hero_glados_tts_types::prelude::*;
use cursor_hero_host_event::prelude::*;
use cursor_hero_host_event_types::prelude::*;
use cursor_hero_host_fs::prelude::*;
use cursor_hero_host_fs_types::prelude::*;
use cursor_hero_inference::inference_plugin::InferencePlugin;
use cursor_hero_inference_types::inference_types_plugin::InferenceTypesPlugin;
use cursor_hero_memory::prelude::*;
use cursor_hero_memory_types::prelude::*;
use cursor_hero_movement_tool::movement_tool_plugin::MovementToolPlugin;
use cursor_hero_movement_tool_types::movement_tool_types_plugin::MovementToolTypesPlugin;
use cursor_hero_observation::observation_plugin::ObservationPlugin;
use cursor_hero_observation_types::observation_types_plugin::ObservationTypesPlugin;
use cursor_hero_ollama::prelude::*;
use cursor_hero_ollama_types::prelude::*;
use cursor_hero_secret::prelude::*;
use cursor_hero_secret_types::prelude::*;
use cursor_hero_start_menu::prelude::*;
use cursor_hero_start_menu_types::prelude::*;
use cursor_hero_taskbar_tool::prelude::*;
use cursor_hero_taskbar_types::prelude::TaskbarTypesPlugin;
use cursor_hero_text_asset::prelude::*;
use cursor_hero_text_asset_types::prelude::*;
use cursor_hero_ui_automation::prelude::*;
use cursor_hero_ui_hover::prelude::*;
use cursor_hero_ui_hover_types::prelude::*;
use cursor_hero_ui_inspector::prelude::*;
use cursor_hero_ui_inspector_types::prelude::*;
use cursor_hero_voice_to_text::prelude::*;
use cursor_hero_voice_to_text_types::prelude::*;
use cursor_hero_window_position::prelude::*;
use cursor_hero_window_position_types::prelude::*;
use cursor_hero_window_swap_tool::prelude::*;
use cursor_hero_window_swap_tool_types::prelude::*;
use itertools::Itertools;
pub struct DefaultLaunchModePlugin;

impl Plugin for DefaultLaunchModePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExplorerAppTypesPlugin);
        app.add_plugins(ExplorerAppPlugin);
        app.add_plugins(ExplorerToolPlugin);
        app.add_plugins(ExplorerToolTypesPlugin);
        app.add_plugins(UiHoverTypesPlugin);
        app.add_plugins(UiHoverPlugin);
        app.add_plugins(UIInspectorTypesPlugin);
        app.add_plugins(UiInspectorPlugin);
        app.add_plugins(MemoryTypesPlugin);
        let memory_config = MemoryConfig {
            save_dir: "Cursor Hero Memory".to_string(),
        };
        app.add_plugins(MemoryPlugin {
            config: memory_config.clone(),
            build_config: MemoryPluginBuildConfig::all_enabled(),
        });
        app.add_plugins(WindowSwapToolPlugin);
        app.add_plugins(WindowSwapToolTypesPlugin);
        app.add_plugins(HostFsTypesPlugin);
        app.add_plugins(HostFsPlugin);
        app.add_plugins(WindowPositionTypesPlugin);
        app.add_plugins(WindowPositionPlugin);
        app.add_plugins(UiAutomationTypesPlugin);
        app.add_plugins(UiAutomationPlugin);
        app.add_plugins(BrickTypesPlugin);
        app.add_plugins(BrickPlugin);
        app.add_plugins(FullscreenToolPlugin);
        app.add_plugins(FullscreenToolTypesPlugin);
        app.add_plugins(FloatyNametagTypesPlugin);
        app.add_plugins(FloatyNametagPlugin);
        app.add_plugins(HostEventTypesPlugin);
        app.add_plugins(HostEventPlugin);
        app.add_plugins(TaskbarToolPlugin);
        app.add_plugins(StartMenuTypesPlugin);
        app.add_plugins(StartMenuPlugin);
        app.add_plugins(CalculatorAppTypesPlugin);
        app.add_plugins(CalculatorAppPlugin);
        app.add_plugins(AppTypesPlugin);
        app.add_plugins(AppPlugin);
        app.add_plugins(SecretsTypesPlugin);
        app.add_plugins(SecretsPlugin);
        app.add_plugins(VoiceToTextTypesPlugin);
        app.add_plugins(VoiceToTextPlugin);
        app.add_plugins(GladosTtsTypesPlugin);
        app.add_plugins(GladosTtsPlugin);
        app.add_plugins(OllamaTypesPlugin);
        app.add_plugins(OllamaPlugin);
        app.add_plugins(EnvironmentTypesPlugin);
        app.add_plugins(ChatTypesPlugin);
        app.add_plugins(ChatPlugin);
        app.add_plugins(ObservationTypesPlugin);
        app.add_plugins(ObservationPlugin);
        app.add_plugins(InferenceTypesPlugin);
        app.add_plugins(InferencePlugin);
        app.add_plugins(MovementToolTypesPlugin);
        app.add_plugins(MovementToolPlugin);
        app.add_plugins(CharacterTypesPlugin);
        app.add_plugins(AgentTypesPlugin);
        app.add_plugins(AgentPlugin);
        app.add_plugins(AboutTextPlugin);
        app.add_plugins(CameraPlugin);
        app.add_plugins(CharacterPlugin);
        app.add_plugins(DampingPlugin);
        app.add_plugins(EnvironmentNametagPlugin);
        app.add_plugins(EnvironmentPlugin);
        app.add_plugins(FpsTextPlugin);
        app.add_plugins(HoverToolPlugin);
        app.add_plugins(HoverUiAutomationPlugin);
        app.add_plugins(IconPlugin);
        app.add_plugins(InputPlugin);
        app.add_plugins(ScreenshotToolPlugin);
        app.add_plugins(LevelBoundsPlugin);
        app.add_plugins(MathPlugin);
        app.add_plugins(PhysicsDebugPlugin);
        app.add_plugins(PhysicsPlugin);
        app.add_plugins(CursorPlugin);
        app.add_plugins(CursorTypesPlugin);
        app.add_plugins(PressurePlatePlugin);
        app.add_plugins(ScreenPlugin);
        app.add_plugins(ScreenCaptureAndUpdatePlugin);
        app.add_plugins(SprintToolPlugin);
        app.add_plugins(SprintToolTypesPlugin);
        app.add_plugins(TaskbarPlugin);
        app.add_plugins(ToolbeltPlugin);
        app.add_plugins(ToolbeltTypesPlugin);
        app.add_plugins(ToolPlugin);
        app.add_plugins(WallpaperPlugin);
        // app.add_plugins(UiWatcherTypesPlugin);
        // app.add_plugins(UiWatcherPlugin);

        // must be before the default plugins
        app.add_plugins(EmbeddedAssetPlugin {
            mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
        });

        let log_plugin = LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "
                wgpu=error
                cursor_hero=debug
                cursor_hero_ollama::ollama_status_worker_plugin=info
                cursor_hero_voice_to_text::voice_to_text_ping_plugin=info
                cursor_hero_voice_to_text::voice_to_text_worker_plugin=info
                cursor_hero_glados_tts::glados_tts_status_worker_plugin=info
                cursor_hero_tools::click_tool=info
                cursor_hero_cursor::cursor_hover_plugin=info

                // cursor_hero_memory=info
                // cursor_hero_ui_automation_types=trace
            "
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.starts_with("//"))
            .filter(|line| !line.is_empty())
            .join(",")
            .trim()
            .into(),
        };
        const AUDIO_SCALE: f32 = 1. / 100.0;
        let version = match app.world.get_resource::<Version>() {
            Some(version) => version.0.clone(),
            None => {
                warn!("Version resource not found");
                "Unknown".to_string()
            }
        };
        let mut window = Window {
            title: format!("Cursor Hero v{}", version),
            resizable: true,
            ..default()
        };
        if let Err(e) = restore_window(&memory_config, &mut window) {
            error!("Failed to restore window: {:?}", e);
        }
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AudioPlugin {
                    spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(window),
                    ..default()
                })
                .set(log_plugin)
                .build(),
        );

        // must be after the default plugins (relies on assetserver existing)
        app.add_plugins(TextAssetTypesPlugin);
        app.add_plugins(TextAssetPlugin);
        app.add_plugins(TaskbarTypesPlugin);

        // must be after the default plugins
        app.add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        );
        app.add_plugins(
            StateInspectorPlugin::<InputMethod>::default()
                .run_if(input_toggle_active(false, KeyCode::Grave)),
        );
        app.add_plugins(FrameTimeDiagnosticsPlugin);
    }
}
