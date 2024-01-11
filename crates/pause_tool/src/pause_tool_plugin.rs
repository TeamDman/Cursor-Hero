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
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        match e {
            ToolbeltEvent::PopulateInspectorToolbelt(toolbelt_id) => {
                spawn_action_tool::<PauseToolAction>(
                    e,
                    &mut commands,
                    *toolbelt_id,
                    &asset_server,
                    PauseTool,
                );
            }
            _ => {}
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
    mut commands: Commands,
    tools: Query<(
        &ActionState<PauseToolAction>,
        Option<&ToolActiveTag>,
        &Parent,
    )>,
) {
    for (t_act, t_enabled, t_parent) in tools.iter() {
        if t_enabled.is_none() {
            continue;
        }
        warn!("TODO: PauseToolAction: {:?}", t_act);
    }
}
