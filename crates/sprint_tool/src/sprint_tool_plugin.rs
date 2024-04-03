use bevy::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_sprint_tool_types::sprint_tool_types_plugin::SprintEvent;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct SprintToolPlugin;

impl Plugin for SprintToolPlugin {
    fn build(&self, app: &mut App) {
        // TODO: move to sprint_tool_types crate
        app.register_type::<SprintTool>();
        app.register_type::<SpawnedCube>();
        app.add_plugins(InputManagerPlugin::<SprintToolAction>::default());
        app.add_systems(Update, (toolbelt_events, handle_input));
    }
}

#[derive(Component, Reflect, Default)]
struct SprintTool;
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum SprintToolAction {
    Sprint,
}

impl SprintToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Sprint => GamepadButtonType::LeftTrigger2.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Sprint => KeyCode::ShiftLeft.into(),
        }
    }
}
impl ToolAction for SprintToolAction {
    fn default_input_map(_event: &ToolbeltPopulateEvent) -> Option<InputMap<SprintToolAction>> {
        let mut input_map = InputMap::default();

        for variant in SprintToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for event in reader.read() {
        let (ToolbeltLoadout::Default | ToolbeltLoadout::Keyboard) =
            event.loadout
        else {
            continue;
        };
        ToolSpawnConfig::<SprintTool, SprintToolAction>::new(SprintTool, event.id, event)
            .with_src_path(file!().into())
            .guess_name(file!())
            .guess_image(file!(), &asset_server, "png")
            .with_description("Go faster, reach further")
            .spawn(&mut commands);
    }
}

#[derive(Component, Reflect)]
pub struct SpawnedCube;

fn handle_input(
    sprint_tool_query: Query<(&ActionState<SprintToolAction>, &Parent), With<ActiveTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    mut character_query: Query<Entity, With<Character>>,
    mut sprint_events: EventWriter<SprintEvent>,
) {
    for sprint_tool in sprint_tool_query.iter() {
        let (tool_actions, tool_parent) = sprint_tool;

        let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
            warn!("Tool not inside a toolbelt?");
            continue;
        };
        let toolbelt_parent = toolbelt;
        let Ok(character) = character_query.get_mut(toolbelt_parent.get()) else {
            warn!("Toolbelt parent not a character?");
            continue;
        };
        let character_id = character;

        if tool_actions.pressed(SprintToolAction::Sprint) {
            if tool_actions.just_pressed(SprintToolAction::Sprint) {
                debug!("Sprint tool action just pressed");
            }
            let mut throttle = tool_actions.value(SprintToolAction::Sprint);
            throttle = throttle.powf(2.0);
            sprint_events.send(SprintEvent::Active {
                character_id,
                throttle,
            });
        } else if tool_actions.just_released(SprintToolAction::Sprint) {
            debug!("Sprint tool action released");
            sprint_events.send(SprintEvent::Stop { character_id });
        }
    }
}
