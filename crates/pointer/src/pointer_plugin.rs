use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::transform::TransformSystem;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_camera::camera_plugin::FollowWithCamera;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_character::character_plugin::CharacterColor;

pub struct PointerPlugin;
#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum PointerSystemSet {
    Spawn,
    Position,
}

impl Plugin for PointerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pointer>()
            .configure_sets(Update, PointerSystemSet::Position)
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Update, insert_pointer.in_set(PointerSystemSet::Spawn))
            .add_systems(
                Update,
                update_pointer_from_mouse.run_if(in_state(ActiveInput::MouseKeyboard)),
            )
            .add_systems(
                PostUpdate,
                update_pointer_position
                    .in_set(PointerSystemSet::Position)
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    Move,
}

impl Action {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Move => UserInput::Single(InputKind::DualAxis(DualAxis::right_stick())),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Move => UserInput::VirtualDPad(VirtualDPad::arrow_keys()),
        }
    }

    fn default_input_map() -> InputMap<Action> {
        let mut input_map = InputMap::default();

        for variant in Action::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Pointer {
    #[inspector(min = 0.0)]
    pub reach: f32,
    #[inspector(min = 0.0)]
    pub default_reach: f32,
    #[inspector(min = 0.0)]
    pub sprint_reach: f32,
}
impl Default for Pointer {
    fn default() -> Self {
        Self {
            reach: 50.0,
            default_reach: 50.0,
            sprint_reach: 2000.0,
        }
    }
}

fn insert_pointer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    character: Query<Entity, Added<Character>>,
) {
    for c_e in character.iter() {
        info!("Creating pointer for character '{:?}'", c_e);
        commands.entity(c_e).with_children(|c_commands| {
            c_commands.spawn((
                Pointer::default(),
                Name::new("Pointer"),
                SpriteBundle {
                    texture: asset_server.load("textures/cursor.png"),
                    transform: Transform::from_xyz(0.0, 0.0, 2.0),
                    sprite: Sprite {
                        color: CharacterColor::default().as_color(),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    ..Default::default()
                },
                InputManagerBundle::<Action> {
                    input_map: Action::default_input_map(),
                    action_state: ActionState::default(),
                },
                RigidBody::Dynamic,
                Collider::cuboid(10.0, 10.0),
                Sensor,
            ));
        });
    }
}

fn update_pointer_position(
    mut pointer_query: Query<(&mut Transform, &ActionState<Action>, &Pointer)>,
    mut debounce: Local<bool>,
) {
    for (mut pointer_transform, p_act, p) in pointer_query.iter_mut() {
        if p_act.pressed(Action::Move) {
            let look = p_act.axis_pair(Action::Move).unwrap().xy();
            if look.x.is_nan() || look.y.is_nan() {
                continue;
            }

            let desired_position = look * p.reach;
            pointer_transform.translation.x = desired_position.x;
            pointer_transform.translation.y = desired_position.y;
            *debounce = false;
        } else if !*debounce {
            pointer_transform.translation.x = 0.0;
            pointer_transform.translation.y = 0.0;
            *debounce = true;
        }
    }
}

fn update_pointer_from_mouse(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<MainCamera>, Without<Character>)>,
    follow_query: Query<(&Transform, &Children), (With<FollowWithCamera>, Without<MainCamera>, Without<Pointer>)>,
    mut pointer_query: Query<&mut Transform, (With<Pointer>, Without<FollowWithCamera>)>,
) {
    let (camera, camera_global_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(current_screen_position) = window.cursor_position() {
        // mouse is inside the window, convert to world coords
        if let Some(current_world_position) = camera
            .viewport_to_world(camera_global_transform, current_screen_position)
            .map(|ray| ray.origin.truncate())
        {
            if let Ok((character_pos, character_kids)) = follow_query.get_single() {
                // get the pointer in the character's children
                for kid in character_kids.iter() {
                    if let Ok(mut pointer) = pointer_query.get_mut(*kid) {
                        pointer.translation.x =
                            current_world_position.x - character_pos.translation.x;
                        pointer.translation.y =
                            current_world_position.y - character_pos.translation.y;
                    }
                }
            }
        }
    }
}
