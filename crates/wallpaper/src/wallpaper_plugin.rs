use bevy::prelude::*;
use cursor_hero_environment::environment_plugin::PopulateEnvironmentEvent;

pub struct WallpaperPlugin;

impl Plugin for WallpaperPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Wallpaper>()
            .add_systems(Update, spawn_wallpaper);
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Wallpaper;

fn spawn_wallpaper(
    mut commands: Commands,
    mut environment_events: EventReader<PopulateEnvironmentEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in environment_events.read() {
        if let PopulateEnvironmentEvent::Game { environment_id } = event {
            info!(
                "Spawning wallpaper for game environment {:?}",
                environment_id
            );
            commands.entity(*environment_id).with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(1920.0, 1080.0)),
                            anchor: bevy::sprite::Anchor::TopLeft,
                            ..default()
                        },
                        texture: asset_server.load("textures/environment/game/wallpaper.png"),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                        ..default()
                    },
                    Name::new("Wallpaper"),
                ));
            });
        }
    }
}
