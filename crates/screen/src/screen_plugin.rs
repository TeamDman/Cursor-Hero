use bevy::prelude::*;
use cursor_hero_bevy::prelude::IExpandable;
use cursor_hero_bevy::prelude::NegativeYIRect;
use cursor_hero_environment_types::prelude::*;
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
    pub refresh_rate: Option<Timer>,
}
#[derive(Component, Default, Reflect)]
pub struct GameScreen;
#[derive(Component, Default, Reflect)]
pub struct HostScreen;

#[derive(Component, Reflect)]
pub struct ScreenParent;

fn spawn_screens_in_new_environments(
    mut populate_events: EventReader<PopulateEnvironmentEvent>,
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    mut level_bounds_events: EventWriter<LevelBoundsEvent>,
    environment_query: Query<(Option<&HostEnvironment>, Option<&GameEnvironment>)>,
) {
    for event in populate_events.read() {
        let environment_id = event.environment_id;
        let Ok((is_host, is_game)) = environment_query.get(event.environment_id) else {
            continue;
        };
        match (is_host, is_game) {
            (Some(_), _) => {
                info!("Populating host environment with screens");
                commands.entity(environment_id).with_children(|parent| {
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
                                    sprite: Sprite {
                                        custom_size: Some(region.size().as_vec2()),
                                        ..default()
                                    },
                                    ..Default::default()
                                },
                                Screen {
                                    name: name.clone(),
                                    id: screen.display_info.id,
                                    refresh_rate: Some(Timer::from_seconds(
                                        0.1,
                                        TimerMode::Repeating,
                                    )),
                                },
                                HostScreen,
                                Name::new(format!("Screen {}", name)),
                            ));

                            level_bounds.push(region.expand((400, 400).into()));
                        }
                    });
                    info!("Broadcasting {} level bounds events", level_bounds.len());
                    for area in level_bounds {
                        level_bounds_events.send(LevelBoundsEvent::AddPlayArea {
                            environment_id,
                            area: area.as_rect(),
                        });
                    }
                });
            }
            (_, Some(_)) => {
                commands.entity(environment_id).with_children(|parent| {
                    info!("Populating game environment with screens");
                    let mut screen_parent_commands = parent.spawn((
                        SpatialBundle::default(),
                        ScreenParent,
                        Name::new("Screens"),
                    ));

                    let mut level_bounds = vec![];

                    screen_parent_commands.with_children(|screen_parent| {
                        let region =
                            IRect::from_corners(IVec2::new(0, 0), IVec2::new(1920, 1080)).neg_y();
                        let name = "Primary Screen".to_string();
                        screen_parent.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(region.size().as_vec2()),
                                    ..default()
                                },
                                transform: Transform::from_translation(
                                    region.center().extend(-1).as_vec3(),
                                ),
                                ..Default::default()
                            },
                            Screen {
                                name: name.to_string(),
                                id: 1,
                                refresh_rate: None,
                            },
                            GameScreen,
                            Name::new(name),
                        ));

                        level_bounds.push(region.expand((400, 400).into()));
                    });
                    info!("Broadcasting {} level bounds events", level_bounds.len());
                    for area in level_bounds {
                        level_bounds_events.send(LevelBoundsEvent::AddPlayArea {
                            environment_id,
                            area: area.as_rect(),
                        });
                    }
                });
            }
            (None, None) => {
                error!(
                    "Environment {:?} is not a host or game environment",
                    environment_id
                );
            }
        }
    }
}
