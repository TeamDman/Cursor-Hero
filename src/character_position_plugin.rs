use crate::{character_plugin::Character, camera_plugin::MainCamera};
use bevy::prelude::*;

pub struct CharacterPositionPlugin;

impl Plugin for CharacterPositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_position_text)
            .add_systems(Update, update_position_text);
    }
}

#[derive(Component)]
struct PositionText;

fn setup_position_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Position: ",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 16.0,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        PositionText,
    ));
}

fn update_position_text(
    character_query: Query<(&Character, &Transform), With<Character>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut text_query: Query<&mut Text, With<PositionText>>,
) {
    for (_character, transform) in character_query.iter() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!(
                "Position: {:.2}, {:.2}, {:.2}",
                transform.translation.x, transform.translation.y, transform.translation.z
            );
        }
    }
}
