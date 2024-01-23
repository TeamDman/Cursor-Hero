use std::fmt::Debug;

use bevy::prelude::*;
use bevy::utils::HashMap;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;
use std::path::Path;

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

#[derive(Event, Debug, Reflect)]
pub enum ToolbeltPopulateEvent {
    Default {
        toolbelt_id: Entity,
        character_id: Entity,
    },
    Inspector {
        toolbelt_id: Entity,
        character_id: Entity,
    },
    Taskbar {
        toolbelt_id: Entity,
        character_id: Entity,
    },
}


#[derive(Event, Debug, Reflect)]
pub enum ToolbeltStateEvent {
    Opened {
        toolbelt_id: Entity,
        character_id: Entity,
    },
    Closed {
        toolbelt_id: Entity,
        character_id: Entity,
    },
}

#[derive(Component, Reflect, Clone, Copy, Debug)]
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

impl Tool {
    pub fn actions_as_info<T>() -> HashMap<String, Vec<UserInput>>
    where
        T: ToolAction + Actionlike + Debug,
    {
        T::default_input_map()
            .iter()
            .map(|v| (format!("{:?}", v.0), v.1.clone()))
            .collect()
    }

    pub fn create_with_actions<T>(
        source_file_path: &str,
        description: String,
        asset_server: &Res<AssetServer>,
    ) -> Tool
    where
        T: ToolAction + Actionlike + Debug,
    {
        let name = Self::format_tool_name_from_source(source_file_path);
        let texture = asset_server.load(Self::format_tool_image_from_source(source_file_path));
        let actions = Self::actions_as_info::<T>();
        Self {
            name,
            description,
            actions,
            texture,
        }
    }

    pub fn create(
        source_file_path: &str,
        description: String,
        asset_server: &Res<AssetServer>,
    ) -> Tool {
        // TODO: structural edit, make first param name; use jetbrains IDE to do this
        let name = Self::format_tool_name_from_source(source_file_path);
        let texture = asset_server.load(Self::format_tool_image_from_source(source_file_path));
        let actions = HashMap::default();
        Self {
            name,
            description,
            actions,
            texture,
        }
    }

    pub fn new(
        name: String,
        description: String,
        actions: HashMap<String, Vec<UserInput>>,
        texture: Handle<Image>,
    ) -> Self {
        Self {
            name,
            description,
            actions,
            texture,
        }
    }

    fn format_tool_name_from_source(file_path: &str) -> String {
        // Extract the file name from the path
        let file_name = Path::new(file_path)
            .file_stem() // Get the file stem (file name without extension)
            .and_then(|stem| stem.to_str()) // Convert OsStr to &str
            .unwrap_or("");

        file_name
            .split('_')
            .map(|word| {
                word.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i == 0 {
                            c.to_uppercase().to_string()
                        } else {
                            c.to_string()
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn format_tool_image_from_source(file_path: &str) -> String {
        // Extract the file name from the path
        let file_name = Path::new(file_path)
            .file_stem() // Get the file stem (file name without extension)
            .and_then(|stem| stem.to_str()) // Convert OsStr to &str
            .unwrap_or("")
            .trim_end_matches("_plugin");
        format!("textures/tools/{}.png", file_name)
    }
}

#[derive(Component, Reflect, Clone, Copy, Debug)]
pub struct ToolHelpTrigger;

pub trait ToolAction: Actionlike {
    fn default_input_map() -> InputMap<Self>;
}

#[derive(Component, Reflect, Debug)]
pub struct ActiveTool;

#[derive(Component, Reflect, Debug)]
pub struct Hovered;

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

#[derive(Component, Reflect, Debug)]
pub struct ToolHelp {
    pub timer: Timer,
}
