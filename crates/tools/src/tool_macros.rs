#[macro_export]
macro_rules! spawn_tool {
    ($e:expr, $commands:expr, $toolbelt_id:expr, $asset_server:expr, $tool_component:expr) => {{
        let name = format_tool_name_from_source(file!());
        $commands.entity($toolbelt_id).with_children(|t_commands| {
            t_commands.spawn((
                ToolBundle {
                    name: Name::new(name),
                    sprite_bundle: SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(100.0, 100.0)),
                            ..default()
                        },
                        texture: $asset_server
                            .load(format_tool_image_from_source(file!())),
                        ..default()
                    },
                    ..default()
                },
                $tool_component,
                ToolActiveTag,
            ));
        });
        info!(
            "{:?} => {:?}",
            $e,
            format_tool_name_from_source(file!())
        );
    }};
}

#[macro_export]
macro_rules! spawn_action_tool {
    ($e:expr, $commands:expr, $toolbelt_id:expr, $asset_server:expr, $tool_component:expr, $tool_actions:ty) => {{
        let name = format_tool_name_from_source(file!());
        $commands.entity($toolbelt_id).with_children(|t_commands| {
            t_commands.spawn((
                ToolBundle {
                    name: Name::new(name),
                    sprite_bundle: SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(100.0, 100.0)),
                            ..default()
                        },
                        texture: $asset_server
                            .load(format_tool_image_from_source(file!())),
                        ..default()
                    },
                    ..default()
                },
                $tool_component,
                ToolActiveTag,
                InputManagerBundle::<$tool_actions> {
                    input_map: <$tool_actions>::default_input_map(),
                    ..default()
                },
            ));
        });
        info!(
            "{:?} <= {:?}",
            $e,
            format_tool_name_from_source(file!())
        );
    }};
}
