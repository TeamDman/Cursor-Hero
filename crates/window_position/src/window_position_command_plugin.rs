use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use cursor_hero_window_position_types::window_position_types::WindowPositionCommand;
use cursor_hero_window_position_types::window_position_types::WindowPositionCommandQueue;

pub struct WindowPositionCommandPlugin;

impl Plugin for WindowPositionCommandPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WindowPositionCommandQueue>();
        app.add_systems(Update, handle_commands);
    }
}

fn handle_commands(
    mut command_queue: ResMut<WindowPositionCommandQueue>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single_mut() else {
        warn!("No window found");
        return;
    };
    let mut window = window;
    for command in command_queue.commands.drain(0..) {
        if let Some(position) = command.position {
            window.position = position;
        }
        if let Some(resolution) = command.resolution {
            window.resolution = resolution;
        }
        if let Some(mode) = command.mode {
            window.mode = mode;
        }
    }
}
