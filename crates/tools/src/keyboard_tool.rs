use crate::prelude::*;
use bevy::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
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

fn handle_input(tool_query: Query<&ActionState<KeyboardToolAction>, With<ActiveTool>>) {
    let mut enigo = Enigo::new();
    for tool_actions in tool_query.iter() {
        let ctrl_down = if tool_actions.pressed(KeyboardToolAction::Ctrl) {
            1
        } else {
            0
        };
        let shift_down = if tool_actions.pressed(KeyboardToolAction::Shift) {
            2
        } else {
            0
        };
        let scan: u16 = ctrl_down | shift_down;
        for variant in KeyboardToolAction::variants() {
            if tool_actions.just_pressed(variant) {
                info!("{:?} key down (scan: {:?})", variant, scan);
                enigo.key_down(variant.to_enigo());
                // TODO: fix enigo not sending shift status when sending arrow keys; selection isn't working
                // enigo.key_down_scan(variant.to_enigo(), scan);
            }
            if tool_actions.just_released(variant) {
                info!("{:?} key up (scan: {:?})", variant, scan);
                enigo.key_up(variant.to_enigo());
                // enigo.key_up_scan(variant.to_enigo(), scan);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // test that sending shift + arrow keys is highlighting text
    #[test]
    fn test_shift_arrow() {
        use enigo::*;
        use std::thread::sleep;
        use std::time::Duration;
        let mut enigo = Enigo::new();
        sleep(Duration::from_secs(3));
        enigo.key_down(Key::Shift);
        enigo.key_down(Key::Control);
        sleep(Duration::from_secs(1));
        enigo.key_down(Key::RightArrow);
        sleep(Duration::from_secs(1));
        enigo.key_up(Key::RightArrow);
        enigo.key_up(Key::Shift);
        enigo.key_up(Key::Control);
    }
}
