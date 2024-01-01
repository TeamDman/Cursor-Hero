use bevy::{input::mouse::MouseWheel, prelude::*, transform::TransformSystem};
use bevy_xpbd_2d::PhysicsSet;
use leafwing_input_manager::{
    action_state::ActionState, input_map::InputMap, plugin::InputManagerPlugin, Actionlike,
    InputManagerBundle, user_input::InputKind,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraAction>::default())
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    update_camera_zoom,
                ),
            )
            .add_systems(
                PostUpdate,
                camera_follow_update
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
        InputManagerBundle::<CameraAction> {
            input_map,
            action_state: ActionState::default(),
        },
    ));
}

#[derive(Component)]
pub struct FollowWithCamera;

pub fn update_camera_zoom(
    mut cam: Query<&mut Transform, With<MainCamera>>,
    mut scroll: EventReader<MouseWheel>,
) {
    for event in scroll.read() {
        let mut scale = cam.single_mut().scale;
        scale *= Vec3::splat(1.0 - event.y / 10.0);
        scale = scale.clamp(Vec3::splat(0.1), Vec3::splat(10.0));
        cam.single_mut().scale = scale;
    }
}

fn camera_follow_update(
    mut cam: Query<&mut Transform, With<MainCamera>>,
    follow: Query<&Transform, (With<FollowWithCamera>, Without<MainCamera>)>, // we exclude the camera to guarantee queries are disjoint
) {
    if let Ok(follow) = follow.get_single() {
        cam.single_mut().translation = follow.translation;
    } else if follow.iter().len() != 0 {
        panic!("Multiple entities with FollowWithCamera component");
    }
}
