use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_math::Lerp;
use cursor_hero_sprint_tool_types::sprint_tool_types_plugin::SprintEvent;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_winutils::win_mouse::scroll_wheel;
use itertools::Itertools;
use leafwing_input_manager::prelude::*;

use crate::prelude::*;

pub struct ScrollToolPlugin;

impl Plugin for ScrollToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ScrollTool>()
            .add_plugins(InputManagerPlugin::<ScrollToolAction>::default())
            .add_systems(Update, (toolbelt_events, handle_input,handle_sprint_events));
    }
}

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
struct ScrollTool {
    #[inspector(min = 0.0)]
    pub speed: f32,
    #[inspector(min = 0.0)]
    pub default_speed: f32,
    #[inspector(min = 0.0)]
    pub sprint_speed: f32,
}
impl Default for ScrollTool {
    fn default() -> Self {
        Self {
            speed: 1.0,
            default_speed: 1.0,
            sprint_speed: 100.0,
        }
    }
}

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Default { toolbelt_id }
        | PopulateToolbeltEvent::Inspector { toolbelt_id } = event
        {
            ToolSpawnConfig::<_, ScrollToolAction>::new(ScrollTool::default(), *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Send scroll events")
                .spawn(&mut commands);
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum ScrollToolAction {
    ScrollUp,
    ScrollDown,
}

impl ScrollToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::ScrollUp => GamepadButtonType::DPadUp.into(),
            Self::ScrollDown => GamepadButtonType::DPadDown.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::ScrollUp => KeyCode::PageDown.into(),
            Self::ScrollDown => KeyCode::PageUp.into(),
        }
    }
}
impl ToolAction for ScrollToolAction {
    fn default_input_map(_event: &PopulateToolbeltEvent) -> Option<InputMap<ScrollToolAction>> {
        let mut input_map = InputMap::default();

        for variant in ScrollToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}

fn handle_input(
    tool_query: Query<(&ActionState<ScrollToolAction>, &ScrollTool), With<ActiveTool>>,
    time: Res<Time>,
) {
    for zoom_tool in tool_query.iter() {
        let (tool_actions, tool) = zoom_tool;
        let mut amount = None;
        if tool_actions.pressed(ScrollToolAction::ScrollUp) {
            if tool_actions.just_pressed(ScrollToolAction::ScrollUp) {
                info!("Scrolling up");
            }
            amount = Some(tool.speed * time.delta_seconds());
        }
        if tool_actions.pressed(ScrollToolAction::ScrollDown) {
            if tool_actions.just_pressed(ScrollToolAction::ScrollDown) {
                info!("Scrolling down");
            }
            amount = Some(-tool.speed * time.delta_seconds());
        }
        if let Some(amount) = amount {
            match scroll_wheel(amount) {
                Ok(_) => {}
                Err(e) => {
                    error!("Error scrolling up: {:?}", e);
                }
            }
        }
    }
}



fn handle_sprint_events(
    mut sprint_events: EventReader<SprintEvent>,
    character_query: Query<&Children, With<Character>>,
    toolbelt_query: Query<&Children, With<Toolbelt>>,
    mut tool_query: Query<&mut ScrollTool>,
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
