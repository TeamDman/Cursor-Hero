use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ToolbeltAction {
    Show,
}

impl ToolbeltAction {
    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Show => UserInput::Single(InputKind::Keyboard(KeyCode::AltLeft)),
        }
    }
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Show => GamepadButtonType::RightTrigger2.into(),
        }
    }
    pub fn default_input_map() -> InputMap<ToolbeltAction> {
        let mut input_map = InputMap::default();

        for variant in ToolbeltAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

#[derive(Component, Reflect, Clone, Copy, Debug)]
pub struct Toolbelt;

#[derive(Bundle)]
pub struct ToolbeltBundle {
    pub circle: Wheel,
    pub spatial: SpatialBundle,
    pub name: Name,
    pub input_manager: InputManagerBundle<ToolbeltAction>,
    pub toolbelt: Toolbelt,
}
impl Default for ToolbeltBundle {
    fn default() -> Self {
        Self {
            circle: Wheel::default(),
            spatial: SpatialBundle {
                visibility: Visibility::Hidden,
                ..default()
            },
            name: Name::new("Toolbelt"),
            input_manager: InputManagerBundle::<ToolbeltAction> {
                input_map: ToolbeltAction::default_input_map(),
                ..Default::default()
            },
            toolbelt: Toolbelt,
        }
    }
}

// #[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]

// pub enum ToolbeltSystemSet {
//     Spawn,
// }

#[derive(Event, Debug, Reflect)]
pub enum ToolbeltEvent {
    EquipDefaultToolbelt(Entity),
    PopulateDefaultToolbelt(Entity),
    EquipInspectorToolbelt(Entity),
    PopulateInspectorToolbelt(Entity),
}

#[derive(Component, Reflect, Clone, Copy, Debug)]
pub struct Wheel {
    pub radius: f32,
    pub radius_start: f32,
    pub radius_end: f32,
    pub spin: f32,
    pub spin_start: f32,
    pub spin_end: f32,
    pub scale: f32,
    pub scale_start: f32,
    pub scale_end: f32,
}
impl Default for Wheel {
    fn default() -> Self {
        Self {
            radius: 200.0,
            // min_radius: 50.0,
            radius_start: 200.0,
            radius_end: 200.0,
            spin: 0.0,
            spin_start: 300.0,
            spin_end: 360.0,
            scale: 1.0,
            scale_start: 0.5,
            scale_end: 1.0,
        }
    }
}

#[derive(Component, Reflect, Clone, Copy, Debug)]
pub struct Tool;

pub trait ToolAction: Actionlike {
    fn default_input_map() -> InputMap<Self>;
}

#[derive(Bundle)]
pub struct ToolBundle {
    pub name: Name,
    pub tool: Tool,
    pub sprite_bundle: SpriteBundle,
}
impl Default for ToolBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Unnamed Tool"),
            tool: Tool,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Component, Reflect, Debug)]
pub struct ToolActiveTag;

#[derive(Component, Reflect, Debug)]
pub struct ToolHoveredTag;

#[derive(Event, Debug, Reflect)]
pub enum ToolHoveredEvent {
    HoverStart(Entity),
    HoverEnd(Entity),
}

#[derive(Event, Debug, Reflect)]
pub enum ToolActivationEvent {
    Activate(Entity),
    Deactivate(Entity),
}
