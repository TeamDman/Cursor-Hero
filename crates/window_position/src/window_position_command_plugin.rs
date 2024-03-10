use bevy::prelude::*;
use cursor_hero_window_position_types::window_position_types::WindowPositionCommand;

pub struct WindowPositionCommandPlugin;

impl Plugin for WindowPositionCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_commands);
    }
}

fn handle_commands(
    mut command_queue: EventReader<WindowPositionCommand>,
    mut window_query: Query<&mut Window>,
) {
    for command in command_queue.read() {
        let Ok(window) = window_query.get_mut(command.window) else {
            warn!("Window {:?} not found", command.window);
            continue;
        };
        debug!("Handling command {:?}", command);
        let mut window = window;
        if let Some(position) = command.position {
            window.position = position;
        }
        if let Some(resolution) = &command.resolution {
            window.resolution = resolution.clone();
        }
        if let Some(mode) = command.mode {
            window.mode = mode;
        }
        // only handle one command a tick
        break;
    }
}
