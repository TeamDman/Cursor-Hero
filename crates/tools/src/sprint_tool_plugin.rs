use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use cursor_hero_character::character_plugin::Character;
use cursor_hero_pointer::pointer_plugin::Pointer;

use cursor_hero_toolbelt::types::*;

pub struct SprintToolPlugin;

impl Plugin for SprintToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SprintTool>()
            .register_type::<SpawnedCube>()
            .add_plugins(InputManagerPlugin::<SprintToolAction>::default())
            .add_systems(
                Update,
                (spawn_tool_event_responder_update_system, handle_input),
            );
    }
}

#[derive(Component, Reflect)]
pub struct SprintTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum SprintToolAction {
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

    fn default_input_map() -> InputMap<SprintToolAction> {
        let mut input_map = InputMap::default();

        for variant in SprintToolAction::variants() {
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
                            name: Name::new("Sprint Tool"),
                            sprite_bundle: SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(100.0, 100.0)),
                                    ..default()
                                },
                                texture: asset_server.load("textures/sprint.png"),
                                ..default()
                            },
                            ..default()
                        },
                        InputManagerBundle::<SprintToolAction> {
                            input_map: SprintToolAction::default_input_map(),
                            ..default()
                        },
                        SprintTool,
                        ToolActiveTag,
                    ));
                });
                info!("Added tool to toolbelt {:?}", toolbelt_id);
            }
        }
    }
}

#[derive(Component, Reflect)]
pub struct SpawnedCube;

fn handle_input(
    tools: Query<(
        &ActionState<SprintToolAction>,
        Option<&ToolActiveTag>,
        &Parent,
    )>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    mut character_query: Query<(&mut Character, &Children)>,
    mut pointer_query: Query<&mut Pointer>,
) {
    for (t_act, t_enabled, t_parent) in tools.iter() {
        if t_enabled.is_none() {
            continue;
        }

        let belt_parent = toolbelts
            .get(t_parent.get())
            .expect("Toolbelt should have a parent")
            .get();
        if let Ok((mut character, character_kids)) = character_query.get_mut(belt_parent) {
            let pointer = character_kids
                .iter()
                .find(|e| pointer_query.get(**e).is_ok());
            if t_act.pressed(SprintToolAction::Sprint) {
                let open = t_act.value(SprintToolAction::Sprint);
                character.speed = character.sprint_speed
                    + (character.default_speed - character.sprint_speed) * (1.0 - open);
                character.zoom_speed = character.zoom_sprint_speed
                    + (character.zoom_default_speed - character.zoom_sprint_speed) * (1.0 - open);

                if let Some(Ok(mut pointer)) = pointer.map(|e| pointer_query.get_mut(*e)) {
                    pointer.reach = pointer.default_reach
                        + (pointer.sprint_reach - pointer.default_reach) * open;
                }
            } else if t_act.just_released(SprintToolAction::Sprint) {
                character.speed = character.default_speed;
                character.zoom_speed = character.zoom_default_speed;

                if let Some(Ok(mut pointer)) = pointer.map(|e| pointer_query.get_mut(*e)) {
                    pointer.reach = pointer.default_reach;
                }
            }
        }
    }
}
