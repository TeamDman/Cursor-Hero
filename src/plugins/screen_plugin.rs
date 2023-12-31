use crate::utils::win_screen_capture::get_all_monitors;
use bevy::prelude::*;
use image::DynamicImage;
use screenshots::Screen as ScreenLib;
use std::collections::VecDeque;

use super::level_bounds_plugin::{LevelBounds, LevelBoundsParent, LevelBoundsSystemSet};

pub struct ScreenPlugin;
impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                apply_deferred,
                spawn_screens.after(LevelBoundsSystemSet::Spawn),
            ),
        )
        .register_type::<Screen>()
        .register_type::<ScreenParent>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Screen {
    pub id: u32,
    pub name: String,
    pub refresh_rate: Timer,
}

#[derive(Component, Reflect)]
pub struct ScreenParent;

fn spawn_screens(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    level_bounds_parent_query: Query<Entity, With<LevelBoundsParent>>,
) {
    info!("Spawning screens");
    let mut screen_parent_commands = commands.spawn((
        SpatialBundle::default(),
        ScreenParent,
        Name::new("Screen Parent"),
    ));

    // create a Screen component for each screen
    let mut screen_names = get_all_monitors()
        .unwrap()
        .iter()
        .map(|monitor| monitor.info.name.clone())
        .collect::<VecDeque<String>>();

    let mut bounds = vec![];

    screen_parent_commands.with_children(|screen_parent| {
        for screen in ScreenLib::all().unwrap().iter() {
            let image_buf = screen.capture().unwrap();
            let dynamic_image = DynamicImage::ImageRgba8(image_buf);
            let image = Image::from_dynamic(dynamic_image, true);
            let texture = textures.add(image);
            let name = screen_names.pop_front().unwrap();

            screen_parent.spawn((
                SpriteBundle {
                    texture,
                    transform: Transform::from_xyz(
                        screen.display_info.x as f32 + (screen.display_info.width as f32) / 2.0,
                        -(screen.display_info.y as f32) - (screen.display_info.height as f32) / 2.0,
                        -1.0,
                    ),
                    ..Default::default()
                },
                Screen {
                    name: name.clone(),
                    id: screen.display_info.id,
                    refresh_rate: Timer::from_seconds(0.1, TimerMode::Repeating),
                },
                Name::new(format!("Screen {}", name)),
            ));
            bounds.push((
                screen.display_info.x,
                screen.display_info.y,
                screen.display_info.width,
                screen.display_info.height,
            ));
        }
    });
    if let Ok(level_bounds_parent) = level_bounds_parent_query.get_single() {
        commands
            .entity(level_bounds_parent)
            .with_children(|level_bounds_parent| {
                for (x, y, width, height) in bounds {
                    level_bounds_parent.spawn((
                        SpatialBundle {
                            transform: Transform::from_xyz(
                                x as f32 + (width as f32) / 2.0,
                                -(y as f32) - (height as f32) / 2.0,
                                0.0,
                            ),
                            ..Default::default()
                        },
                        LevelBounds,
                        Name::new(format!("Level Bounds")),
                    ));
                }
            });
    } else {
        unreachable!("Level bounds parent should exist by now");
    }
}
