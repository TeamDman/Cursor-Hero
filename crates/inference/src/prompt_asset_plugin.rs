use bevy::prelude::*;
use cursor_hero_inference_types::prelude::*;

pub struct PromptAssetPlugin;

impl Plugin for PromptAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TextPromptHandles>();
        app.add_systems(Startup, load_prompt_assets);
    }
}

fn load_prompt_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(TextPromptHandles {
        raw: asset_server.load("prompt_templates/raw.txt"),
        chat: asset_server.load("prompt_templates/chat.txt"),
    });
}
