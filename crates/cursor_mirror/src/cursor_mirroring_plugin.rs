use bevy::prelude::*;
use bevy::sprite::Anchor;

use cursor_hero_winutils::win_mouse::get_cursor_position;

pub struct CursorMirroringPlugin;
impl Plugin for CursorMirroringPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CursorMirror>()
            .add_systems(Startup, setup)
            .add_systems(Update, (update_cursor_position, update_visuals).chain())
            .insert_resource(CursorPosition::default());
    }
}

#[derive(Component, Reflect)]
pub struct CursorMirror;

#[derive(Resource, Deref, Default)]
pub struct CursorPosition(pub IVec2);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 20.0),
            sprite: Sprite {
                // custom_size: Some(Vec2::new(20.0, 20.0)),
                anchor: Anchor::TopLeft,
                ..default()
            },

            texture: asset_server.load("textures/cursor.png"),
            ..default()
        },
        CursorMirror,
        Name::new("Cursor Mirror"),
    ));
}

fn update_cursor_position(mut res: ResMut<CursorPosition>) {
    if let Ok(pos) = get_cursor_position() {
        res.0 = pos;
    }
}

fn update_visuals(
    mut cursor_mirrors: Query<(&mut Transform, &CursorMirror)>,
    cursor_position: Res<CursorPosition>,
) {
    for (mut transform, _) in &mut cursor_mirrors.iter_mut() {
        transform.translation.x = cursor_position.x as f32;
        transform.translation.y = -cursor_position.y as f32;
    }
}
