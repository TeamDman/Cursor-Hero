use crate::prelude::*;
use bevy::prelude::*;

pub struct TextAssetTypesPlugin;

impl Plugin for TextAssetTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TextAsset>();
    }
}
