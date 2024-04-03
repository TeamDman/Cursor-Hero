use crate::prelude::*;
use bevy::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;

pub struct DefaultWheelToolPlugin;

impl Plugin for DefaultWheelToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DefaultWheelTool>();
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, tick);
    }
}

#[derive(Component, Reflect, Default)]
struct DefaultWheelTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for event in reader.read() {
        let (ToolbeltLoadout::Taskbar
        | ToolbeltLoadout::Keyboard
        | ToolbeltLoadout::WindowPosition) = event.loadout
        else {
            continue;
        };
        ToolSpawnConfig::<DefaultWheelTool, NoInputs>::new(DefaultWheelTool, event.id, event)
            .with_src_path(file!().into())
            .guess_name(file!())
            .guess_image(file!(), &asset_server, "png")
            .with_description("Swaps to default tools")
            .with_starting_state(StartingState::Inactive)
            .spawn(&mut commands);
    }
}

fn tick(
    mut commands: Commands,
    tool_query: Query<&Parent, (Added<ActiveTool>, With<DefaultWheelTool>)>,
    mut toolbelt_events: EventWriter<ToolbeltPopulateEvent>,
) {
    for toolbelt_id in tool_query.iter() {
        info!("Switching toolbelt {:?} to default tools", toolbelt_id);
        let toolbelt_id = toolbelt_id.get();
        commands.entity(toolbelt_id).despawn_descendants();
        toolbelt_events.send(ToolbeltPopulateEvent {
            id: toolbelt_id,
            loadout: ToolbeltLoadout::Default,
        });
    }
}
