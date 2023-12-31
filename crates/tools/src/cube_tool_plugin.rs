use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use cursor_hero_character::character_plugin::Character;
use cursor_hero_physics::damping_plugin::MovementDamping;
use cursor_hero_pointer::pointer_plugin::Pointer;

use cursor_hero_toolbelt::types::*;

pub struct CubeToolPlugin;

impl Plugin for CubeToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CubeTool>()
            .register_type::<SpawnedCube>()
            .add_plugins(InputManagerPlugin::<CubeToolAction>::default())
            .add_systems(
                Update,
                (spawn_tool_event_responder_update_system, handle_input),
            );
    }
}

#[derive(Component, Reflect)]
pub struct CubeTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CubeToolAction {
    SpawnCube,
    RemoveCube,
    AttractCube,
}

impl CubeToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::SpawnCube => GamepadButtonType::South.into(),
            Self::RemoveCube => GamepadButtonType::East.into(),
            Self::AttractCube => GamepadButtonType::LeftTrigger.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::SpawnCube => KeyCode::ControlLeft.into(),
            Self::RemoveCube => KeyCode::ControlRight.into(),
            Self::AttractCube => KeyCode::AltRight.into(),
        }
    }

    fn default_input_map() -> InputMap<CubeToolAction> {
        let mut input_map = InputMap::default();

        for variant in CubeToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

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
                            name: Name::new(format!("Cube Tool")),
                            sprite_bundle: SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(100.0, 100.0)),
                                    ..default()
                                },
                                texture: asset_server.load("textures/cube.png"),
                                ..default()
                            },
                            ..default()
                        },
                        InputManagerBundle::<CubeToolAction> {
                            input_map: CubeToolAction::default_input_map(),
                            ..default()
                        },
                        CubeTool,
                    ));
                });
                info!("Added tool to toolbelt {:?}", toolbelt_id);
            }
        }
    }
}

#[derive(Component, Reflect)]
pub struct SpawnedCube;

fn handle_input(
    mut commands: Commands,
    tools: Query<(
        &ActionState<CubeToolAction>,
        Option<&ToolActiveTag>,
        &Parent,
    )>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<&Children, With<Character>>,
    pointers: Query<&GlobalTransform, With<Pointer>>,
    mut cubes: Query<(Entity, &GlobalTransform, &mut LinearVelocity), With<SpawnedCube>>,
) {
    for (t_act, t_enabled, t_parent) in tools.iter() {
        if t_enabled.is_none() {
            continue;
        }
        let c_kids = characters
            .get(
                toolbelts
                    .get(t_parent.get())
                    .expect("Toolbelt should have a parent")
                    .get(),
            )
            .expect("Toolbelt should have a character");
        let pointer = c_kids
            .iter()
            .filter_map(|x| pointers.get(*x).ok())
            .next()
            .expect("Character should have a pointer");
        if t_act.just_pressed(CubeToolAction::SpawnCube) {
            info!("Spawn Cube");
            commands.spawn((
                SpawnedCube,
                MovementDamping { factor: 0.98 },
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(15.0, 15.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(pointer.translation()),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::cuboid(15.0, 15.0),
                Name::new("Cube"),
            ));
        }
        if t_act.just_pressed(CubeToolAction::RemoveCube) {
            info!("Remove Cube");
            // remove the cube closest to the pointer
            let mut closest_cube = None;
            let mut closest_dist = f32::MAX;
            for (c_e, c_t, _) in cubes.iter() {
                let dist = c_t.translation().distance(pointer.translation());
                if dist < closest_dist {
                    closest_cube = Some(c_e);
                    closest_dist = dist;
                }
            }
            if let Some(cube) = closest_cube {
                commands.entity(cube).despawn_recursive();
            }
        }
        if t_act.pressed(CubeToolAction::AttractCube) {
            if t_act.just_pressed(CubeToolAction::AttractCube) {
                info!("Attract Cube");
            }
            // add a force to all cubes towards the pointer
            for (_, c_t, mut c_v) in cubes.iter_mut() {
                let diff = pointer.translation() - c_t.translation();
                let force = diff.normalize() * 100.0;
                c_v.x += force.x;
                c_v.y += force.y;
            }
        }
    }
}
