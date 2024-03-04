use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use cursor_hero_character_types::prelude::*;
use cursor_hero_math::Lerp;
use cursor_hero_sprint_tool_types::sprint_tool_types_plugin::SprintEvent;
use cursor_hero_toolbelt_types::prelude::*;
use enigo::Direction::Press;
use enigo::Direction::Release;
use enigo::*;
use itertools::Itertools;
use leafwing_input_manager::prelude::*;

pub struct KeyboardToolPlugin;

impl Plugin for KeyboardToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<KeyboardToolAction>::default());
        app.add_systems(
            Update,
            (toolbelt_events, handle_input, handle_sprint_events),
        );
    }
}

#[derive(Component, InspectorOptions, Debug, Reflect)]
#[reflect(Component, InspectorOptions)]
struct KeyboardTool {
    #[inspector(min = 0.0)]
    repeat_delay: f32,
    #[inspector(min = 0.0)]
    default_repeat_delay: f32,
    #[inspector(min = 0.0)]
    sprint_repeat_delay: f32,
}
impl Default for KeyboardTool {
    fn default() -> Self {
        Self {
            repeat_delay: 0.1,
            default_repeat_delay: 0.1,
            sprint_repeat_delay: 0.001,
        }
    }
}

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if event.loadout != ToolbeltLoadout::Keyboard {
            continue;
        }
        ToolSpawnConfig::<KeyboardTool, KeyboardToolAction>::new(
            KeyboardTool::default(),
            event.id,
            event,
        )
        .guess_name(file!())
        .guess_image(file!(), &asset_server, "png")
        .with_description("Keyboard inputs")
        .spawn(&mut commands);
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum KeyboardToolAction {
    Ctrl,
    Tab,
    Enter,
    Backspace,
    // Escape,
    Shift,
    Space,
    Windows,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
}

impl KeyboardToolAction {
    fn to_enigo(self) -> Key {
        match self {
            Self::Ctrl => Key::Control,
            Self::Tab => Key::Tab,
            Self::Enter => Key::Return,
            Self::Backspace => Key::Backspace,
            // Self::Escape => Key::Escape,
            Self::Shift => Key::Shift,
            Self::Space => Key::Space,
            Self::Windows => Key::Meta,
            Self::UpArrow => Key::UpArrow,
            Self::DownArrow => Key::DownArrow,
            Self::LeftArrow => Key::LeftArrow,
            Self::RightArrow => Key::RightArrow,
        }
    }
}

impl KeyboardToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Ctrl => GamepadButtonType::RightTrigger.into(),
            Self::Tab => GamepadButtonType::West.into(),
            Self::Enter => GamepadButtonType::North.into(),
            Self::Backspace => GamepadButtonType::East.into(),
            // Self::Escape => GamepadButtonType::Select.into(),
            Self::Shift => GamepadButtonType::LeftTrigger.into(),
            Self::Space => GamepadButtonType::South.into(),
            Self::Windows => GamepadButtonType::Start.into(),
            Self::UpArrow => GamepadButtonType::DPadUp.into(),
            Self::DownArrow => GamepadButtonType::DPadDown.into(),
            Self::LeftArrow => GamepadButtonType::DPadLeft.into(),
            Self::RightArrow => GamepadButtonType::DPadRight.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Ctrl => KeyCode::ControlLeft.into(),
            Self::Tab => KeyCode::Tab.into(),
            Self::Enter => KeyCode::Return.into(),
            Self::Backspace => KeyCode::Back.into(),
            // Self::Escape => KeyCode::Escape.into(),
            Self::Shift => KeyCode::ShiftLeft.into(),
            Self::Space => KeyCode::Space.into(),
            Self::Windows => KeyCode::SuperLeft.into(),
            Self::UpArrow => KeyCode::Up.into(),
            Self::DownArrow => KeyCode::Down.into(),
            Self::LeftArrow => KeyCode::Left.into(),
            Self::RightArrow => KeyCode::Right.into(),
        }
    }
}

impl ToolAction for KeyboardToolAction {
    fn default_input_map(_event: &PopulateToolbeltEvent) -> Option<InputMap<KeyboardToolAction>> {
        let mut input_map = InputMap::default();

        for variant in KeyboardToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}

fn handle_input(
    tool_query: Query<(Entity, &ActionState<KeyboardToolAction>, &KeyboardTool), With<ActiveTool>>,
    mut enigo: Local<Option<Enigo>>,
    time: Res<Time>,
    mut debounce: Local<HashMap<(Entity, KeyboardToolAction), Timer>>,
) {
    debounce.values_mut().for_each(|timer| {
        timer.tick(time.delta());
    });
    debounce.retain(|_, timer| !timer.finished());

    if enigo.is_none() {
        *enigo = Enigo::new(&Settings::default()).ok();
    }
    let Some(ref mut enigo) = *enigo else {
        warn!("Failed to create enigo");
        return;
    };

    for tool in tool_query.iter() {
        let (tool_id, tool_actions, tool) = tool;
        for variant in KeyboardToolAction::variants() {
            if tool_actions.pressed(variant) {
                if tool_actions.just_pressed(variant) {
                    info!("{:?} key down", variant);
                }
                if (*debounce).contains_key(&(tool_id, variant)) {
                    continue;
                } else {
                    debounce.insert(
                        (tool_id, variant),
                        Timer::from_seconds(tool.repeat_delay, TimerMode::Once),
                    );
                }
                if let Err(e) = enigo.key(variant.to_enigo(), Press) {
                    warn!("Failed to send key: {:?}", e);
                }
            }
            if tool_actions.just_released(variant) {
                info!("{:?} key up", variant);
                if let Err(e) = enigo.key(variant.to_enigo(), Release) {
                    warn!("Failed to send key: {:?}", e);
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    // test that sending shift + arrow keys is highlighting text
    #[test]
    fn test_shift_arrow() {
        use enigo::Direction::Press;
        use enigo::Direction::Release;
        use enigo::Enigo;
        use enigo::Key;
        use enigo::Keyboard;
        use enigo::Settings;
        use std::thread::sleep;
        use std::time::Duration;

        let mut enigo = Enigo::new(&Settings::default()).unwrap();

        sleep(Duration::from_secs(1));
        enigo.key(Key::Shift, Press).unwrap();
        enigo.key(Key::Control, Press).unwrap();
        enigo.key(Key::RightArrow, Press).unwrap();

        enigo.key(Key::RightArrow, Release).unwrap();
        enigo.key(Key::Control, Release).unwrap();
        enigo.key(Key::Shift, Release).unwrap();
    }
}

fn handle_sprint_events(
    mut sprint_events: EventReader<SprintEvent>,
    character_query: Query<&Children, With<Character>>,
    toolbelt_query: Query<&Children, With<Toolbelt>>,
    mut tool_query: Query<&mut KeyboardTool>,
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
                    tool.repeat_delay =
                        (tool.default_repeat_delay, tool.sprint_repeat_delay).lerp(*throttle);
                }
            }
            SprintEvent::Stop { .. } => {
                let mut iter = tool_query.iter_many_mut(&tool_ids);
                while let Some(mut tool) = iter.fetch_next() {
                    tool.repeat_delay = tool.default_repeat_delay;
                }
            }
        }
    }
}
