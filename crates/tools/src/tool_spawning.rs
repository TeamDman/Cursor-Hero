use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_pointer_types::prelude::*;

use cursor_hero_toolbelt_types::prelude::*;
use leafwing_input_manager::prelude::*;
use std::path::Path;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum NoInputs {}

impl ToolAction for NoInputs {
    fn default_input_map(_event: &PopulateToolbeltEvent) -> Option<InputMap<Self>> {
        None
    }
}

pub struct ToolSpawnConfig<T, Action>
where
    T: Bundle,
    Action: ToolAction + Actionlike,
{
    tag: T,
    event: PopulateToolbeltEvent,
    name: String,
    description: String,
    display_actions: HashMap<String, Vec<UserInput>>,
    texture: Handle<Image>,
    toolbelt_id: Entity,
    starting_state: StartingState,
    size: Option<Vec2>,
    input_map: Option<InputMap<Action>>,
}

impl<T, Action> ToolSpawnConfig<T, Action>
where
    T: Bundle,
    Action: ToolAction + Actionlike + core::fmt::Debug,
{
    pub fn new(tag: T, toolbelt_id: Entity, event: &PopulateToolbeltEvent) -> Self {
        Self {
            tag,
            event: *event,
            name: "Unnamed Tool".to_string(),
            description: "Who knows what this does?".to_string(),
            texture: Handle::default(),
            toolbelt_id,
            starting_state: StartingState::Active,
            size: Some(Vec2::new(100.0, 100.0)),
            input_map: None,
            display_actions: HashMap::new(),
        }
        .with_input_map(Action::default_input_map(event))
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_description(mut self, description: &'static str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn guess_name(mut self, file_path: &str) -> Self {
        self.name = Self::format_tool_name_from_source(file_path);
        self
    }

    pub fn with_input_map(mut self, input_map: Option<InputMap<Action>>) -> Self {
        self.display_actions = match input_map {
            None => HashMap::new(),
            Some(ref input_map) => input_map
                .iter()
                .map(|v| (format!("{:?}", v.0), v.1.clone()))
                .collect(),
        };
        self.input_map = input_map;
        self
    }

    fn format_tool_name_from_source(file_path: &str) -> String {
        // Extract the file name from the path
        Self::clean_name(file_path)
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

    pub fn guess_image(
        mut self,
        file_path: &str,
        asset_server: &Res<AssetServer>,
        extension: &str,
    ) -> Self {
        self.texture = asset_server.load(Self::format_tool_image_from_source(file_path, extension));
        self
    }

    fn clean_name(file_path: &str) -> &str {
        Path::new(file_path)
        .file_stem() // Get the file stem (file name without extension)
        .and_then(|stem| stem.to_str()) // Convert OsStr to &str
        .unwrap_or("")
        .trim_end_matches("_plugin")
        .trim_end_matches("_populate")
        .trim_start_matches("spawn_")
    }

    fn format_tool_image_from_source(file_path: &str, extension: &str) -> String {
        format!("textures/tools/{}.{}", Self::clean_name(file_path), extension)
    }

    pub fn with_asset_image(
        mut self,
        file_name: &'static str,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        self.texture = asset_server.load(format!("textures/tools/{}", file_name));
        self
    }

    pub fn with_image(mut self, texture: Handle<Image>) -> Self {
        self.texture = texture;
        self
    }

    pub fn with_starting_state(mut self, state: StartingState) -> Self {
        self.starting_state = state;
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = Some(size);
        self
    }

    pub fn spawn(self, commands: &mut Commands) {
        commands.entity(self.toolbelt_id).with_children(|toolbelt| {
            let mut tool = toolbelt.spawn((
                Tool {
                    name: self.name.clone(),
                    description: self.description,
                    actions: self.display_actions,
                    texture: self.texture.clone(),
                },
                self.tag,
                Name::new(self.name.clone()),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: self.size,
                        ..default()
                    },
                    texture: self.texture,
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Sensor,
                RigidBody::Kinematic,
                Hoverable,
                Collider::cuboid(100.0, 100.0),
            ));
            if let StartingState::Active = self.starting_state {
                tool.insert(ActiveTool);
            }
            let input_map = self.input_map.unwrap_or_default();
            tool.insert(InputManagerBundle {
                input_map,
                ..default()
            });
        });
        info!("{:?} => {:?}", self.event, self.name);
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_tool_impl(
    tool: Tool,
    event: &PopulateToolbeltEvent,
    commands: &mut Commands,
    toolbelt_id: Entity,
    _asset_server: &Res<AssetServer>,
    tool_component: impl Component,
    input_manager: Option<impl Bundle>,
    starting_state: StartingState,
    custom_size: Option<Vec2>,
) {
    let tool_name = tool.name.clone();
    commands.entity(toolbelt_id).with_children(|toolbelt| {
        let name = Name::new(tool_name.clone());
        let texture = tool.texture.clone();
        let mut tool = toolbelt.spawn((
            tool,
            name,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: custom_size.or(Some(Vec2::new(100.0, 100.0))),
                    ..default()
                },
                texture,
                visibility: Visibility::Hidden,
                ..default()
            },
            tool_component,
            Sensor,
            RigidBody::Kinematic,
            Collider::cuboid(100.0, 100.0),
        ));
        if let StartingState::Active = starting_state {
            tool.insert(ActiveTool);
        }
        if let Some(bundle) = input_manager {
            tool.insert(bundle);
        }
    });
    info!("{:?} => {:?}", event, tool_name);
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_action_tool<T>(
    tool: Tool,
    event: &PopulateToolbeltEvent,
    commands: &mut Commands,
    toolbelt_id: Entity,
    _character_id: Entity,
    asset_server: &Res<AssetServer>,
    tool_component: impl Component,
    starting_state: StartingState,
    custom_size: Option<Vec2>,
) where
    T: ToolAction + Actionlike,
{
    spawn_tool_impl(
        tool,
        event,
        commands,
        toolbelt_id,
        asset_server,
        tool_component,
        T::default_input_map(event).map(|input_map| InputManagerBundle::<T> {
            input_map,
            ..default()
        }),
        starting_state,
        custom_size,
    )
}

#[derive(Bundle)]
pub struct NoopBundle {}

#[derive(Debug)]
pub enum StartingState {
    Active,
    Inactive,
}
impl StartingState {
    pub fn as_active(&self) -> Option<ActiveTool> {
        match self {
            StartingState::Active => Some(ActiveTool),
            StartingState::Inactive => None,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_tool(
    tool: Tool,
    event: &PopulateToolbeltEvent,
    commands: &mut Commands,
    toolbelt_id: Entity,
    _character_id: Entity,
    asset_server: &Res<AssetServer>,
    tool_component: impl Component,
    starting_state: StartingState,
    custom_size: Option<Vec2>,
) {
    spawn_tool_impl(
        tool,
        event,
        commands,
        toolbelt_id,
        asset_server,
        tool_component,
        None::<NoopBundle>,
        starting_state,
        custom_size,
    )
}
