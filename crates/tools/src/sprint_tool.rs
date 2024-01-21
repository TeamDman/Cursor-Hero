use bevy::prelude::*;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_movement::Movement;
use leafwing_input_manager::prelude::*;

use cursor_hero_character::character_plugin::Character;
use cursor_hero_pointer::pointer_plugin::Pointer;

use cursor_hero_toolbelt::types::*;

use crate::prelude::*;

pub struct SprintToolPlugin;

impl Plugin for SprintToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SprintTool>()
            .register_type::<SpawnedCube>()
            .add_plugins(InputManagerPlugin::<SprintToolAction>::default())
            .add_systems(Update, (toolbelt_events, handle_input));
    }
}

#[derive(Component, Reflect)]
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
    fn default_input_map() -> InputMap<SprintToolAction> {
        let mut input_map = InputMap::default();

        for variant in SprintToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for event in reader.read() {
        if let ToolbeltEvent::PopulateDefaultToolbelt {
            toolbelt_id,
            character_id,
        }
        | ToolbeltEvent::PopulateInspectorToolbelt {
            toolbelt_id,
            character_id,
        } = event
        {
            spawn_action_tool::<SprintToolAction>(
                Tool::create_with_actions::<SprintToolAction>(
                    file!(),
                    "Go faster, reach further".to_string(),
                    &asset_server,
                ),
                event,
                &mut commands,
                *toolbelt_id,
                *character_id,
                &asset_server,
                SprintTool,
                StartingState::Active,
            );
        }
    }
}

#[derive(Component, Reflect)]
pub struct SpawnedCube;

fn handle_input(
    tools: Query<(&ActionState<SprintToolAction>, Option<&ActiveTool>, &Parent)>,
    toolbelts: Query<(&Wheel, &Parent), With<Toolbelt>>,
    mut character_query: Query<
        (&mut Character, Option<&mut Movement>, &Children),
        Without<MainCamera>,
    >,
    mut pointer_query: Query<&mut Pointer>,
    mut camera_query: Query<Option<&mut Movement>, (With<MainCamera>, Without<Character>)>,
) {
    for (t_act, t_enabled, t_parent) in tools.iter() {
        if t_enabled.is_none() {
            continue;
        }

        let (wheel, toolbelt_parent) = toolbelts
            .get(t_parent.get())
            .expect("Toolbelt should have a parent");

        if let Ok((mut character, movement, character_kids)) =
            character_query.get_mut(toolbelt_parent.get())
        {
            let pointer = character_kids
                .iter()
                .find(|e| pointer_query.get(**e).is_ok());
            if t_act.pressed(SprintToolAction::Sprint) {
                let mut open = t_act.value(SprintToolAction::Sprint);
                open = open.powf(2.0);

                match movement {
                    Some(mut movement) => {
                        movement.speed = movement.sprint_speed
                            + (movement.default_speed - movement.sprint_speed) * (1.0 - open);
                    }
                    None => {
                        if let Some(mut movement) = camera_query.single_mut() {
                            movement.speed = movement.sprint_speed
                                + (movement.default_speed - movement.sprint_speed) * (1.0 - open);
                        }
                    }
                }
                character.zoom_speed = character.zoom_sprint_speed
                    + (character.zoom_default_speed - character.zoom_sprint_speed) * (1.0 - open);

                if !wheel.open {
                    if let Some(Ok(mut pointer)) = pointer.map(|e| pointer_query.get_mut(*e)) {
                        pointer.reach = pointer.default_reach
                            + (pointer.sprint_reach - pointer.default_reach) * open;
                    }
                }
            } else if t_act.just_released(SprintToolAction::Sprint) {
                match movement {
                    Some(mut movement) => {
                        movement.speed = movement.default_speed;
                    }
                    None => {
                        if let Some(mut movement) = camera_query.single_mut() {
                            movement.speed = movement.default_speed;
                        }
                    }
                }
                character.zoom_speed = character.zoom_default_speed;
                if !wheel.open {
                    if let Some(Ok(mut pointer)) = pointer.map(|e| pointer_query.get_mut(*e)) {
                        pointer.reach = pointer.default_reach;
                    }
                }
            }
        }
    }
}
