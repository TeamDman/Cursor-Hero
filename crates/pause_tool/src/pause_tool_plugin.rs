use bevy::prelude::*;
use cursor_hero_toolbelt::types::*;
use cursor_hero_tools::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PauseToolPlugin;

impl Plugin for PauseToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PauseTool>()
            .add_plugins(InputManagerPlugin::<PauseToolAction>::default())
            .add_systems(Update, (toolbelt_events, handle_input));
    }
}

#[derive(Component, Reflect)]
struct PauseTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for e in reader.read() {
        if let ToolbeltPopulateEvent::Inspector {
            toolbelt_id,
            character_id,
        } = e
        {
            spawn_action_tool::<PauseToolAction>(
                Tool::create_with_actions::<PauseToolAction>(
                    file!(),
                    "Pauses the game (not yet implemented)".to_string(),
                    &asset_server,
                ),
                e,
                &mut commands,
                *toolbelt_id,
                *character_id,
                &asset_server,
                PauseTool,
                StartingState::Active,
                None,
            );
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum PauseToolAction {
    TogglePause,
}

impl PauseToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::TogglePause => GamepadButtonType::Start.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::TogglePause => KeyCode::Escape.into(),
        }
    }
}
impl ToolAction for PauseToolAction {
    fn default_input_map() -> InputMap<PauseToolAction> {
        let mut input_map = InputMap::default();

        for variant in PauseToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

fn handle_input(
    mut _commands: Commands,
    tool_query: Query<&ActionState<PauseToolAction>, (With<ActiveTool>, With<PauseTool>)>,
) {
    for t_act in tool_query.iter() {
        if t_act.just_pressed(PauseToolAction::TogglePause) {
            warn!("TODO: PauseToolAction: {:?}", t_act);
        }
    }
}
