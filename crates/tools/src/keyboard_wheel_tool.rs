use crate::prelude::*;
use bevy::prelude::*;
use cursor_hero_toolbelt::types::*;

pub struct KeyboardWheelToolPlugin;

impl Plugin for KeyboardWheelToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<KeyboardWheelTool>()
            .add_systems(Update, toolbelt_events)
            .add_systems(Update, tick);
    }
}

#[derive(Component, Reflect, Default)]
struct KeyboardWheelTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Default { toolbelt_id } = event {
            ToolSpawnConfig::<KeyboardWheelTool, NoInputs>::new(
                KeyboardWheelTool,
                *toolbelt_id,
                event,
            )
            .guess_name(file!())
            .guess_image(file!(), &asset_server)
            .with_description("Swaps to keyboard tools")
            .with_starting_state(StartingState::Inactive)
            .spawn(&mut commands);
        }
    }
}

fn tick(
    mut commands: Commands,
    tool_query: Query<&Parent, (Added<ActiveTool>, With<KeyboardWheelTool>)>,
    mut toolbelt_events: EventWriter<PopulateToolbeltEvent>,
) {
    for toolbelt_id in tool_query.iter() {
        info!("Switching toolbelt {:?} to keyboard tools", toolbelt_id);
        let toolbelt_id = toolbelt_id.get();
        commands.entity(toolbelt_id).despawn_descendants();
        toolbelt_events.send(PopulateToolbeltEvent::Keyboard { toolbelt_id });
    }
}
