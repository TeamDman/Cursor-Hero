use bevy::prelude::*;

pub struct CharacterAppearancePlugin;
use cursor_hero_camera::camera_plugin::CameraEvent;
use cursor_hero_character_types::prelude::*;

impl Plugin for CharacterAppearancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_character_appearance_from_camera_events);
    }
}

fn update_character_appearance_from_camera_events(
    mut camera_events: EventReader<CameraEvent>,
    asset_server: Res<AssetServer>,
    mut character_query: Query<&mut Handle<Image>, With<Character>>,
) {
    for event in camera_events.read() {
        match event {
            CameraEvent::BeginFollowing { target_id } => {
                if let Ok(mut texture) = character_query.get_mut(*target_id) {
                    *texture = asset_server.load(CharacterAppearance::Focused.get_texture_path());
                    info!("Updated character appearance to focused");
                }
            }
            CameraEvent::StopFollowing { target_id } => {
                if let Ok(mut texture) = character_query.get_mut(*target_id) {
                    *texture = asset_server.load(CharacterAppearance::Unfocused.get_texture_path());
                    info!("Updated character appearance to unfocused");
                }
            }
        }
    }
}
