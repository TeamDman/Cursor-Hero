use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use cursor_hero_fullscreen_tool_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct FullscreenToolTickPlugin;

impl Plugin for FullscreenToolTickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick);
    }
}

fn tick(
    tool_query: Query<(), With<FullscreenTool>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut tool_events: EventReader<ToolActivationEvent>,
) {
    for event in tool_events.read() {
        match event {
            ToolActivationEvent::Activate(tool_id) if tool_query.contains(*tool_id) => {
                info!("FullscreenTool activated.");
                for mut window in window_query.iter_mut() {
                    window.mode = bevy::window::WindowMode::BorderlessFullscreen;
                }
            }
            ToolActivationEvent::Deactivate(tool_id) if tool_query.contains(*tool_id) => {
                info!("FullscreenTool deactivated.");
                for mut window in window_query.iter_mut() {
                    window.mode = bevy::window::WindowMode::Windowed;
                }
            }
            _ => {}
        }
    }
}
