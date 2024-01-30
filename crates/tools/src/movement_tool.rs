use crate::tool_spawning::ToolSpawnConfig;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_xpbd_2d::math::*;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_character::character_plugin::MainCharacter;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use cursor_hero_physics::damping_plugin::DampingSystemSet;
use cursor_hero_sprint_tool_types::sprint_tool_types_plugin::SprintData;
use cursor_hero_toolbelt::types::ActiveTool;
use cursor_hero_toolbelt::types::PopulateToolbeltEvent;
use cursor_hero_toolbelt::types::ToolAction;
use cursor_hero_toolbelt::types::Toolbelt;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

pub struct MovementToolPlugin;

impl Plugin for MovementToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MovementToolAction>::default());
        app.register_type::<MovementTool>();
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, handle_inputs.after(DampingSystemSet::Dampen));
        app.add_systems(OnEnter(ActiveInput::MouseKeyboard), set_mnk_speed);
        app.add_systems(OnEnter(ActiveInput::Gamepad), set_gamepad_speed);
    }
}

#[derive(Component, Reflect, Debug, Default, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct MovementTool;

#[derive(Bundle, Debug, Reflect)]
struct MovementToolBundle {
    tool: MovementTool,
    data: SprintData,
}
impl Default for MovementToolBundle {
    fn default() -> Self {
        Self {
            tool: MovementTool::default(),
            data: match ActiveInput::default() {
                ActiveInput::MouseKeyboard => default_mnk_data(),
                ActiveInput::Gamepad => default_gamepad_data(),
            },
        }
    }
}

fn default_mnk_data() -> SprintData {
    SprintData {
        value: 8000.0,
        default_value: 8000.0,
        sprint_value: 40000.0,
        ..default()
    }
}
fn default_gamepad_data() -> SprintData {
    SprintData {
        value: 800.0,
        default_value: 800.0,
        sprint_value: 80000.0,
        ..default()
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
        let toolbelt_id = match event {
            PopulateToolbeltEvent::Default { toolbelt_id }
            | PopulateToolbeltEvent::Inspector { toolbelt_id }
            | PopulateToolbeltEvent::Taskbar { toolbelt_id }
            | PopulateToolbeltEvent::Keyboard { toolbelt_id } => toolbelt_id,
        };
        ToolSpawnConfig::<_, MovementToolAction>::new(
            MovementToolBundle::default(),
            *toolbelt_id,
            event,
        )
        .guess_name(file!())
        .guess_image(file!(), &asset_server)
        .with_description("Go faster, reach further")
        .spawn(&mut commands);
    }
}

fn handle_inputs(
    time: Res<Time>,
    tool_query: Query<(&ActionState<MovementToolAction>, &SprintData, &Parent), With<ActiveTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    mut character_query: Query<&mut LinearVelocity, With<Character>>,
) {
    let delta_time = time.delta_seconds_f64().adjust_precision();
    for (tool_actions, tool_data, tool_parent) in tool_query.iter() {
        if !tool_actions.pressed(MovementToolAction::Move) {
            continue;
        }
        let Ok(toolbelt_parent) = toolbelt_query.get(tool_parent.get()) else {
            continue;
        };
        let Ok(character) = character_query.get_mut(toolbelt_parent.get()) else {
            continue;
        };
        let mut character_velocity = character;
        let move_delta = delta_time
            * tool_actions
                .clamped_axis_pair(MovementToolAction::Move)
                .unwrap()
                .xy();
        character_velocity.x += move_delta.x * tool_data.value;
        character_velocity.y += move_delta.y * tool_data.value;
    }
}

fn set_mnk_speed(
    mut tool_query: Query<(&mut SprintData, &Parent), With<MovementTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    character_query: Query<(), With<MainCharacter>>,
) {
    for tool in tool_query.iter_mut() {
        let (mut tool_data, tool_parent) = tool;
        let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
            continue;
        };
        if character_query.get(toolbelt.get()).is_err() {
            continue;
        }
        *tool_data = default_mnk_data();
    }
}

fn set_gamepad_speed(
    mut tool_query: Query<(&mut SprintData, &Parent), With<MovementTool>>,
    toolbelt_query: Query<&Parent, With<Toolbelt>>,
    character_query: Query<(), With<MainCharacter>>,
) {
    for tool in tool_query.iter_mut() {
        let (mut tool_data, tool_parent) = tool;
        let Ok(toolbelt) = toolbelt_query.get(tool_parent.get()) else {
            continue;
        };
        if character_query.get(toolbelt.get()).is_err() {
            continue;
        }
        *tool_data = default_gamepad_data();
    }
}
