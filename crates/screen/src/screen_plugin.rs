use bevy::prelude::*;
use cursor_hero_bevy::{IExpandable, NegativeYIRect};
use cursor_hero_environment::environment_plugin::PopulateEnvironmentEvent;
use cursor_hero_bevy::NegativeYIVec2;
use cursor_hero_level_bounds::level_bounds_plugin::LevelBoundsEvent;
use cursor_hero_winutils::win_screen_capture::get_all_monitors;
use image::DynamicImage;
use screenshots::Screen as ScreenLib;
use std::collections::VecDeque;

use crate::ToBevyIRect;

pub struct ScreenPlugin;
impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_screens_in_new_environments)
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

fn spawn_screens_in_new_environments(
    mut environment_reader: EventReader<PopulateEnvironmentEvent>,
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    mut level_bounds_events: EventWriter<LevelBoundsEvent>,
) {
    for environment_event in environment_reader.read() {
        match environment_event {
            PopulateEnvironmentEvent::Host { environment_id } => {
                info!("Spawning screens for host environment");
                commands.entity(*environment_id).with_children(|parent| {
                    let mut screen_parent_commands = parent.spawn((
                        SpatialBundle::default(),
                        ScreenParent,
                        Name::new("Screens"),
                    ));

                    // create a Screen component for each screen
                    let mut screen_names = get_all_monitors()
                        .unwrap()
                        .iter()
                        .map(|monitor| monitor.info.name.clone())
                        .collect::<VecDeque<String>>();
                    // todo: remove this and use win_screen_capture

                    let mut level_bounds = vec![];

                    screen_parent_commands.with_children(|screen_parent| {
                        for screen in ScreenLib::all().unwrap().iter() {
                            let image_buf = screen.capture().unwrap();
                            let dynamic_image = DynamicImage::ImageRgba8(image_buf);
                            let image = Image::from_dynamic(dynamic_image, true);
                            let texture = textures.add(image);
                            let name = screen_names.pop_front().unwrap();
                            let region = screen.display_info.to_bevy_irect().neg_y();
                            screen_parent.spawn((
                                SpriteBundle {
                                    texture,
                                    transform: Transform::from_translation(
                                        region.center().extend(-1).as_vec3(),
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

                            level_bounds.push(region.expand((400, 400).into()));
                        }
                    });
                    for area in level_bounds {
                        level_bounds_events.send(LevelBoundsEvent::AddPlayArea {
                            environment_id: *environment_id,
                            area: area.as_rect(),
                        });
                    }
                });
            }
            _ => {}
        }
    }
}
