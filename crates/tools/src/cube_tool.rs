use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use cursor_hero_character_types::prelude::*;
use cursor_hero_physics::damping_plugin::MovementDamping;
use cursor_hero_pointer_types::prelude::*;

use cursor_hero_toolbelt_types::prelude::*;

use crate::prelude::*;

pub struct CubeToolPlugin;

impl Plugin for CubeToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CubeTool>()
            .register_type::<CubeToolInteractable>()
            .add_plugins(InputManagerPlugin::<CubeToolAction>::default())
            .add_systems(Update, (toolbelt_events, handle_input));
    }
}

#[derive(Component, Reflect, Default)]
struct CubeTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Inspector { toolbelt_id } = event {
            ToolSpawnConfig::<CubeTool, CubeToolAction>::new(CubeTool, *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server, "png")
                .with_description("Spawn and attract cubes")
                .spawn(&mut commands);
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum CubeToolAction {
    Spawn,
    Remove,
    Attract,
    KillAll,
}

impl CubeToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Spawn => GamepadButtonType::South.into(),
            Self::Remove => GamepadButtonType::East.into(),
            Self::Attract => GamepadButtonType::LeftTrigger.into(),
            Self::KillAll => GamepadButtonType::West.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Spawn => KeyCode::Q.into(),
            Self::Remove => KeyCode::R.into(),
            Self::Attract => KeyCode::F.into(),
            Self::KillAll => KeyCode::X.into(),
        }
    }
}
impl ToolAction for CubeToolAction {
    fn default_input_map(_event: &PopulateToolbeltEvent) -> Option<InputMap<CubeToolAction>> {
        let mut input_map = InputMap::default();

        for variant in CubeToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}

#[derive(Component, Reflect)]
pub struct CubeToolInteractable;

fn handle_input(
    mut commands: Commands,
    tools: Query<(&ActionState<CubeToolAction>, &Parent), With<ActiveTool>>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<&Children, With<Character>>,
    pointers: Query<&GlobalTransform, With<Pointer>>,
    mut cubes: Query<(Entity, &GlobalTransform, &mut LinearVelocity), With<CubeToolInteractable>>,
) {
    for tool in tools.iter() {
        let (tool_actions, tool_parent) = tool;

        let Ok(toolbelt) = toolbelts.get(tool_parent.get()) else {
            warn!("Tool not inside a toolbelt?");
            continue;
        };
        let toolbelt_parent = toolbelt;

        let Ok(character) = characters.get(toolbelt_parent.get()) else {
            warn!("Toolbelt parent not a character?");
            continue;
        };
        let character_children = character;

        let Some(pointer) = character_children
            .iter()
            .filter_map(|x| pointers.get(*x).ok())
            .next()
        else {
            //TODO: warn if more than one pointer found
            warn!("Character {:?} missing a pointer?", toolbelt_parent.get());
            debug!("Character children: {:?}", character_children);
            continue;
        };
        let pointer_transform = pointer;

        if tool_actions.just_pressed(CubeToolAction::Spawn) {
            info!("Spawn Cube");
            commands.spawn((
                CubeToolInteractable,
                MovementDamping { factor: 0.98 },
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(15.0, 15.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(pointer_transform.translation()),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::cuboid(15.0, 15.0),
                Name::new("Cube"),
            ));
        }
        if tool_actions.just_pressed(CubeToolAction::Remove) {
            info!("Remove Cube");
            // remove the cube closest to the pointer
            let mut closest_cube = None;
            let mut closest_dist = f32::MAX;
            for (c_e, c_t, _) in cubes.iter() {
                let dist = c_t.translation().distance(pointer_transform.translation());
                if dist < closest_dist {
                    closest_cube = Some(c_e);
                    closest_dist = dist;
                }
            }
            if let Some(cube) = closest_cube {
                commands.entity(cube).despawn_recursive();
            }
        }
        if tool_actions.just_pressed(CubeToolAction::KillAll) {
            info!("Kill All Cubes");
            // remove all cubes
            for (c_e, _, _) in cubes.iter() {
                commands.entity(c_e).despawn_recursive();
            }
        }
        if tool_actions.pressed(CubeToolAction::Attract) {
            if tool_actions.just_pressed(CubeToolAction::Attract) {
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
