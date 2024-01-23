use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_toolbelt::types::*;
use leafwing_input_manager::prelude::*;

fn spawn_tool_impl(
    tool: Tool,
    event: &ToolbeltPopulateEvent,
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

pub fn spawn_action_tool<T>(
    tool: Tool,
    event: &ToolbeltPopulateEvent,
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
        Some(InputManagerBundle::<T> {
            input_map: T::default_input_map(),
            ..default()
        }),
        starting_state,
        custom_size
    )
}

#[derive(Bundle)]
struct WeAintGotNoBundle {}

pub enum StartingState {
    Active,
    Inactive,
}

pub fn spawn_tool(
    tool: Tool,
    event: &ToolbeltPopulateEvent,
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
        None::<WeAintGotNoBundle>,
        starting_state,
        custom_size
    )
}
