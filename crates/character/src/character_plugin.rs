use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_xpbd_2d::prelude::*;

use cursor_hero_camera::camera_plugin::CameraEvent;
use cursor_hero_bevy::NegativeYVec2;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use cursor_hero_movement::Movement;
use cursor_hero_physics::damping_plugin::MovementDamping;
use cursor_hero_winutils::win_mouse::get_cursor_position;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum CharacterSystemSet {
    Spawn,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, CharacterSystemSet::Spawn)
            .add_systems(Startup, spawn_character.in_set(CharacterSystemSet::Spawn))
            .add_systems(Update, handle_camera_events)
            .add_systems(OnEnter(ActiveInput::MouseKeyboard), set_mnk_speed)
            .add_systems(OnEnter(ActiveInput::Gamepad), set_gamepad_speed)
            .register_type::<Character>();
    }
}

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Character {
    #[inspector(min = 0.0)]
    pub zoom_speed: f32,
    #[inspector(min = 0.0)]
    pub zoom_default_speed: f32,
    #[inspector(min = 0.0)]
    pub zoom_sprint_speed: f32,
}

#[derive(Component)]
pub struct MainCharacter;

impl Default for Character {
    fn default() -> Self {
        Self {
            zoom_speed: 1.0,
            zoom_default_speed: 1.0,
            zoom_sprint_speed: 150.0,
        }
    }
}

#[derive(Component, Reflect, Eq, PartialEq, Debug)]
pub enum CharacterColor {
    Unfocused,
    FocusedWithCamera,
}
impl Default for CharacterColor {
    fn default() -> Self {
        Self::FocusedWithCamera
    }
}

impl CharacterColor {
    pub fn as_color(self) -> Color {
        match self {
            Self::Unfocused => Color::rgb(0.2, 0.7, 0.9),
            Self::FocusedWithCamera => Color::rgb(0.149, 0.549, 0.184),
        }
    }
    pub fn as_material(self) -> ColorMaterial {
        ColorMaterial::from(self.as_color())
    }
}

fn spawn_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut camera_events: EventWriter<CameraEvent>,
) {
    let default_material = materials.add(CharacterColor::FocusedWithCamera.as_material());
    let os_cursor_pos = get_cursor_position().expect("Should be able to fetch cursor pos from OS");
    let character = commands.spawn((
        // MaterialMesh2dBundle {
        //     mesh: meshes
        //         .add(
        //             shape::Capsule {
        //                 radius: 12.5,
        //                 depth: 20.0,
        //                 ..default()
        //             }
        //             .into(),
        //         )
        //         .into(),
        //     material: default_material,
        //     transform: Transform::from_translation(os_cursor_pos.as_vec2().neg_y().extend(100.0)),
        //     ..default()
        // },
        SpriteBundle {
            texture: asset_server.load("textures/character/default_character.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(os_cursor_pos.as_vec2().neg_y().extend(100.0)),
            ..default()
        },
        Character::default(),
        MainCharacter,
        MovementDamping { factor: 0.90 },
        Name::new("Character"),
        RigidBody::Kinematic,
        Collider::capsule(15.0, 12.5),
        SpatialListener::new(7.0),
        Movement::default(),
    ));
    if CharacterColor::default() == CharacterColor::FocusedWithCamera {
        camera_events.send(CameraEvent::BeginFollowing {
            target_id: character.id(),
        });
    }
    info!("Spawned character");
}

fn handle_camera_events(
    mut camera_events: EventReader<CameraEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut character_query: Query<&mut Handle<ColorMaterial>, With<Character>>,
) {
    for event in camera_events.read() {
        match event {
            CameraEvent::BeginFollowing { target_id } => {
                if let Ok(mut material) = character_query.get_mut(*target_id) {
                    *material = materials.add(CharacterColor::FocusedWithCamera.as_material());
                    info!("Updated character color to focused");
                }
            }
            CameraEvent::StopFollowing { target_id } => {
                if let Ok(mut material) = character_query.get_mut(*target_id) {
                    *material = materials.add(CharacterColor::Unfocused.as_material());
                    info!("Updated character color to unfocused");
                }
            }
        }
    }
}

fn set_mnk_speed(mut query: Query<&mut Movement, With<MainCharacter>>) {
    for mut movement in &mut query.iter_mut() {
        *movement = Movement::default_mnk();
    }
}

fn set_gamepad_speed(mut query: Query<&mut Movement, With<MainCharacter>>) {
    for mut movement in &mut query.iter_mut() {
        *movement = Movement::default_gamepad();
    }
}
