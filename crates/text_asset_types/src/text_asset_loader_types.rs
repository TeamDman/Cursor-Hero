use bevy::prelude::*;

use bevy::asset::io::Reader;
use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::LoadContext;
use bevy::utils::thiserror;
use bevy::utils::BoxedFuture;
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, Reflect, Debug, Deserialize)]
pub struct TextAsset {
    pub value: String,
}

#[derive(Default)]
pub struct TextAssetLoader;

/// Possible errors that can be produced by [`TextAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum TextAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for TextAssetLoader {
    type Asset = TextAsset;
    type Settings = ();
    type Error = TextAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = TextAsset {
                value: String::from_utf8(bytes).unwrap(),
            };
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
}
