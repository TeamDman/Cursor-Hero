use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::transform::TransformSystem;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_character::character_plugin::MainCharacter;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use cursor_hero_character::character_plugin::Character;
use cursor_hero_character::character_plugin::CharacterAppearance;

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
            .add_plugins(InputManagerPlugin::<PointerAction>::default())
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
pub enum PointerAction {
    Move,
}

impl PointerAction {
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

    // TODO: convert pointer to normal tool structure
    fn default_input_map() -> InputMap<PointerAction> {
        let mut input_map = InputMap::default();

        for variant in PointerAction::variants() {
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
    for character_id in character.iter() {
        info!("Creating pointer for character '{:?}'", character_id);
        commands.entity(character_id).with_children(|parent| {
            parent.spawn((
                Pointer::default(),
                Name::new("Pointer"),
                SpriteBundle {
                    texture: asset_server.load("textures/cursor.png"),
                    transform: Transform::from_xyz(0.0, 0.0, 2.0),
                    sprite: Sprite {
                        color: Color::rgb(0.149, 0.549, 0.184),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    ..Default::default()
                },
                InputManagerBundle::<PointerAction> {
                    input_map: PointerAction::default_input_map(),
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
    mut pointer_query: Query<
        (
            &mut Position,
            &ActionState<PointerAction>,
            &Pointer,
            &Parent,
        ),
        Without<Character>,
    >,
    mut character_query: Query<&Position, (With<Character>, Without<Pointer>)>,
    mut debounce: Local<bool>,
) {
    for (mut pointer_position, pointer_actions, pointer_id, pointer_parent) in
        pointer_query.iter_mut()
    {
        let character_position = character_query.get_mut(pointer_parent.get()).unwrap();
        if pointer_actions.pressed(PointerAction::Move) {
            let look = pointer_actions.axis_pair(PointerAction::Move).unwrap().xy();
            if look.x.is_nan() || look.y.is_nan() {
                continue;
            }

            let offset = look * pointer_id.reach;
            let desired_position = character_position.xy() + offset;
            pointer_position.x = desired_position.x;
            pointer_position.y = desired_position.y;
            *debounce = false;
        } else if !*debounce {
            let desired_position = character_position.xy();
            pointer_position.x = desired_position.x;
            pointer_position.y = desired_position.y;
            *debounce = true;
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_pointer_from_mouse(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<MainCamera>, Without<Character>)>,
    character_query: Query<&Children, (With<MainCharacter>, Without<MainCamera>, Without<Pointer>)>,
    mut pointer_query: Query<&mut Position, With<Pointer>>,
) {
    let (camera, camera_global_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(current_screen_position) = window.cursor_position() {
        // mouse is inside the window, convert to world coords
        if let Some(current_world_position) = camera
            .viewport_to_world(camera_global_transform, current_screen_position)
            .map(|ray| ray.origin.truncate())
        {
            if let Ok(character_children) = character_query.get_single() {
                for child in character_children.iter() {
                    if let Ok(mut pointer_position) = pointer_query.get_mut(*child) {
                        pointer_position.x = current_world_position.x;
                        pointer_position.y = current_world_position.y;
                    }
                }
            }
        }
    }
}
