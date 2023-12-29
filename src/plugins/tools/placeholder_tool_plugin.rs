use bevy::prelude::*;
use leafwing_input_manager::{
    action_state::ActionState, input_map::InputMap, user_input::UserInput, Actionlike,
    InputManagerBundle,
};

use super::super::toolbelt::types::*;
pub struct PlaceholderToolPlugin;

impl Plugin for PlaceholderToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlaceholderTool>().add_systems(
            Update,
            (spawn_tool_event_responder_update_system, handle_input),
        );
    }
}

#[derive(Component, Reflect)]
pub struct PlaceholderTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlaceholderToolAction {
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

    fn default_input_map() -> InputMap<PlaceholderToolAction> {
        let mut input_map = InputMap::default();

        for variant in PlaceholderToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

fn spawn_tool_event_responder_update_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        match e {
            ToolbeltEvent::Populate(toolbelt_id) => {
                commands.entity(*toolbelt_id).with_children(|t_commands| {
                    for i in 0..7 {
                        t_commands.spawn((
                            PlaceholderTool,
                            ToolBundle {
                                tool: Tool,
                                name: Name::new(format!("Placeholder Tool {}", i)),
                                sprite_bundle: SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(100.0, 100.0)),
                                        ..default()
                                    },
                                    texture: asset_server.load("textures/tool_placeholder.png"),
                                    ..default()
                                },
                                input_manager: InputManagerBundle::<PlaceholderToolAction> {
                                    input_map: PlaceholderToolAction::default_input_map(),
                                    ..default()
                                },
                            },
                        ));
                    }
                });
            }
        }
    }
}

fn handle_input(actors: Query<(&ActionState<PlaceholderToolAction>, Option<&ToolActiveTag>)>) {
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
