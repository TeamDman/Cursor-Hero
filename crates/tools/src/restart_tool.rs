use std::os::windows::process::CommandExt;
use std::path::PathBuf;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use windows::Win32::System::Threading::CREATE_NEW_PROCESS_GROUP;

use cursor_hero_data::paths::CURSOR_HERO_GIT_DIR;
use cursor_hero_toolbelt::types::*;
pub struct RestartToolPlugin;

impl Plugin for RestartToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RestartTool>()
            .add_plugins(InputManagerPlugin::<ToolAction>::default())
            .add_systems(
                Update,
                (spawn_tool_event_responder_update_system, handle_input),
            );
    }
}

#[derive(Component, Reflect)]
pub struct RestartTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ToolAction {
    CancelAndRunPreviousTerminalCommand,
}

impl ToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::CancelAndRunPreviousTerminalCommand => GamepadButtonType::Start.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::CancelAndRunPreviousTerminalCommand => KeyCode::Return.into(),
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
                            name: Name::new("Restart Tool"),
                            sprite_bundle: SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(100.0, 100.0)),
                                    ..default()
                                },
                                texture: asset_server.load("textures/restart_tool.png"),
                                ..default()
                            },
                            ..default()
                        },
                        InputManagerBundle::<ToolAction> {
                            input_map: ToolAction::default_input_map(),
                            ..default()
                        },
                        RestartTool,
                        ToolActiveTag,
                    ));
                });
                info!("Added tool to toolbelt {:?}", toolbelt_id);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn handle_input(tools: Query<(&ActionState<ToolAction>, Option<&ToolActiveTag>)>) {
    for (t_act, t_enabled) in tools.iter() {
        if t_enabled.is_none() {
            continue;
        }
        if t_act.just_pressed(ToolAction::CancelAndRunPreviousTerminalCommand) {
            // run target/release/uparrow-enter.exe

            let mut path = PathBuf::from(CURSOR_HERO_GIT_DIR);
            path.push("target/release/uparrow-enter.exe");

            // if it doesn't exist
            if !path.exists() {
                let mut other_project_path = PathBuf::from(CURSOR_HERO_GIT_DIR);
                other_project_path.push("other/uparrow-enter");
                // run cargo build --release
                match std::process::Command::new("cargo")
                    .arg("build")
                    .arg("--release")
                    .current_dir(other_project_path)
                    .spawn()
                {
                    Ok(_) => info!("Successfully ran cargo build --release"),
                    Err(e) => error!("Failed to run cargo build --release: {}", e),
                }
            }

            match std::process::Command::new(path)
                .creation_flags(CREATE_NEW_PROCESS_GROUP.0)
                .spawn()
            {
                Ok(_) => info!("Successfully ran uparrow-enter.exe"),
                Err(e) => error!("Failed to run uparrow-enter.exe: {}", e),
            }
        }
    }
}
