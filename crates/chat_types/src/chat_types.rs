use bevy::prelude::*;
use bevy::utils::HashSet;
use cursor_hero_toolbelt_types::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component, Debug, Reflect, Default)]
pub struct ChatTool {
    pub focused: bool,
    pub buffer: String,
    pub tools_disabled_during_focus: HashSet<Entity>,
}

#[derive(Component, Reflect, Default)]
pub struct ChatWheelTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ChatToolAction {
    Focus,
    Unfocus,
    Submit,
}

impl ChatToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Focus => GamepadButtonType::North.into(),
            Self::Unfocus => GamepadButtonType::East.into(),
            Self::Submit => GamepadButtonType::South.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Focus => KeyCode::Return.into(),
            Self::Unfocus => KeyCode::Escape.into(),
            Self::Submit => KeyCode::Return.into(),
        }
    }
}
impl ToolAction for ChatToolAction {
    fn default_input_map(_event: &PopulateToolbeltEvent) -> Option<InputMap<ChatToolAction>> {
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

#[derive(Bundle, Debug, Default)]
pub struct ChatInputBundle {
    pub text: Text2dBundle,
    pub background: Sprite,
    pub chat: ChatInput,
}
impl ChatInputBundle {
    pub fn new(position: Vec3, starting_text: String) -> Self {
        Self {
            text: Text2dBundle {
                text: Text::from_section(
                    starting_text,
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                transform: Transform::from_translation(position),
                ..default()
            },
            chat: ChatInput,
            background: Sprite {
                custom_size: Some(Vec2::new(300.0, 100.0)),
                color: Color::ALICE_BLUE,
                ..default()
            },
        }
    }
}

#[derive(Component, Reflect, Debug, Default)]
pub struct ChatBubble;

#[derive(Bundle, Debug, Default)]
pub struct ChatBubbleBundle {
    pub text: Text2dBundle,
    pub background: Sprite,
    pub chat: ChatBubble,
}
impl ChatBubbleBundle {
    pub fn new(position: Vec3, message: String) -> Self {
        Self {
            text: Text2dBundle {
                text: Text::from_section(
                    message,
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                transform: Transform::from_translation(position),
                ..default()
            },
            background: Sprite {
                custom_size: Some(Vec2::new(300.0, 100.0)),
                color: Color::BLACK,
                ..default()
            },
            chat: ChatBubble,
        }
    }
}
