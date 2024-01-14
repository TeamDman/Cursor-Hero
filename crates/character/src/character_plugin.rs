use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_xpbd_2d::prelude::*;

use cursor_hero_camera::camera_plugin::CameraEvent;
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut camera_events: EventWriter<CameraEvent>,
) {
    let default_material = materials.add(CharacterColor::FocusedWithCamera.as_material());
    let os_cursor_pos = get_cursor_position().expect("Should be able to fetch cursor pos from OS");
    let mut character = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Capsule {
                        radius: 12.5,
                        depth: 20.0,
                        ..default()
                    }
                    .into(),
                )
                .into(),
            material: default_material,
            transform: Transform::from_xyz(os_cursor_pos.x, -os_cursor_pos.y, 100.0),
            ..default()
        },
        Character::default(),
        MainCharacter,
        MovementDamping { factor: 0.90 },
        Name::new("Character"),
        RigidBody::Kinematic,
        Collider::capsule(20.0, 12.5),
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
    mut commands: Commands,
    mut camera_events: EventReader<CameraEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut character_query: Query<(Entity, &mut Handle<ColorMaterial>), With<Character>>,
) {
    for event in camera_events.read() {
        match event {
            CameraEvent::BeginFollowing { target_id } => {
                if let Ok((character_id, mut material)) = character_query.get_mut(*target_id) {
                    *material = materials.add(CharacterColor::FocusedWithCamera.as_material());
                    info!("Updated character color to focused");
                }
            }
            CameraEvent::StopFollowing { target_id } => {
                if let Ok((character_id, mut material)) = character_query.get_mut(*target_id) {
                    *material = materials.add(CharacterColor::Unfocused.as_material());
                    info!("Updated character color to unfocused");
                }
            }
        }
    }
}
