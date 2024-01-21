use bevy::prelude::*;
use cursor_hero_toolbelt::types::*;
use cursor_hero_tools::prelude::*;

use crate::hover_ui_automation_plugin::HoverInfo;

pub struct HoverToolPlugin;

impl Plugin for HoverToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HoverTool>()
            .add_systems(Update, toolbelt_events)
            .add_systems(Update, tick);
    }
}

#[derive(Component, Reflect)]
struct HoverTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        if let ToolbeltEvent::PopulateInspectorToolbelt {
            toolbelt_id,
            character_id,
        } = e
        {
            spawn_tool(
                Tool::create(
                    file!(),
                    "UI hover visuals".to_string(),
                    &asset_server,
                ),
                e,
                &mut commands,
                *toolbelt_id,
                *character_id,
                &asset_server,
                HoverTool,
                StartingState::Active,
            );
        }
    }
}

fn tick(
    tool_query: Query<Entity, (With<ActiveTool>, With<HoverTool>)>,
    mut hover_info: ResMut<HoverInfo>
) {
    if let Some(_) = tool_query.iter().next() {
        if !hover_info.is_enabled() {
            info!("Enabling hover info");
            hover_info.set_enabled(true);
        }
    } else if hover_info.is_enabled() {
        info!("Disabling hover info");
        hover_info.set_enabled(false);
    }
}
