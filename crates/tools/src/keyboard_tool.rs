use bevy::prelude::*;
use cursor_hero_toolbelt::types::ActiveTool;
use cursor_hero_toolbelt::types::ToolAction;
use leafwing_input_manager::prelude::*;
use cursor_hero_toolbelt::types::*;
use crate::prelude::*;
use enigo::*;

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
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for event in reader.read() {
        if let ToolbeltPopulateEvent::Keyboard {
            toolbelt_id,
        } = event
        {
            ToolSpawnConfig::<KeyboardTool, KeyboardToolAction>::new(KeyboardTool, *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Keyboard inputs")
                .spawn(&mut commands);
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum KeyboardToolAction {
    CtrlKey,
    TabKey,
    EnterKey,
    BackspaceKey,
    EscapeKey,
    ShiftKey,
    SpaceKey,
    WindowsKey,
}

impl KeyboardToolAction {
    fn to_enigo(&self) -> Key {
        match self {
            Self::CtrlKey => Key::Control,
            Self::TabKey => Key::Tab,
            Self::EnterKey => Key::Return,
            Self::BackspaceKey => Key::Backspace,
            Self::EscapeKey => Key::Escape,
            Self::ShiftKey => Key::Shift,
            Self::SpaceKey => Key::Space,
            Self::WindowsKey => Key::Meta,
        }
    }
}

impl KeyboardToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::CtrlKey => GamepadButtonType::RightTrigger.into(),
            Self::TabKey => GamepadButtonType::West.into(),
            Self::EnterKey => GamepadButtonType::North.into(),
            Self::BackspaceKey => GamepadButtonType::East.into(),
            Self::EscapeKey => GamepadButtonType::Select.into(),
            Self::ShiftKey => GamepadButtonType::LeftTrigger.into(),
            Self::SpaceKey => GamepadButtonType::South.into(),
            Self::WindowsKey => GamepadButtonType::Start.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::CtrlKey => KeyCode::ControlLeft.into(),
            Self::TabKey => KeyCode::Tab.into(),
            Self::EnterKey => KeyCode::Return.into(),
            Self::BackspaceKey => KeyCode::Back.into(),
            Self::EscapeKey => KeyCode::Escape.into(),
            Self::ShiftKey => KeyCode::ShiftLeft.into(),
            Self::SpaceKey => KeyCode::Space.into(),
            Self::WindowsKey => KeyCode::SuperLeft.into(),
        }
    }
}

impl ToolAction for KeyboardToolAction {
    fn default_input_map(_event: &ToolbeltPopulateEvent) -> Option<InputMap<KeyboardToolAction>> {
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
        for variant in KeyboardToolAction::variants() {
            if tool_actions.just_pressed(variant) {
                info!("{:?} key down", variant);
                enigo.key_down(variant.to_enigo());
            }
            if tool_actions.just_released(variant) {
                info!("{:?} key up", variant);
                enigo.key_up(variant.to_enigo());
            }
        }
    }
}
