use bevy::{
    prelude::*,
    transform::TransformSystem,
};
use bevy_xpbd_2d::prelude::*;
use itertools::Itertools;

use crate::{
    plugins::{
        character_plugin::Character,
        pointer_plugin::{Pointer, PointerSystemSet},
    },
    utils::win_mouse::set_cursor_position,
};

use super::super::toolbelt::types::*;

pub struct PointerScreenPositionToolPlugin;

impl Plugin for PointerScreenPositionToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PointerScreenPositionTool>()
            .add_systems(Update, spawn_tool_event_responder_update_system)
            .add_systems(
                PostUpdate,
                snap_mouse_to_pointer
                    .after(PointerSystemSet::Position)
                    .after(PhysicsSet::Sync)
                    .after(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Component, Reflect)]
pub struct PointerScreenPositionTool;

fn spawn_tool_event_responder_update_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        match e {
            ToolbeltEvent::Populate(toolbelt_id) => {
                commands.entity(*toolbelt_id).with_children(|t_commands| {
                    t_commands.spawn((
                        ToolBundle {
                            name: Name::new("Pointer Screen Position Tool"),
                            sprite_bundle: SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(100.0, 100.0)),
                                    ..default()
                                },
                                texture: asset_server
                                    .load("textures/pointer_window_position_tool.png"),
                                ..default()
                            },
                            ..default()
                        },
                        PointerScreenPositionTool,
                        // ToolActiveTag,
                    ));
                });
                info!("Added tool to toolbelt {:?}", toolbelt_id);
            }
        }
    }
}

fn snap_mouse_to_pointer(
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<(Ref<GlobalTransform>, &Children), With<Character>>,
    pointers: Query<Ref<GlobalTransform>, With<Pointer>>,
    tools: Query<(Option<&ToolActiveTag>, &Parent), With<PointerScreenPositionTool>>,
) {
    // ensure only a single cursor positioning tool is active
    let active = tools
        .iter()
        .filter(|(t_active, _)| t_active.is_some())
        .collect_vec();
    let active_count = active.len();
    if active_count > 1 {
        warn!("Only one cursor positioning tool should be active at a time");
    }
    if active_count == 0 {
        return;
    }

    // get the pointer position
    let (c_pos, c_kids) = characters
        .get(
            toolbelts
                .get(active.first().unwrap().1.get())
                .expect("Toolbelt should have a parent")
                .get(),
        )
        .expect("Toolbelt should have a character");
    let p_pos = c_kids
        .iter()
        .filter_map(|x| pointers.get(*x).ok())
        .next()
        .expect("Character should have a pointer");

    // ensure a change has occurred
    if !p_pos.is_changed() && !c_pos.is_changed() {
        return;
    }

    let t = p_pos.translation();
    if set_cursor_position(t.x as i32, -t.y as i32).is_err() {
        warn!("Failed to set cursor position");
    }
}
