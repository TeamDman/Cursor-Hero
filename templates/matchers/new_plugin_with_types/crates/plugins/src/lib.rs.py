# {{use_statements}}
# use cursor_hero_{{crate_name}}::prelude::*;
# use cursor_hero_{{crate_name}}_types::prelude::*;
# {{plugin_start}}
#         app.add_plugins({{crate_name_pascal}}TypesPlugin);
#         app.add_plugins({{crate_name_pascal}}Plugin);
# {{plugin_remaining}}

from typing import Tuple

def chunk(text: str) -> Tuple[str, str, str]:
    # TODO: Implement logic for plugin_remaining
    # TODO: Implement logic for plugin_start
    # TODO: Implement logic for use_statements
    return ()

##### WORKSPACE CONTENT
#use bevy::input::common_conditions::input_toggle_active;
#use bevy::prelude::*;
#
#use bevy::audio::AudioPlugin;
#use bevy::audio::SpatialScale;
#use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
#use bevy::log::LogPlugin;
#use bevy_embedded_assets::EmbeddedAssetPlugin;
#use bevy_inspector_egui::quick::WorldInspectorPlugin;
#use cursor_hero_agent::agent_plugin::AgentPlugin;
#use cursor_hero_camera::camera_plugin::CameraPlugin;
#use cursor_hero_character::character_plugin::CharacterPlugin;
#use cursor_hero_cursor_mirror::cursor_mirroring_plugin::CursorMirroringPlugin;
#use cursor_hero_environment::environment_plugin::EnvironmentPlugin;
#use cursor_hero_environment_nametag::environment_nametag_plugin::EnvironmentNametagPlugin;
#use cursor_hero_hover::afterimage_plugin::AfterimagePlugin;
#use cursor_hero_hover::hover_tool::HoverToolPlugin;
#use cursor_hero_hover::hover_ui_automation_plugin::HoverUiAutomationPlugin;
#use cursor_hero_hover::inspect_tool::InspectToolPlugin;
#use cursor_hero_hover::inspect_wheel_tool::InspectWheelToolPlugin;
#use cursor_hero_icon::IconPlugin;
#use cursor_hero_input::InputPlugin;
#use cursor_hero_level_bounds::level_bounds_plugin::LevelBoundsPlugin;
#use cursor_hero_math::math_plugin::MathPlugin;
#use cursor_hero_pause_tool::pause_tool_plugin::PauseToolPlugin;
#use cursor_hero_physics::damping_plugin::DampingPlugin;
#use cursor_hero_physics::physics_plugin::PhysicsPlugin;
#use cursor_hero_physics_debug::physics_debug_plugin::PhysicsDebugPlugin;
#use cursor_hero_pointer::pointer_plugin::PointerPlugin;
#use cursor_hero_pointer_types::pointer_types_plugin::PointerTypesPlugin;
#use cursor_hero_pressure_plate::pressure_plate_plugin::PressurePlatePlugin;
#use cursor_hero_restart_memory::MemoryPlugin;
##[cfg(debug_assertions)]
#use cursor_hero_screen::screen_plugin::ScreenPlugin;
#use cursor_hero_screen::screen_update_plugin::ScreenUpdatePlugin;
#use cursor_hero_sprint_tool::sprint_tool_plugin::SprintToolPlugin;
#use cursor_hero_sprint_tool_types::sprint_tool_types_plugin::SprintToolTypesPlugin;
#use cursor_hero_taskbar::taskbar_plugin::TaskbarPlugin;
#use cursor_hero_toolbelt::toolbelt_plugin::ToolbeltPlugin;
#use cursor_hero_toolbelt_types::toolbelt_types_plugin::ToolbeltTypesPlugin;
#use cursor_hero_tools::ToolPlugin;
#use cursor_hero_ui::about_text_plugin::AboutTextPlugin;
#use cursor_hero_ui::fps_text_plugin::FpsTextPlugin;
#use cursor_hero_version::version_plugin::Version;
#use cursor_hero_wallpaper::wallpaper_plugin::WallpaperPlugin;
#
#use cursor_hero_agent_types::agent_types_plugin::AgentTypesPlugin;
#use cursor_hero_character_types::character_types_plugin::CharacterTypesPlugin;
#use cursor_hero_chat::chat_plugin::ChatPlugin;
#use cursor_hero_chat_types::chat_types_plugin::ChatTypesPlugin;
#use cursor_hero_inference::inference_plugin::InferencePlugin;
#use cursor_hero_inference_types::inference_types_plugin::InferenceTypesPlugin;
#use cursor_hero_movement_tool::movement_tool_plugin::MovementToolPlugin;
#use cursor_hero_movement_tool_types::movement_tool_types_plugin::MovementToolTypesPlugin;
#use cursor_hero_observation::observation_plugin::ObservationPlugin;
#use cursor_hero_observation_types::observation_types_plugin::ObservationTypesPlugin;
#use cursor_hero_tts::tts_plugin::TtsPlugin;
#use cursor_hero_tts_types::tts_types_plugin::TtsTypesPlugin;
#use cursor_hero_environment_types::environment_types_plugin::EnvironmentTypesPlugin;
#pub struct MyPlugin;
#
#impl Plugin for MyPlugin {
#    fn build(&self, app: &mut App) {
#        app.add_plugins(EnvironmentTypesPlugin);
#        app.add_plugins(ChatTypesPlugin);
#        app.add_plugins(ChatPlugin);
#        app.add_plugins(TtsPlugin);
#        app.add_plugins(TtsTypesPlugin);
#        app.add_plugins(ObservationTypesPlugin);
#        app.add_plugins(ObservationPlugin);
#        app.add_plugins(InferenceTypesPlugin);
#        app.add_plugins(InferencePlugin);
#        app.add_plugins(MovementToolTypesPlugin);
#        app.add_plugins(MovementToolPlugin);
#        app.add_plugins(CharacterTypesPlugin);
#        app.add_plugins(AgentTypesPlugin);
#        app.add_plugins(AgentPlugin);
#        //app.add_plugins(ClickDragMovementPlugin);
#        //app.add_plugins(HoverShowerRelayPlugin);
#        //app.add_plugins(HoverShowerServicePlugin);
#        app.add_plugins(AboutTextPlugin);
#        app.add_plugins(AfterimagePlugin);
#        app.add_plugins(CameraPlugin);
#        app.add_plugins(CharacterPlugin);
#        app.add_plugins(CursorMirroringPlugin);
#        app.add_plugins(DampingPlugin);
#        app.add_plugins(EnvironmentNametagPlugin);
#        app.add_plugins(EnvironmentPlugin);
#        app.add_plugins(FpsTextPlugin);
#        app.add_plugins(HoverToolPlugin);
#        app.add_plugins(HoverUiAutomationPlugin);
#        app.add_plugins(IconPlugin);
#        app.add_plugins(InputPlugin);
#        app.add_plugins(InspectToolPlugin);
#        app.add_plugins(InspectWheelToolPlugin);
#        app.add_plugins(LevelBoundsPlugin);
#        app.add_plugins(MathPlugin);
#        app.add_plugins(MemoryPlugin);
#        app.add_plugins(PauseToolPlugin);
#        app.add_plugins(PhysicsDebugPlugin);
#        app.add_plugins(PhysicsPlugin);
#        app.add_plugins(PointerPlugin);
#        app.add_plugins(PointerTypesPlugin);
#        // app.add_plugins(PositionTextPlugin);
#        app.add_plugins(PressurePlatePlugin);
#        app.add_plugins(ScreenPlugin);
#        app.add_plugins(ScreenUpdatePlugin);
#        app.add_plugins(SprintToolPlugin);
#        app.add_plugins(SprintToolTypesPlugin);
#        app.add_plugins(TaskbarPlugin);
#        app.add_plugins(ToolbeltPlugin);
#        app.add_plugins(ToolbeltTypesPlugin);
#        app.add_plugins(ToolPlugin);
#        app.add_plugins(WallpaperPlugin);
#
#        // must be before the default plugins
#        app.add_plugins(EmbeddedAssetPlugin {
#            mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
#        });
#
#        #[cfg(debug_assertions)]
#        let log_plugin = LogPlugin {
#            level: bevy::log::Level::DEBUG,
#            filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_ecs=info,cursor_hero=debug".into(),
#            // filter: "debug,wgpu_core=warn,wgpu_hal=warn,bevy_ecs=info,cursor_hero=debug".into(),
#        };
#        #[cfg(not(debug_assertions))]
#        let log_plugin = LogPlugin {
#            level: bevy::log::Level::INFO,
#            filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
#        };
#        const AUDIO_SCALE: f32 = 1. / 100.0;
#        let version = match app.world.get_resource::<Version>() {
#            Some(version) => version.0.clone(),
#            None => {
#                warn!("Version resource not found");
#                "Unknown".to_string()
#            }
#        };
#        app.add_plugins(
#            DefaultPlugins
#                .set(ImagePlugin::default_nearest())
#                .set(AudioPlugin {
#                    spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
#                    ..default()
#                })
#                .set(WindowPlugin {
#                    primary_window: Some(Window {
#                        transparent: true,
#                        title: format!("Cursor Hero v{}", version),
#                        resizable: true,
#                        ..default()
#                    }),
#                    ..default()
#                })
#                .set(log_plugin)
#                .build(),
#        );
#
#        // must be after the default plugins
#        app.add_plugins(
#            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
#        );
#        app.add_plugins(FrameTimeDiagnosticsPlugin);
#    }
#}
#