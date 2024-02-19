use bevy::prelude::*;
use cursor_hero_screen::screen_plugin::GameScreen;
use cursor_hero_taskbar_types::prelude::*;
use cursor_hero_winutils::win_colors::get_start_color;

pub struct TaskbarSpawnPlugin;

impl Plugin for TaskbarSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_taskbar);
    }
}

fn spawn_taskbar(
    mut commands: Commands,
    mut taskbar_events: EventWriter<TaskbarEvent>,
    screen_query: Query<(Entity, &Sprite), Added<GameScreen>>,
) {
    for screen in screen_query.iter() {
        let (screen_id, screen_sprite) = screen;
        let Some(screen_size) = screen_sprite.custom_size else {
            warn!("Screen {:?} has no custom size", screen_id);
            continue;
        };
        let taskbar_size = Vec2::new(screen_size.x, 40.0);
        let taskbar_translation = Vec3::new(0.0, -screen_size.y / 2.0 + 40.0 / 2.0, 5.0);

        let mut color = match get_start_color() {
            Ok(color) => color,
            Err(err) => {
                warn!("Couldn't get accent color: {:?}", err);
                Color::rgba(0.0, 0.0, 0.0, 1.0)
            }
        };
        color.set_a(0.9);

        let taskbar_id = commands.entity(screen_id).with_children(|parent| {
            parent.spawn((
                Taskbar,
                Name::new("Taskbar"),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(taskbar_size),
                        color,
                        ..default()
                    },
                    transform: Transform::from_translation(taskbar_translation),
                    ..default()
                },
            ));
        }).id();

        let event = TaskbarEvent::Populate { taskbar_id };
        debug!("Sending taskbar event: {:?}", event);
        taskbar_events.send(event);
    }
}
