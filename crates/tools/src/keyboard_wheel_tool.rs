use crate::prelude::*;
use bevy::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;

pub struct KeyboardWheelToolPlugin;

impl Plugin for KeyboardWheelToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<KeyboardWheelTool>();
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, tick);
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
        let ToolbeltLoadout::Default = event.loadout else {
            continue;
        };
        ToolSpawnConfig::<KeyboardWheelTool, NoInputs>::new(KeyboardWheelTool, event.id, event)
            .with_src_path(file!().into())
            .guess_name(file!())
            .guess_image(file!(), &asset_server, "png")
            .with_description("Swaps to keyboard tools")
            .with_starting_state(StartingState::Inactive)
            .spawn(&mut commands);
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
        toolbelt_events.send(PopulateToolbeltEvent {
            id: toolbelt_id,
            loadout: ToolbeltLoadout::Keyboard,
        });
    }
}
