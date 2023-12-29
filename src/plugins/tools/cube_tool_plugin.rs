use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::super::toolbelt::types::*;

pub struct CubeToolPlugin;

impl Plugin for CubeToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CubeTool>()
            .add_plugins(InputManagerPlugin::<CubeToolAction>::default())
            .add_systems(
                Update,
                (spawn_tool_event_responder_update_system, handle_input),
            );
    }
}

#[derive(Component, Reflect)]
pub struct CubeTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CubeToolAction {
    SpawnCube,
    RemoveCube,
    AttractCube,
}

impl CubeToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::SpawnCube => GamepadButtonType::South.into(),
            Self::RemoveCube => GamepadButtonType::East.into(),
            Self::AttractCube => GamepadButtonType::West.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::SpawnCube => KeyCode::ControlLeft.into(),
            Self::RemoveCube => KeyCode::ControlRight.into(),
            Self::AttractCube => KeyCode::AltRight.into(),
        }
    }

    fn default_input_map() -> InputMap<CubeToolAction> {
        let mut input_map = InputMap::default();

        for variant in CubeToolAction::variants() {
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
                    t_commands.spawn((
                        ToolBundle {
                            tool: Tool,
                            name: Name::new(format!("Cube Tool")),
                            input_manager: InputManagerBundle::<CubeToolAction> {
                                input_map: CubeToolAction::default_input_map(),
                                ..default()
                            },
                            sprite_bundle: SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(100.0, 100.0)),
                                    ..default()
                                },
                                texture: asset_server.load("textures/tool_bulb.png"),
                                ..default()
                            },
                        },
                        CubeTool,
                    ));
                });
                info!("Added cube tool to toolbelt {:?}", toolbelt_id);
            }
        }
    }
}

fn handle_input(
    // mut commands: Commands,
    actors: Query<(&ActionState<CubeToolAction>, Option<&ToolActiveTag>)>,
) {
    for (action_state, active_tool_tag) in actors.iter() {
        if active_tool_tag.is_none() {
            continue;
        }
        if action_state.just_pressed(CubeToolAction::SpawnCube) {
            info!("Just pressed Spawn Cube");
        }
        if action_state.just_pressed(CubeToolAction::RemoveCube) {
            info!("Just pressed Remove Cube");
        }
        if action_state.just_pressed(CubeToolAction::AttractCube) {
            info!("Just pressed Attract Cube");
        }
    }
}
