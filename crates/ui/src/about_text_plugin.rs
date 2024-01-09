use bevy::prelude::*;

pub struct AboutTextPlugin;

impl Plugin for AboutTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_text);
    }
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            format!("Cursor Hero v0.2.0\nby @TeamDman"),
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
