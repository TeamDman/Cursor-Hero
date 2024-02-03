use crate::tool_spawning::ToolSpawnConfig;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_xpbd_2d::math::*;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use cursor_hero_math::Lerp;
use cursor_hero_physics::damping_plugin::DampingSystemSet;
use cursor_hero_sprint_tool_types::sprint_tool_types_plugin::SprintEvent;
use cursor_hero_toolbelt_types::prelude::*;
use itertools::Itertools;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

pub struct MovementToolPlugin;

impl Plugin for MovementToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MovementToolAction>::default());
        app.register_type::<MovementTool>();
        app.add_event::<MovementTargetEvent>();
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, handle_sprint_events);
        app.add_systems(Update, handle_set_movement_events);
        app.add_systems(Update, handle_inputs.after(DampingSystemSet::Dampen));
        app.add_systems(OnEnter(ActiveInput::MouseKeyboard), set_mnk_speed);
        app.add_systems(OnEnter(ActiveInput::Gamepad), set_gamepad_speed);
    }
}

#[derive(Component, Reflect, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct MovementTool {
    #[inspector(min = 0.0)]
    speed: f32,
    #[inspector(min = 0.0)]
    default_speed: f32,
    #[inspector(min = 0.0)]
    sprint_speed: f32,
    target: MovementTarget,
}

#[derive(Reflect, Debug, Clone, Copy)]
pub enum MovementTarget {
    Character,
    Camera(Entity),
}

#[derive(Event, Debug, Reflect)]
pub enum MovementTargetEvent {
    SetTarget {
        tool_id: Entity,
        target: MovementTarget,
    },
}

impl Default for MovementTool {
    fn default() -> Self {
        match ActiveInput::default() {
            ActiveInput::MouseKeyboard => default_mnk(),
            ActiveInput::Gamepad => default_gamepad(),
        }
    }
}

fn default_mnk() -> MovementTool {
    MovementTool {
        speed: 8000.0,
        default_speed: 8000.0,
        sprint_speed: 40000.0,
        target: MovementTarget::Character,
    }
}
fn default_gamepad() -> MovementTool {
    MovementTool {
        speed: 800.0,
        default_speed: 800.0,
        sprint_speed: 80000.0,
        target: MovementTarget::Character,
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum MovementToolAction {
    Move,
}

impl MovementToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Move => UserInput::Single(InputKind::DualAxis(DualAxis::left_stick())),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Move => UserInput::VirtualDPad(VirtualDPad::wasd()),
        }
    }
}
impl ToolAction for MovementToolAction {
    fn default_input_map(_event: &PopulateToolbeltEvent) -> Option<InputMap<MovementToolAction>> {
        let mut input_map = InputMap::default();

        for variant in MovementToolAction::variants() {
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
        match event {
            PopulateToolbeltEvent::Default { toolbelt_id }
            | PopulateToolbeltEvent::Inspector { toolbelt_id }
            | PopulateToolbeltEvent::Taskbar { toolbelt_id }
            | PopulateToolbeltEvent::Keyboard { toolbelt_id } => {
                ToolSpawnConfig::<_, MovementToolAction>::new(
                    MovementTool::default(),
                    *toolbelt_id,
                    event,
                )
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Go faster, reach further")
                .spawn(&mut commands);
            }
            PopulateToolbeltEvent::Agent { toolbelt_id } => {
                ToolSpawnConfig::<_, MovementToolAction>::new(
                    MovementTool::default(),
                    *toolbelt_id,
                    event,
                )
                .with_input_map(None)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Go faster, reach further")
                .spawn(&mut commands);
            }
        }
    }
}

fn handle_set_movement_events(
    mut movement_target_events: EventReader<MovementTargetEvent>,
    mut tool_query: Query<&mut MovementTool>,
) {
    for event in movement_target_events.read() {
        match event {
            MovementTargetEvent::SetTarget { tool_id, target } => {
                let Ok(mut tool) = tool_query.get_mut(*tool_id) else {
                    warn!("Tool {:?} does not exist", tool_id);
                    continue;
                };
                tool.target = *target;
            }
        }
    }
}

fn handle_inputs(
    time: Res<Time<Physics>>,
    tool_query: Query<(&ActionState<MovementToolAction>, &MovementTool, &Parent), With<ActiveTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    mut character_query: Query<&mut LinearVelocity, (With<Character>, Without<Camera>)>,
    mut camera_query: Query<&mut LinearVelocity, (With<Camera>, Without<Character>)>,
) {
    let delta_time = time.delta_seconds_f64().adjust_precision();
    for tool in tool_query.iter() {
        let (tool_actions, tool, tool_parent) = tool;
        if !tool_actions.pressed(MovementToolAction::Move) {
            continue;
        }
        let Ok(toolbelt_parent) = toolbelt_query.get(tool_parent.get()) else {
            continue;
        };
        let move_delta = delta_time
            * tool_actions
                .clamped_axis_pair(MovementToolAction::Move)
                .unwrap()
                .xy();
        match tool.target {
            MovementTarget::Character => {
                let Ok(character) = character_query.get_mut(toolbelt_parent.get()) else {
                    warn!("Character {:?} does not exist", toolbelt_parent);
                    continue;
                };
                let mut character_velocity = character;
                character_velocity.x += move_delta.x * tool.speed;
                character_velocity.y += move_delta.y * tool.speed;
            }
            MovementTarget::Camera(camera_id) => {
                let Ok(camera) = camera_query.get_mut(camera_id) else {
                    warn!("Camera {:?} does not exist", camera_id);
                    continue;
                };
                let mut camera_velocity = camera;
                camera_velocity.x += move_delta.x * tool.speed;
                camera_velocity.y += move_delta.y * tool.speed;
            }
        }
    }
}

fn set_mnk_speed(
    mut tool_query: Query<(&mut MovementTool, &Parent), With<MovementTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    character_query: Query<(), With<MainCharacter>>,
) {
    for tool in tool_query.iter_mut() {
        let (mut tool, tool_parent) = tool;
        let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
            continue;
        };
        if character_query.get(toolbelt.get()).is_err() {
            continue;
        }
        *tool = MovementTool {
            target: tool.target,
            ..default_mnk()
        };
    }
}

