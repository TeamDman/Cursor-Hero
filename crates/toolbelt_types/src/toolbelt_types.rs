use std::fmt::Debug;

use bevy::prelude::*;
use bevy::utils::HashMap;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;
use serde::{Deserialize, Serialize};

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

    // TODO: convert toolbelt to normal tool structure
    pub fn default_input_map() -> InputMap<ToolbeltAction> {
        let mut input_map = InputMap::default();

        for variant in ToolbeltAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

#[derive(Component, Reflect, Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Toolbelt {
    pub open: bool,
    pub loadout: ToolbeltLoadout,
    pub layout: ToolbeltLayout,
}
impl Default for Toolbelt {
    fn default() -> Self {
        Self {
            open: false,
            loadout: ToolbeltLoadout::Default,
            layout: ToolbeltLoadout::Default.layout(),
        }
    }
}

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
            toolbelt: Toolbelt::default(),
        }
    }
}

#[derive(Reflect, Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum ToolbeltLoadout {
    #[default]
    Default,
    Inspector,
    Taskbar,
    Keyboard,
    Agent,
    Chat,
    WindowPosition,
}
impl ToolbeltLoadout {
    pub fn layout(&self) -> ToolbeltLayout {
        match self {
            Self::Taskbar => ToolbeltLayout::Taskbar {
                wheel: Wheel::default(),
            },
            _ => ToolbeltLayout::default(),
        }
    }
}

#[derive(Reflect, Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum ToolbeltLayout {
    Circle { wheel: Wheel },
    Taskbar { wheel: Wheel },
}
impl Default for ToolbeltLayout {
    fn default() -> Self {
        Self::Circle {
            wheel: Wheel::default(),
        }
    }
}

#[derive(Event, Debug, Reflect, Clone, Copy)]
pub struct ToolbeltPopulateEvent {
    pub id: Entity,
    pub loadout: ToolbeltLoadout,
}

#[derive(Event, Debug, Reflect)]
pub enum ToolbeltOpeningEvent {
    Opened { toolbelt_id: Entity },
    Closed { toolbelt_id: Entity },
}

#[derive(Component, Reflect, Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Wheel {
    pub radius: f32,
    pub radius_start: f32,
    pub radius_end: f32,
    pub radius_end_bonus_per_tool_after_8: f32,
    pub spin: f32,
    pub spin_start: f32,
    pub spin_end: f32,
    pub scale: f32,
    pub scale_start: f32,
    pub scale_end: f32,
    pub alpha: f32,
    pub alpha_start: f32,
    pub alpha_end: f32,
    pub open: bool,
}
impl Default for Wheel {
    fn default() -> Self {
        Self {
            radius: 200.0,
            // min_radius: 50.0,
            radius_start: 200.0,
            radius_end: 200.0,
            radius_end_bonus_per_tool_after_8: 10.0,
            spin: 0.0,
            spin_start: 300.0,
            spin_end: 360.0,
            scale: 1.0,
            scale_start: 0.5,
            scale_end: 1.0,
            alpha: 0.0,
            alpha_start: 0.0,
            alpha_end: 1.0,
            open: false,
        }
    }
}

#[derive(Component, Reflect, Clone, Debug)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub actions: HashMap<String, Vec<UserInput>>,
    pub texture: Handle<Image>,
}

#[derive(Component, Reflect, Clone, Copy, Debug)]
pub struct ToolHelpTrigger;

pub trait ToolAction: Actionlike {
    fn default_input_map(_event: &ToolbeltPopulateEvent) -> Option<InputMap<Self>>;
    fn with_defaults<G, K>(gamepad: G, keyboard: K) -> InputMap<Self>
    where
        G: Fn(&Self) -> UserInput,
        K: Fn(&Self) -> UserInput,
        Self: Clone,
    {
        let mut input_map = InputMap::default();

        for variant in Self::variants() {
            let g = gamepad(&variant);
            let k = keyboard(&variant);
            input_map.insert(g, variant.clone());
            input_map.insert(k, variant);
        }
        input_map
    }
}

#[derive(Component, Reflect, Debug, PartialEq, Eq)]
pub struct ActiveTool;

#[derive(Event, Debug, Reflect)]
pub enum ToolActivationEvent {
    Activate(Entity),
    Deactivate(Entity),
}

#[derive(Component, Reflect, Debug)]
pub struct ToolHelp {
    pub timer: Timer,
}
