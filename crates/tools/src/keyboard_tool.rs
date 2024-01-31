use crate::prelude::*;
use bevy::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use enigo::Direction::Press;
use enigo::Direction::Release;
use enigo::*;
use leafwing_input_manager::prelude::*;

pub struct KeyboardToolPlugin;

impl Plugin for KeyboardToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<KeyboardToolAction>::default())
            .add_systems(Update, (toolbelt_events, handle_input));
    }
}

#[derive(Component, Reflect, Default)]
struct KeyboardTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Keyboard { toolbelt_id } = event {
            ToolSpawnConfig::<KeyboardTool, KeyboardToolAction>::new(
                KeyboardTool,
                *toolbelt_id,
                event,
            )
            .guess_name(file!())
            .guess_image(file!(), &asset_server)
            .with_description("Keyboard inputs")
            .spawn(&mut commands);
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum KeyboardToolAction {
    Ctrl,
    Tab,
    Enter,
    Backspace,
    Escape,
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
            Self::Escape => Key::Escape,
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
            Self::Escape => GamepadButtonType::Select.into(),
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
            Self::Escape => KeyCode::Escape.into(),
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
    tool_query: Query<&ActionState<KeyboardToolAction>, With<ActiveTool>>,
    mut enigo: Local<Option<Enigo>>,
    mut cooldown: Local<Option<Timer>>,
    time: Res<Time>,
    mut debounce: Local<bool>,
) {
    if cooldown.is_none() {
        *cooldown = Some(Timer::from_seconds(0.1, TimerMode::Repeating));
    }
    let Some(ref mut cooldown) = *cooldown else {
        warn!("Failed to create cooldown timer");
        return;
    };
    cooldown.tick(time.delta());
    if cooldown.finished() {
        *debounce = false;
    }

    if enigo.is_none() {
        *enigo = Enigo::new(&Settings::default()).ok();
    }
    let Some(ref mut enigo) = *enigo else {
        warn!("Failed to create enigo");
        return;
    };

    for tool_actions in tool_query.iter() {
        for variant in KeyboardToolAction::variants() {
            if tool_actions.pressed(variant) {
                if *debounce {
                    continue;
                }
                *debounce = true;
                info!("{:?} key down", variant);
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