fn set_gamepad_speed(
    mut tool_query: Query<(&mut MovementTool, &Parent), With<MovementTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    character_query: Query<(), With<MainCharacter>>,
) {
    for tool in tool_query.iter_mut() {
        let (mut tool, tool_parent) = tool;
        let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
            continue;
        };
        if character_query.get(toolbelt.get()).is_err() {
            continue;
        }
        *tool = MovementTool {
            target: tool.target,
            ..default_gamepad()
        };
    }
}

fn handle_sprint_events(
    mut sprint_events: EventReader<SprintEvent>,
    character_query: Query<&Children, With<Character>>,
    toolbelt_query: Query<&Children, With<Toolbelt>>,
    mut tool_query: Query<&mut MovementTool>,
) {
    for event in sprint_events.read() {
        let character_id = match event {
            SprintEvent::Active { character_id, .. } => character_id,
            SprintEvent::Stop { character_id } => character_id,
        };
        let Ok(character) = character_query.get(*character_id) else {
            warn!("Character {:?} does not exist", character_id);
            continue;
        };
        let character_kids = character;

        let tool_ids = character_kids
            .iter()
            .filter_map(|kid| toolbelt_query.get(*kid).ok())
            .flat_map(|toolbelt| toolbelt.iter())
            .filter(|kid| tool_query.contains(**kid))
            .cloned()
            .collect_vec();

        match event {
            SprintEvent::Active { throttle, .. } => {
                let mut iter = tool_query.iter_many_mut(&tool_ids);
                while let Some(mut tool) = iter.fetch_next() {
                    tool.speed = (tool.default_speed, tool.sprint_speed).lerp(*throttle);
                }
            }
            SprintEvent::Stop { .. } => {
                let mut iter = tool_query.iter_many_mut(&tool_ids);
                while let Some(mut tool) = iter.fetch_next() {
                    tool.speed = tool.default_speed;
                }
            }
        }
    }
}
