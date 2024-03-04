use bevy::prelude::*;
use cursor_hero_{{crate_name}}_tool_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::tool_spawning::ToolSpawnConfig;

pub struct {{crate_name_pascal}}ToolPopulatePlugin;

impl Plugin for {{crate_name_pascal}}ToolPopulatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_toolbelt_events);
    }
}

fn handle_toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        match event {
            PopulateToolbeltEvent::Default { toolbelt_id } => {
                ToolSpawnConfig::<_, {{crate_name_pascal}}ToolAction>::new(
                    {{crate_name_pascal}}Tool::default(),
                    event.id,
                    event,
                )
                .with_src_path(file!().into())
                .guess_name(file!())
                .guess_image(file!(), &asset_server, "png")
                .with_description("REPLACE THIS DESCRIPTION!!!")
                .spawn(&mut commands);
            }
            _ => {}
        }
    }
}
