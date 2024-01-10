use bevy::prelude::*;
use cursor_hero_version::version_plugin::Version;

pub struct AboutTextPlugin;

impl Plugin for AboutTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_text);
    }
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>, version: Res<Version>) {
    commands.spawn((
        TextBundle::from_section(
            format!("Cursor Hero v{}\nby @TeamDman", version.0),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 16.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        Name::new("About Text"),
    ));
}
