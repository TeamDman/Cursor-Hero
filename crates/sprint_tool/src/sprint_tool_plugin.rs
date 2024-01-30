use bevy::prelude::*;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_sprint_tool_types::sprint_tool_types_plugin::SprintData;
use cursor_hero_toolbelt::types::*;
use cursor_hero_tools::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct SprintToolPlugin;

impl Plugin for SprintToolPlugin {
    fn build(&self, app: &mut App) {
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
    fn default_input_map(_event: &PopulateToolbeltEvent) -> Option<InputMap<SprintToolAction>> {
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
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Default { toolbelt_id }
        | PopulateToolbeltEvent::Inspector { toolbelt_id }
        | PopulateToolbeltEvent::Keyboard { toolbelt_id } = event
        {
            ToolSpawnConfig::<SprintTool, SprintToolAction>::new(SprintTool, *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Go faster, reach further")
                .spawn(&mut commands);
        }
    }
}

#[derive(Component, Reflect)]
pub struct SpawnedCube;

// TODO: convert this to use events instead
fn handle_input(
    sprint_tool_query: Query<(&ActionState<SprintToolAction>, &Parent), With<ActiveTool>>,
    mut sprint_data_query: Query<&mut SprintData>,
    toolbelt_query: Query<(&Parent, &Children), With<Toolbelt>>,
    mut character_query: Query<&Children, With<Character>>,
) {
    for sprint_tool in sprint_tool_query.iter() {
        let (tool_actions, tool_parent) = sprint_tool;

        let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
            warn!("Sprint tool not inside a toolbelt?");
            continue;
        };
        let (toolbelt_parent, toolbelt_children) = toolbelt;

        let Ok(character) = character_query.get_mut(toolbelt_parent.get()) else {
            warn!("Toolbelt parent not a character?");
            continue;
        };
        let character_kids = character;

        if tool_actions.pressed(SprintToolAction::Sprint) {
            if tool_actions.just_pressed(SprintToolAction::Sprint) {
                debug!("Sprint tool action just pressed");
            }
            let mut open = tool_actions.value(SprintToolAction::Sprint);
            open = open.powf(2.0);
            for toolbelt_child in toolbelt_children.iter().chain(character_kids.iter()) {
                if let Ok(mut sprint_data) = sprint_data_query.get_mut(*toolbelt_child) {
                    if sprint_data.sprint_enabled {
                        let sprint_bonus = sprint_data.default_value - sprint_data.sprint_value;
                        sprint_data.value = sprint_data.default_value + sprint_bonus * open;
                    }
                }
            }
        } else if tool_actions.just_released(SprintToolAction::Sprint) {
            debug!("Sprint tool action released");
            for toolbelt_child in toolbelt_children.iter().chain(character_kids.iter()) {
                if let Ok(mut sprint_data) = sprint_data_query.get_mut(*toolbelt_child) {
                    if sprint_data.sprint_enabled {
                        sprint_data.value = sprint_data.default_value;
                    }
                }
            }
        }
    }
}
