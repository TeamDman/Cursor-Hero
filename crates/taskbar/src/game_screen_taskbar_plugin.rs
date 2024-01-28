use bevy::prelude::*;
use cursor_hero_screen::screen_plugin::GameScreen;
use cursor_hero_screen::screen_plugin::Screen;
use cursor_hero_winutils::win_colors::get_accent_color;
use cursor_hero_winutils::win_colors::get_start_color;

pub struct GameScreenTaskbarPlugin;

impl Plugin for GameScreenTaskbarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Taskbar>()
            .add_event::<TaskbarEvent>()
            .add_systems(
                Update,
                (
                    detect_new_game_screens_and_send_taskbar_create_event,
                    handle_taskbar_create_events,
                ),
            );
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Taskbar;

#[derive(Event, Debug, Reflect)]
pub enum TaskbarEvent {
    Create { screen_id: Entity },
    Open { taskbar_id: Entity },
}

fn detect_new_game_screens_and_send_taskbar_create_event(
    mut taskbar_events: EventWriter<TaskbarEvent>,
    screen_query: Query<Entity, Added<GameScreen>>,
) {
    for screen_id in screen_query.iter() {
        taskbar_events.send(TaskbarEvent::Create { screen_id });
    }
}

fn handle_taskbar_create_events(
    mut taskbar_events: EventReader<TaskbarEvent>,
    mut commands: Commands,
    screen_query: Query<(&Transform, &Sprite), With<Screen>>,
) {
    for event in taskbar_events.read() {
        let TaskbarEvent::Create { screen_id } = event else {
            continue;
        };
        let Ok(screen) = screen_query.get(*screen_id) else {
            warn!("Couldn't find screen with id {:?}", screen_id);
            continue;
        };
        let (screen_transform, screen_sprite) = screen;
        let Some(screen_size) = screen_sprite.custom_size else {
            warn!("Screen {:?} has no custom size", screen_id);
            continue;
        };
        let taskbar_size = Vec2::new(screen_size.x, 40.0);
        let taskbar_translation = Vec3::new(0.0, -screen_size.y / 2.0 + 40.0 / 2.0, 2.0);

        let mut color = match get_start_color() {
            Ok(color) => color,
            Err(err) => {
                warn!("Couldn't get accent color: {:?}", err);
                Color::rgba(0.0, 0.0, 0.0, 1.0)
            }
        };
        color.set_a(0.9);

        commands.entity(*screen_id).with_children(|parent| {
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
        });
    }
}
