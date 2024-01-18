use bevy::ecs::query::QuerySingleError::MultipleEntities;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_physics::damping_plugin::MovementDamping;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::user_input::InputKind;
use leafwing_input_manager::Actionlike;
use leafwing_input_manager::InputManagerBundle;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraAction>::default())
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (update_camera_zoom, handle_events))
            .add_event::<CameraEvent>()
            .add_systems(
                PostUpdate,
                follow
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            )
            .register_type::<MainCamera>();
    }
}

#[derive(Component, Reflect)]
pub struct MainCamera;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CameraAction {
    ToggleFollowCharacter,
}

fn spawn_camera(mut commands: Commands) {
    let mut input_map = InputMap::default();
    input_map.insert(KeyCode::Space, CameraAction::ToggleFollowCharacter);
    input_map.insert(
        InputKind::GamepadButton(GamepadButtonType::North),
        CameraAction::ToggleFollowCharacter,
    );
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
        MovementDamping { factor: 0.90 },
        MassPropertiesBundle::new_computed(&Collider::ball(10.0), 1.0),
        RigidBody::Dynamic,
        InputManagerBundle::<CameraAction> {
            input_map,
            action_state: ActionState::default(),
        },
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
) {
    for event in scroll.read() {
        let mut scale = cam.single_mut().scale;
        scale *= Vec2::splat(1.0 - event.y / 10.0).extend(1.0);
        scale = scale.clamp(Vec3::splat(0.1), Vec3::splat(10.0));
        cam.single_mut().scale = scale;
    }
}

fn handle_events(mut commands: Commands, mut camera_events: EventReader<CameraEvent>) {
    for event in camera_events.read() {
        match event {
            CameraEvent::BeginFollowing { target_id } => {
                info!("Camera following character '{:?}'", target_id);
                // tag character to mark it as being followed
                commands.entity(*target_id).insert(FollowWithMainCamera);
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
    match follow_query.get_single() {
        Ok(follow_transform) => match cam_query.get_single_mut() {
            Ok(mut cam_transform) => {
                cam_transform.translation = follow_transform.translation();
            }
            Err(e) => {
                if let MultipleEntities(e) = e {
                    error!("Error getting camera transform: {:?}", e);
                }
            }
        },
        Err(e) => {
            if let MultipleEntities(e) = e {
                error!("Error getting camera follow transform: {:?}", e);
            }
        }
    }
}
