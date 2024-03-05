use bevy::prelude::*;
use bevy::utils::HashSet;
use cursor_hero_toolbelt_types::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component, Debug, Reflect, Default)]
pub struct ChatTool {
    pub focused: bool,
    pub buffer: String,
    pub tools_disabled_during_focus: HashSet<Entity>,
    pub state: ChatToolState,
}

#[derive(PartialEq, Reflect, Debug, Default, Clone)]
pub enum ChatToolState {
    #[default]
    Idle,
    InitialRepeatDelay(Timer),
    RepeatDelay(Timer),
}

#[derive(Component, Reflect, Default)]
pub struct ChatWheelTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ChatToolAction {
    Focus,
    Unfocus,
    Submit,
    WordModifier,
    Backspace,
}

impl ChatToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Focus => GamepadButtonType::North.into(),
            Self::Unfocus => GamepadButtonType::East.into(),
            Self::Submit => GamepadButtonType::South.into(),
            Self::WordModifier => GamepadButtonType::LeftTrigger.into(),
            Self::Backspace => GamepadButtonType::West.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Focus => KeyCode::Return.into(),
            Self::Unfocus => KeyCode::Escape.into(),
            Self::Submit => KeyCode::Return.into(),
            Self::WordModifier => KeyCode::ControlLeft.into(),
            Self::Backspace => KeyCode::Back.into(),
        }
    }
}
impl ToolAction for ChatToolAction {
    fn default_input_map(_event: &ToolbeltPopulateEvent) -> Option<InputMap<ChatToolAction>> {
        let mut input_map = InputMap::default();

        for variant in ChatToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}

#[derive(Event, PartialEq, Eq, Clone, Hash, Debug, Reflect)]
pub enum ChatEvent {
    Chat {
        character_id: Entity,
        message: String,
    },
}

#[derive(Event, PartialEq, Eq, Clone, Hash, Debug, Reflect)]
pub enum ChatInputEvent {
    Focus {
        tool_id: Entity,
        toolbelt_id: Entity,
        character_id: Entity,
    },
    Unfocus {
        tool_id: Entity,
        toolbelt_id: Entity,
        character_id: Entity,
    },
    TextChanged {
        character_id: Entity,
        toolbelt_id: Entity,
        tool_id: Entity,
    },
}

#[derive(Component, Reflect, Debug, Default)]
pub struct ChatInput;

#[derive(Component, Reflect, Debug, Default)]
pub struct ChatBubble {
    pub lifetime: Timer,
}
