use bevy::prelude::*;
use bevy::utils::HashSet;
use cursor_hero_bevy::prelude::Vec2ToRect;
use cursor_hero_environment_types::prelude::*;
use cursor_hero_screen::screen_plugin::Screen;
use cursor_hero_screen::screen_plugin::ScreenParent;

pub struct EnvironmentNametagPlugin;

impl Plugin for EnvironmentNametagPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NametagEvent>();
        app.add_systems(Update, spawn_nametags_in_new_environments);
        app.add_systems(Update, recalc_new_screen_nametags);
        app.add_systems(Update, handle_nametag_update_event);
        app.add_systems(Update, handle_nametag_recalculate_position_event);
        app.register_type::<Nametag>();
    }
}

#[allow(clippy::type_complexity)]
fn spawn_nametags_in_new_environments(
    mut environment_reader: EventReader<PopulateEnvironmentEvent>,
    mut commands: Commands,
    environment_query: Query<&Name, Or<(With<HostEnvironment>, With<AgentEnvironment>)>>,
    asset_server: Res<AssetServer>,
) {
    for event in environment_reader.read() {
        let Ok(environment_name) = environment_query.get(event.environment_id) else {
            continue;
        };
        let environment_id = event.environment_id;
        info!(
            "Spawning nametags for environment {:?} ({})",
            environment_id, environment_name
        );
        commands.entity(environment_id).with_children(|parent| {
            parent.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        environment_name.to_string(),
                        TextStyle {
                            font_size: 72.0,
                            font: asset_server
                                .load("fonts/kenney_kenney-fonts/Fonts/Kenney Future Narrow.ttf"),
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    transform: Transform::from_xyz(0.0, 200.0, 1.0).with_scale(Vec3::splat(4.0)),
                    ..default()
                },
                Nametag,
                Name::new("Nametag"),
            ));
        });
    }
}

fn recalc_new_screen_nametags(
    mut nametag_events: EventWriter<NametagEvent>,
    screen_query: Query<&Parent, Added<Screen>>,
    screen_parent_query: Query<&Parent, With<ScreenParent>>,
) {
    for new_screen_parent_id in screen_query.iter() {
        if let Ok(environment_id) = screen_parent_query.get(new_screen_parent_id.get()) {
            info!(
                "Detected new screen {:?} in environment {:?}, sending recalculate position event",
                new_screen_parent_id, environment_id
            );
            nametag_events.send(NametagEvent::RecalculatePosition {
                environment_id: environment_id.get(),
            });
        }
    }
}

fn handle_nametag_update_event(
    mut nametag_events: EventReader<NametagEvent>,
    environment_query: Query<&Children, With<EnvironmentKind>>,
    mut nametag_query: Query<(&mut Text, &mut Transform), With<Nametag>>,
) {
    for nametag_event in nametag_events.read() {
        if let NametagEvent::Update {
            environment_id,
            name,
        } = nametag_event
        {
            info!(
                "Updating nametag for environment {:?} to {}",
                environment_id, name
            );
            if let Ok(environment_children) = environment_query.get(*environment_id) {
                for child in environment_children.iter() {
                    if let Ok((mut nametag_text, _)) = nametag_query.get_mut(*child) {
                        nametag_text.sections[0].value.clone_from(name);
                    }
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn handle_nametag_recalculate_position_event(
    mut nametag_events: EventReader<NametagEvent>,
    environment_query: Query<&Children, With<EnvironmentKind>>,
    mut nametag_query: Query<(&mut Text, &mut Transform), (With<Nametag>, Without<Screen>)>,
    screen_parent_query: Query<&Children, With<ScreenParent>>,
    screen_query: Query<(&Sprite, &GlobalTransform), (With<Screen>, Without<Nametag>)>,
) {
    let mut debounce = HashSet::new();
    for nametag_event in nametag_events.read() {
        if let NametagEvent::RecalculatePosition { environment_id } = nametag_event {
            if debounce.contains(environment_id) {
                debug!(
                    "Debounced recalculate position event for environment {:?}, skipping",
                    environment_id
                );
                continue;
            }
            info!(
                "Recalculating nametag position for environment {:?}",
                environment_id
            );
            debounce.insert(environment_id);
            if let Ok(environment_children) = environment_query.get(*environment_id) {
                let mut max_extents = Rect::default();
                for environment_child_id in environment_children.iter() {
                    // debug!(
                    //     "Checking environment child {:?} for environment {:?}",
                    //     environment_child_id, environment_id
                    // );
                    if let Ok(screen_parent_children) =
                        screen_parent_query.get(*environment_child_id)
                    {
                        debug!(
                            "Found screen parent children {:?} for environment {:?}",
                            screen_parent_children, environment_id
                        );
                        for screen_id in screen_parent_children.iter() {
                            if let Ok((screen_sprite, screen_transform)) =
                                screen_query.get(*screen_id)
                            {
                                if let Some(screen_size) = screen_sprite.custom_size {
                                    max_extents =
                                        max_extents.union(screen_size.as_size_of_rect_with_center(
                                            &screen_transform.translation().xy(),
                                        ));
                                } else {
                                    warn!(
                                        "Screen {:?} did not have custom size, skipping",
                                        screen_id
                                    );
                                }
                            }
                        }
                    }
                }
                if max_extents.is_empty() {
                    warn!(
                        "Max extents for environment {:?} was empty, skipping",
                        environment_id
                    );
                    continue;
                }
                info!(
                    "Max extents for environment {:?} is {:?}",
                    environment_id, max_extents
                );
                for child in environment_children.iter() {
                    if let Ok((_, mut nametag_transform)) = nametag_query.get_mut(*child) {
                        nametag_transform.translation.x = max_extents.center().x;
                        nametag_transform.translation.y = max_extents.max.y + 200.0;
                    }
                }
            } else {
                warn!(
                    "Could not find environment children for environment {:?}",
                    environment_id
                );
            }
        }
    }
}
