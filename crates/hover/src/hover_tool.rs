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

#[derive(Component, Reflect, Default)]
struct HoverTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for event in reader.read() {
        if let ToolbeltPopulateEvent::Inspector { toolbelt_id } = event {
            ToolSpawnConfig::<HoverTool, NoInputs>::new(HoverTool, *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("UI hover visuals")
                .spawn(&mut commands);
        }
    }
}

fn tick(
    tool_query: Query<Entity, (With<ActiveTool>, With<HoverTool>)>,
    mut hover_info: ResMut<HoverInfo>,
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
