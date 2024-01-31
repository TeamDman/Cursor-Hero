use bevy::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::prelude::*;

pub struct InspectWheelToolPlugin;

impl Plugin for InspectWheelToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InspectWheelTool>()
            .add_systems(Update, toolbelt_events)
            .add_systems(Update, tick);
    }
}

#[derive(Component, Reflect, Default)]
struct InspectWheelTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Default { toolbelt_id } = event {
            ToolSpawnConfig::<InspectWheelTool, NoInputs>::new(
                InspectWheelTool,
                *toolbelt_id,
                event,
            )
            .guess_name(file!())
            .guess_image(file!(), &asset_server)
            .with_description("Swaps to inspection tools")
            .with_starting_state(StartingState::Inactive)
            .spawn(&mut commands);
        }
    }
}

fn tick(
    mut commands: Commands,
    tool_query: Query<&Parent, (Added<ActiveTool>, With<InspectWheelTool>)>,
    mut toolbelt_events: EventWriter<PopulateToolbeltEvent>,
) {
    for toolbelt_id in tool_query.iter() {
        let toolbelt_id = toolbelt_id.get();
        commands.entity(toolbelt_id).despawn_descendants();
        toolbelt_events.send(PopulateToolbeltEvent::Inspector { toolbelt_id });
    }
}
