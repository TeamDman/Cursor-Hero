use bevy::prelude::*;

use crate::prompt_asset_plugin::PromptAssetPlugin;

pub struct InferencePlugin;

impl Plugin for InferencePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PromptAssetPlugin);
    }
}
