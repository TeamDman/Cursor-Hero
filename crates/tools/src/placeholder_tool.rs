use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::UserInput;
use leafwing_input_manager::Actionlike;

use cursor_hero_toolbelt::types::*;

use crate::tool_spawning::{spawn_action_tool, StartingState};
pub struct PlaceholderToolPlugin;

impl Plugin for PlaceholderToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlaceholderTool>()
            .add_plugins(InputManagerPlugin::<PlaceholderToolAction>::default())
            .add_systems(Update, (toolbelt_events, handle_input));
    }
}

#[derive(Component, Reflect)]
struct PlaceholderTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum PlaceholderToolAction {
    Action1,
    Action2,
    Action3,
}

impl PlaceholderToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Action1 => GamepadButtonType::South.into(),
            Self::Action2 => GamepadButtonType::East.into(),
            Self::Action3 => GamepadButtonType::West.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Action1 => KeyCode::ControlLeft.into(),
            Self::Action2 => KeyCode::ControlRight.into(),
            Self::Action3 => KeyCode::AltRight.into(),
        }
    }
}
impl ToolAction for PlaceholderToolAction {
    fn default_input_map() -> InputMap<PlaceholderToolAction> {
        let mut input_map = InputMap::default();

        for variant in PlaceholderToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for e in reader.read() {
        if let ToolbeltPopulateEvent::Default {
            toolbelt_id,
            character_id,
        } = e
        {
            for _ in 0..0 {
                // for _ in 0..1 {
                spawn_action_tool::<PlaceholderToolAction>(
                    Tool::create_with_actions::<PlaceholderToolAction>(
                        file!(),
                        "Balances the wheel".to_string(),
                        &asset_server,
                    ),
                    e,
                    &mut commands,
                    *toolbelt_id,
                    *character_id,
                    &asset_server,
                    PlaceholderTool,
                    StartingState::Active,
                    None,
                );
            }
        }
    }
}

fn handle_input(actors: Query<(&ActionState<PlaceholderToolAction>, Option<&ActiveTool>)>) {
    for (action_state, active_tool_tag) in actors.iter() {
        if active_tool_tag.is_none() {
            continue;
        }
        if action_state.just_pressed(PlaceholderToolAction::Action1) {
            info!("Just pressed Action1");
        }
        if action_state.just_pressed(PlaceholderToolAction::Action2) {
            info!("Just pressed Action2");
        }
        if action_state.just_pressed(PlaceholderToolAction::Action3) {
            info!("Just pressed Action3");
        }
    }
}
