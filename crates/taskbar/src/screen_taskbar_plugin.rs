use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use cursor_hero_bevy::AabbToRect;
use cursor_hero_environment::environment_plugin::PopulateEnvironmentEvent;
use cursor_hero_screen::screen_plugin::Screen;
use cursor_hero_screen::screen_plugin::ScreenParent;

pub struct ScreenTaskbarPlugin;

impl Plugin for ScreenTaskbarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Taskbar>()
            .add_event::<TaskbarEvent>()
            .add_systems(Update, populate_taskbar)
            .add_systems(Update, detect_new_screens_and_send_send_taskbar_event);
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Taskbar;

#[derive(Event, Debug, Reflect)]
pub enum TaskbarEvent {
    Create {
        environment_id: Entity,
        dimensions: Rect,
    },
}

fn detect_new_screens_and_send_send_taskbar_event(
    mut taskbar_events: EventWriter<TaskbarEvent>,
    screen_query: Query<(&Aabb, &Parent), Added<Screen>>,
    screen_parent_query: Query<&Parent, With<ScreenParent>>,
) {
    for (screen_bounds, screen_parent_id) in screen_query.iter() {
        if let Ok(environment_id) = screen_parent_query.get(screen_parent_id.get()) {
            info!(
                "Detected new screen in environment {:?}, sending taskbar event",
                environment_id
            );
            taskbar_events.send(TaskbarEvent::Create {
                environment_id: environment_id.get(),
                dimensions: screen_bounds.to_rect(),
            });
        }
    }
}

fn populate_taskbar(
    mut environment_events: EventReader<PopulateEnvironmentEvent>,
    mut commands: Commands,
) {
    for event in environment_events.read() {
        if let PopulateEnvironmentEvent::Game { environment_id } = event {
            commands.entity(*environment_id).with_children(|parent| {
                parent.spawn((
                    Taskbar,
                    Name::new("Taskbar"),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(1920.0, 40.0)),
                            color: Color::rgb(0.0, 0.0, 0.0),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, -540.0, 0.0)),
                        ..default()
                    },
                ));
            });
        }
    }
}
