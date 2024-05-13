use bevy::prelude::*;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_character_types::prelude::*;
use cursor_hero_math::prelude::Lerp;
use cursor_hero_sprint_tool_types::sprint_tool_types_plugin::SprintEvent;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_zoom_tool_types::prelude::*;
use itertools::Itertools;
use leafwing_input_manager::prelude::*;

pub struct ZoomToolTickPlugin;

impl Plugin for ZoomToolTickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ZoomToolAction>::default());
        app.add_systems(Update, tick);
        app.add_systems(Update, handle_sprint_events);
    }
}

fn tick(
    tool_query: Query<(&ActionState<ZoomToolAction>, &ZoomTool), With<ActiveTool>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    for zoom_tool in tool_query.iter() {
        let (tool_actions, tool) = zoom_tool;
        let camera = camera_query.single_mut();
        let mut camera_transform = camera;
        if tool_actions.pressed(ZoomToolAction::Out) {
            let mut scale = camera_transform.scale;
            let diff = 0.1 * time.delta_seconds() * tool.speed;
            scale *= Vec3::splat(1.0) + Vec2::splat(diff).extend(0.0);
            // debug!(
            //     "scale_min: {}, scale_max: {}",
            //     tool.scale_min, tool.scale_max
            // );
            scale = scale.clamp(Vec3::splat(tool.scale_min), Vec3::splat(tool.scale_max));
            camera_transform.scale = scale;
            if tool_actions.just_pressed(ZoomToolAction::Out) {
                info!("Zooming out");
            }
        }
        if tool_actions.pressed(ZoomToolAction::In) {
            let mut scale = camera_transform.scale;
            let diff = 0.1 * time.delta_seconds() * tool.speed;
            scale *= Vec3::splat(1.0) - Vec2::splat(diff).extend(0.0);
            debug!(
                "scale_min: {}, scale_max: {}",
                tool.scale_min, tool.scale_max
            );
            scale = scale.clamp(Vec3::splat(tool.scale_min), Vec3::splat(tool.scale_max));
            camera_transform.scale = scale;
            if tool_actions.just_pressed(ZoomToolAction::In) {
                info!("Zooming in");
            }
        }
    }
}

fn handle_sprint_events(
    mut sprint_events: EventReader<SprintEvent>,
    character_query: Query<&Children, With<Character>>,
    toolbelt_query: Query<&Children, With<Toolbelt>>,
    mut tool_query: Query<&mut ZoomTool>,
) {
    for event in sprint_events.read() {
        let character_id = match event {
            SprintEvent::Active { character_id, .. } => character_id,
            SprintEvent::Stop { character_id } => character_id,
        };
        let Ok(character) = character_query.get(*character_id) else {
            warn!("Character {:?} does not exist", character_id);
            continue;
        };
        let character_kids = character;
        let tool_ids = character_kids
            .iter()
            .filter_map(|kid| toolbelt_query.get(*kid).ok())
            .flat_map(|toolbelt| toolbelt.iter())
            .filter(|kid| tool_query.contains(**kid))
            .cloned()
            .collect_vec();

        match event {
            SprintEvent::Active { throttle, .. } => {
                let mut iter = tool_query.iter_many_mut(&tool_ids);
                while let Some(mut tool) = iter.fetch_next() {
                    tool.speed = (tool.default_speed, tool.sprint_speed).lerp(*throttle);
                }
            }
            SprintEvent::Stop { .. } => {
                let mut iter = tool_query.iter_many_mut(&tool_ids);
                while let Some(mut tool) = iter.fetch_next() {
                    tool.speed = tool.default_speed;
                }
            }
        }
    }
}
