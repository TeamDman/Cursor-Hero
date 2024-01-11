use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use itertools::Itertools;

use cursor_hero_character::character_plugin::Character;
use cursor_hero_pointer::pointer_plugin::Pointer;
use cursor_hero_pointer::pointer_plugin::PointerSystemSet;
use cursor_hero_toolbelt::types::*;
use cursor_hero_winutils::win_mouse::set_cursor_position;

use crate::prelude::*;

pub struct CursorMonitorPositionToolPlugin;

impl Plugin for CursorMonitorPositionToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CursorMonitorPositionTool>()
            .add_systems(Update, toolbelt_events)
            .add_systems(
                PostUpdate,
                snap_mouse_to_pointer
                    .run_if(in_state(ActiveInput::Gamepad))
                    .after(PointerSystemSet::Position)
                    .after(PhysicsSet::Sync)
                    .after(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Component, Reflect)]
struct CursorMonitorPositionTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        match e {
            ToolbeltEvent::PopulateDefaultToolbelt(toolbelt_id) => {
                spawn_tool(
                    file!(),
                    e,
                    &mut commands,
                    *toolbelt_id,
                    &asset_server,
                    CursorMonitorPositionTool,
                );
            }
            _ => {}
        }
    }
}

fn snap_mouse_to_pointer(
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<(Ref<GlobalTransform>, &Children), With<Character>>,
    pointers: Query<Ref<GlobalTransform>, With<Pointer>>,
    tools: Query<(Option<&ToolActiveTag>, &Parent), With<CursorMonitorPositionTool>>,
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
