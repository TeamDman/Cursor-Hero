use bevy::prelude::*;
use crate::prelude::*;

pub struct SecretsTypesPlugin;

impl Plugin for SecretsTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SecretString>();
    }
}