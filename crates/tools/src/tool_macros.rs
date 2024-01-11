use crate::tool_naming::format_tool_image_from_source;
use crate::tool_naming::format_tool_name_from_source;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_toolbelt::types::ToolAction;
use cursor_hero_toolbelt::types::ToolActiveTag;
use cursor_hero_toolbelt::types::ToolBundle;
use cursor_hero_toolbelt::types::ToolbeltEvent;
use leafwing_input_manager::prelude::*;

pub fn spawn_action_tool_impl<T>(
    event: &ToolbeltEvent,
    commands: &mut Commands,
    toolbelt_id: Entity,
    asset_server: &Res<AssetServer>,
    tool_component: impl Component,
) where
    T: ToolAction + Actionlike,
{
    let name = format_tool_name_from_source(file!());
    commands.entity(toolbelt_id).with_children(|t_commands| {
        t_commands.spawn((
            ToolBundle {
                name: Name::new(name),
                sprite_bundle: SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(100.0, 100.0)),
                        ..default()
                    },
                    texture: asset_server.load(format_tool_image_from_source(file!())),
                    ..default()
                },
                ..default()
            },
            tool_component,
            ToolActiveTag,
            InputManagerBundle::<T> {
                input_map: T::default_input_map(),
                ..default()
            },
        ));
    });
    info!("{:?} => {:?}", event, format_tool_name_from_source(file!()));
}

pub fn spawn_tool_impl(
    event: &ToolbeltEvent,
    commands: &mut Commands,
    toolbelt_id: Entity,
    asset_server: &Res<AssetServer>,
    tool_component: impl Component,
) {
    let name = format_tool_name_from_source(file!());
    commands.entity(toolbelt_id).with_children(|t_commands| {
        let mut bundle = ToolBundle {
            name: Name::new(name),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                texture: asset_server.load(format_tool_image_from_source(file!())),
                ..default()
            },
            ..default()
        };

        let builder = t_commands
            .spawn(bundle)
            .insert(tool_component)
            .insert(ToolActiveTag);
    });
    info!("{:?} => {:?}", event, format_tool_name_from_source(file!()));
}

#[macro_export]
macro_rules! spawn_tool {
    ($event:expr, $commands:expr, $toolbelt_id:expr, $asset_server:expr, $tool_component:expr) => {{
        spawn_tool_impl(
            $event,
            $commands,
            $toolbelt_id,
            $asset_server,
            $tool_component,
        );
    }};
}

#[macro_export]
macro_rules! spawn_action_tool {
    ($event:expr, $commands:expr, $toolbelt_id:expr, $asset_server:expr, $tool_component:expr, $tool_actions:ty) => {{
        spawn_action_tool_impl::<$tool_actions>(
            $event,
            $commands,
            $toolbelt_id,
            $asset_server,
            $tool_component,
        );
    }};
}
