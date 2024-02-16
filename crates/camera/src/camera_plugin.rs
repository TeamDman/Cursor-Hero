use bevy::ecs::query::QuerySingleError::MultipleEntities;
use bevy::ecs::query::QuerySingleError::NoEntities;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_physics::damping_plugin::MovementDamping;
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (update_camera_zoom, handle_events))
            .add_event::<CameraEvent>()
            .add_systems(
                PostUpdate,
                follow
                    .in_set(CameraSystemSet::Follow)
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            )
            .register_type::<MainCamera>();
    }
}

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum CameraSystemSet {
    Follow,
}

#[derive(Component, Reflect)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
        MovementDamping { factor: 0.90 },
        MassPropertiesBundle::new_computed(&Collider::ball(10.0), 1.0),
        RigidBody::Dynamic,
        SpatialListener::new(-7.0),
    ));
}

#[derive(Component)]
pub struct FollowWithMainCamera;

#[derive(Event, Debug, Reflect)]
pub enum CameraEvent {
    BeginFollowing { target_id: Entity },
    StopFollowing { target_id: Entity },
}

pub fn update_camera_zoom(
    mut cam: Query<&mut Transform, With<MainCamera>>,
    mut scroll: EventReader<MouseWheel>,
    egui_context_query: Query<&EguiContext, With<PrimaryWindow>>,
) {
    let Ok(egui_context) = egui_context_query.get_single() else {
        return;
    };
    let hovering_over_egui = egui_context.clone().get_mut().is_pointer_over_area();
    if hovering_over_egui {
        scroll.clear();
        return;
    }
    for event in scroll.read() {
        let mut scale = cam.single_mut().scale;
        scale *= Vec2::splat(1.0 - event.y / 10.0).extend(1.0);
        scale = scale.clamp(Vec3::splat(0.1), Vec3::splat(10.0));
        cam.single_mut().scale = scale;
    }
}

fn handle_events(
    mut commands: Commands,
    mut camera_events: EventReader<CameraEvent>,
    character_query: Query<&GlobalTransform, Without<MainCamera>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    for event in camera_events.read() {
        match event {
            CameraEvent::BeginFollowing { target_id } => {
                info!("Camera following character '{:?}'", target_id);
                // tag character to mark it as being followed
                commands.entity(*target_id).insert(FollowWithMainCamera);
                if let Ok(mut camera_transform) = camera_query.get_single_mut()
                    && let Ok(character_transform) = character_query.get_single()
                {
                    camera_transform.translation = character_transform.translation();
                }
            }
            CameraEvent::StopFollowing { target_id } => {
                info!("Camera stopped following character '{:?}'", target_id);
                // remove tag from character
                commands.entity(*target_id).remove::<FollowWithMainCamera>();
            }
        }
    }
}

fn follow(
    follow_query: Query<&GlobalTransform, With<FollowWithMainCamera>>,
    mut cam_query: Query<&mut Transform, With<MainCamera>>,
) {
    let follow = match follow_query.get_single() {
        Ok(follow) => follow,
        Err(NoEntities(_)) => return,
        Err(MultipleEntities(e)) => {
            error!("Multiple entities are being followed: {:?}", e);
            return;
        }
    };
    let follow_global_transform = follow;

    let camera = match cam_query.get_single_mut() {
        Ok(camera) => camera,
        Err(NoEntities(_)) => return,
        Err(MultipleEntities(e)) => {
            error!("Multiple cameras found: {:?}", e);
            return;
        }
    };
    let mut camera_transform = camera;

    // update transform
    let follow_translation = follow_global_transform.translation();
    camera_transform.translation = follow_translation;
}
