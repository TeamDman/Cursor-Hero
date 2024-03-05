use crate::prelude::*;
use bevy::prelude::*;
use cursor_hero_level_bounds::level_bounds_plugin::LevelBounds;
use cursor_hero_toolbelt_types::prelude::*;

pub struct LevelBoundsVisibilityToolPlugin;

impl Plugin for LevelBoundsVisibilityToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LevelBoundsVisibilityTool>();
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, tick);
    }
}

#[derive(Component, Reflect, Default)]
struct LevelBoundsVisibilityTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for event in reader.read() {
        let ToolbeltLoadout::Inspector = event.loadout else {
            continue;
        };
        ToolSpawnConfig::<LevelBoundsVisibilityTool, NoInputs>::new(
            LevelBoundsVisibilityTool,
            event.id,
            event,
        )
        .with_src_path(file!().into())
        .guess_name(file!())
        .guess_image(file!(), &asset_server, "png")
        .with_description("Shows the play area.")
        .with_starting_state(StartingState::Inactive)
        .spawn(&mut commands);
    }
}

fn tick(
    mut commands: Commands,
    tool_query: Query<Entity, (Added<ActiveTool>, With<LevelBoundsVisibilityTool>)>,
    mut level_bounds_query: Query<&mut Visibility, With<LevelBounds>>,
) {
    for tool_id in tool_query.iter() {
        commands.entity(tool_id).remove::<ActiveTool>();
        for mut visibility in level_bounds_query.iter_mut() {
            *visibility = match *visibility {
                Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
                Visibility::Inherited => Visibility::Visible,
            };
        }
    }
}
