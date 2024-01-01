use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::plugins::{
    camera_plugin::FollowWithCamera,
    character_plugin::{Character, CharacterColor},
};

use super::super::toolbelt::types::*;

pub struct FollowToolPlugin;

impl Plugin for FollowToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FollowTool>()
            .add_plugins(InputManagerPlugin::<ToolAction>::default())
            .add_systems(
                Update,
                (spawn_tool_event_responder_update_system, handle_input),
            );
    }
}

#[derive(Component, Reflect)]
pub struct FollowTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ToolAction {
    ToggleFollowCharacter,
}

impl ToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::ToggleFollowCharacter => GamepadButtonType::North.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::ToggleFollowCharacter => KeyCode::Numpad1.into(),
        }
    }

    fn default_input_map() -> InputMap<ToolAction> {
        let mut input_map = InputMap::default();

        for variant in ToolAction::variants() {
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
                            name: Name::new(format!("Follow Tool")),
                            sprite_bundle: SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(100.0, 100.0)),
                                    ..default()
                                },
                                texture: asset_server.load("textures/target.png"),
                                ..default()
                            },
                            ..default()
                        },
                        InputManagerBundle::<ToolAction> {
                            input_map: ToolAction::default_input_map(),
                            ..default()
                        },
                        FollowTool,
                    ));
                });
                info!("Added tool to toolbelt {:?}", toolbelt_id);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn handle_input(
    tools: Query<(&ActionState<ToolAction>, Option<&ToolActiveTag>, &Parent)>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    mut characters: Query<
        (
            Entity,
            Option<&FollowWithCamera>,
            &mut Handle<ColorMaterial>,
        ),
        With<Character>,
    >,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (t_act, t_enabled, t_parent) in tools.iter() {
        if t_enabled.is_none() {
            continue;
        }
        if t_act.just_pressed(ToolAction::ToggleFollowCharacter) {
            info!("Toggle follow character");
            let toolbelt = toolbelts
                .get(t_parent.get())
                .expect("Toolbelt should have a parent");
            let character = characters
                .get_mut(toolbelt.get())
                .expect("Toolbelt should have a character");
            let (character_entity , character_is_followed, mut material) =
                character;

            if character_is_followed.is_none() {
                commands.entity(character_entity).insert(FollowWithCamera);
                *material = materials.add(CharacterColor::FollowingWithCamera.as_material());
                info!("now following");
            } else {
                commands.entity(character_entity).remove::<FollowWithCamera>();
                *material = materials.add(CharacterColor::Default.as_material());
                info!("no longer following");
            }
        }
    }
}
