use crate::plugins::{camera_plugin::MainCamera, character_plugin::Character};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct PositionTextPlugin;

impl Plugin for PositionTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_position_text)
            .add_systems(Update, update_position_text)
            .register_type::<CharacterPositionText>()
            .register_type::<MouseWorldPositionText>()
            .register_type::<MouseScreenPositionText>();
    }
}

#[derive(Component, Reflect)]
struct CharacterPositionText;
#[derive(Component, Reflect)]
struct MouseWorldPositionText;
#[derive(Component, Reflect)]
struct MouseScreenPositionText;

fn setup_position_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Character: ",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 16.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        CharacterPositionText,
        Name::new("Character Position Text"),
    ));
    commands.spawn((
        TextBundle::from_section(
            "Mouse Cursor: ",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 16.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15.0),
            left: Val::Px(5.0),
            ..default()
        }),
        MouseWorldPositionText,
        Name::new("Mouse Cursor Position Text"),
    ));
    commands.spawn((
        TextBundle::from_section(
            "Mouse Screen: ",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 16.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(25.0),
            left: Val::Px(5.0),
            ..default()
        }),
        MouseScreenPositionText,
        Name::new("Mouse Screen Position Text"),
    ));
}

fn update_position_text(
    character_query: Query<(&Character, &Transform), With<Character>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut character_position_text_query: Query<
        &mut Text,
        (
            With<CharacterPositionText>,
            Without<MouseWorldPositionText>,
            Without<MouseScreenPositionText>,
        ),
    >,
    mut mouse_world_position_text_query: Query<
        &mut Text,
        (
            With<MouseWorldPositionText>,
            Without<CharacterPositionText>,
            Without<MouseScreenPositionText>,
        ),
    >,
    mut mouse_screen_position_text_query: Query<
        &mut Text,
        (
            With<MouseScreenPositionText>,
            Without<CharacterPositionText>,
            Without<MouseWorldPositionText>,
        ),
    >,
    // mut text_query: Query<&mut Text, With<CharacterPositionText>>,
) {
    for (_character, transform) in character_query.iter() {
        character_position_text_query.single_mut().sections[0].value = format!(
            "Character: {:.2}, {:.2}, {:.2}",
            transform.translation.x, transform.translation.y, transform.translation.z
        );
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_world_position_text_query.single_mut().sections[0].value = format!(
            "Mouse world: {:.2}, {:.2}",
            world_position.x, world_position.y,
        );
    } else {
        mouse_world_position_text_query.single_mut().sections[0].value = "Mouse world: None".to_string();
    }
    if let Some(cursor_position) = window.cursor_position() {
        mouse_screen_position_text_query.single_mut().sections[0].value = format!(
            "Mouse screen: {:.2}, {:.2}",
            cursor_position.x, cursor_position.y,
        );
    } else {
        mouse_screen_position_text_query.single_mut().sections[0].value = "Mouse screen: None".to_string();
    }
}
