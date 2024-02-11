use bevy::prelude::*;
use cursor_hero_text_asset_types::prelude::*;
pub struct TextAssetPlugin;

impl Plugin for TextAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<TextAsset>();
        app.init_asset_loader::<TextAssetLoader>();
    }
}
