use crate::plugins::character_plugin::Character;
use bevy::prelude::*;

use super::types::*;

pub fn toolbelt_spawn_setup_system(
    mut commands: Commands,
    character: Query<Entity, With<Character>>,
    mut writer: EventWriter<ToolbeltEvent>,
) {
    let character_id = character.single();
    commands.entity(character_id).with_children(|c_commands| {
        let toolbelt = c_commands.spawn(ToolbeltBundle::default());
        writer.send(ToolbeltEvent::Populate(toolbelt.id()));
    });

    info!("Toolbelt setup complete");
}
