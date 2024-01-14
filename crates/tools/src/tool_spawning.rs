use crate::tool_naming::format_tool_image_from_source;
use crate::tool_naming::format_tool_name_from_source;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_toolbelt::types::ActiveTool;
use cursor_hero_toolbelt::types::ToolAction;
use cursor_hero_toolbelt::types::ToolBundle;
use cursor_hero_toolbelt::types::ToolJoint;
use cursor_hero_toolbelt::types::ToolbeltEvent;
use leafwing_input_manager::prelude::*;

fn spawn_tool_impl(
    source_path: &str,
    event: &ToolbeltEvent,
    commands: &mut Commands,
    toolbelt_id: Entity,
    character_id: Entity,
    asset_server: &Res<AssetServer>,
    tool_component: impl Component,
    input_manager: Option<impl Bundle>,
) {
    let name = format_tool_name_from_source(source_path);
    commands.entity(toolbelt_id).with_children(|toolbelt| {
        let mut tool = toolbelt.spawn((
            ToolBundle {
                name: Name::new(name),
                sprite_bundle: SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(100.0, 100.0)),
                        ..default()
                    },
                    texture: asset_server.load(format_tool_image_from_source(source_path)),
                    ..default()
                },
                ..default()
            },
            tool_component,
            Sensor,
            RigidBody::Dynamic,
            Collider::cuboid(100.0, 100.0),
            ActiveTool,
        ));
        if let Some(bundle) = input_manager {
            tool.insert(bundle);
        }
        let tool_id = tool.id();
        toolbelt.spawn((
            FixedJoint::new(character_id, tool_id).with_compliance(0.0),
            ToolJoint,
        ));
    });
    info!(
        "{:?} => {:?}",
        event,
        format_tool_name_from_source(source_path)
    );
}

pub fn spawn_action_tool<T>(
    source_path: &str,
    event: &ToolbeltEvent,
    commands: &mut Commands,
    toolbelt_id: Entity,
    character_id: Entity,
    asset_server: &Res<AssetServer>,
    tool_component: impl Component,
) where
    T: ToolAction + Actionlike,
{
    spawn_tool_impl(
        source_path,
        event,
        commands,
        toolbelt_id,
        character_id,
        asset_server,
        tool_component,
        Some(InputManagerBundle::<T> {
            input_map: T::default_input_map(),
            ..default()
        }),
    )
}

#[derive(Bundle)]
struct WeAintGotNoBundle {}

pub fn spawn_tool(
    source_path: &str,
    event: &ToolbeltEvent,
    commands: &mut Commands,
    toolbelt_id: Entity,
    character_id: Entity,
    asset_server: &Res<AssetServer>,
    tool_component: impl Component,
) {
    spawn_tool_impl(
        source_path,
        event,
        commands,
        toolbelt_id,
        character_id,
        asset_server,
        tool_component,
        None::<WeAintGotNoBundle>,
    )
}
