use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use cursor_hero_fullscreen_tool_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::tool_spawning::StartingState;

pub struct FullscreenToolTickPlugin;

impl Plugin for FullscreenToolTickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_toggle);
        app.add_systems(Update, match_window_state);
    }
}

fn match_window_state(
    mut commands: Commands,
    tool_query: Query<(Entity, Option<&ActiveTool>), With<FullscreenTool>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(mode) = window_query.iter().map(|w| w.mode).next() else {
        warn!("No window found");
        return;
    };
    for tool in tool_query.iter() {
        let (tool_id, tool_active) = tool;
        let desired_state = FullscreenTool::state_for_mode(mode);
        if tool_active == desired_state.as_active().as_ref() {
            continue;
        }
        match desired_state {
            StartingState::Active => {
                info!("Activating FullscreenTool without event to match window state");
                commands.entity(tool_id).insert(ActiveTool);
            }
            StartingState::Inactive => {
                info!("Deactivating FullscreenTool without event to match window state");
                commands.entity(tool_id).remove::<ActiveTool>();
            }
        }
    }
}

fn handle_toggle(
    tool_query: Query<(), With<FullscreenTool>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut tool_events: EventReader<ToolActivationEvent>,
) {
    for event in tool_events.read() {
        match event {
            ToolActivationEvent::Activate(tool_id) if tool_query.contains(*tool_id) => {
                info!("FullscreenTool activated, setting window to fullscreen.");
                if window_query.is_empty() {
                    warn!("No window found");
                    continue;
                }
                for mut window in window_query.iter_mut() {
                    window.mode = bevy::window::WindowMode::BorderlessFullscreen;
                }
            }
            ToolActivationEvent::Deactivate(tool_id) if tool_query.contains(*tool_id) => {
                info!("FullscreenTool deactivated, setting window to windowed.");
                if window_query.is_empty() {
                    warn!("No window found");
                    continue;
                }
                for mut window in window_query.iter_mut() {
                    window.mode = bevy::window::WindowMode::Windowed;
                }
            }
            _ => {}
        }
    }
}
