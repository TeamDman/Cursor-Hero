use crate::prelude::*;
use bevy::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;

pub struct DefaultWheelToolPlugin;

impl Plugin for DefaultWheelToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DefaultWheelTool>()
            .add_systems(Update, toolbelt_events)
            .add_systems(Update, tick);
    }
}

#[derive(Component, Reflect, Default)]
struct DefaultWheelTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Inspector { toolbelt_id }
        | PopulateToolbeltEvent::Taskbar { toolbelt_id }
        | PopulateToolbeltEvent::Keyboard { toolbelt_id } = event
        {
            ToolSpawnConfig::<DefaultWheelTool, NoInputs>::new(
                DefaultWheelTool,
                *toolbelt_id,
                event,
            )
            .guess_name(file!())
            .guess_image(file!(), &asset_server)
            .with_description("Swaps to default tools")
            .with_starting_state(StartingState::Inactive)
            .spawn(&mut commands);
        }
    }
}

fn tick(
    mut commands: Commands,
    tool_query: Query<&Parent, (Added<ActiveTool>, With<DefaultWheelTool>)>,
    mut toolbelt_events: EventWriter<PopulateToolbeltEvent>,
) {
    for toolbelt_id in tool_query.iter() {
        info!("Switching toolbelt {:?} to default tools", toolbelt_id);
        let toolbelt_id = toolbelt_id.get();
        commands.entity(toolbelt_id).despawn_descendants();
        toolbelt_events.send(PopulateToolbeltEvent::Default { toolbelt_id });
    }
}
