use bevy::audio::Volume;
use bevy::audio::VolumeLevel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::EguiContext;
use cursor_hero_character_types::prelude::*;
use cursor_hero_click_tool_types::prelude::*;
use cursor_hero_cursor_types::prelude::*;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_winutils::win_mouse::left_mouse_down;
use cursor_hero_winutils::win_mouse::left_mouse_up;
use cursor_hero_winutils::win_mouse::right_mouse_down;
use cursor_hero_winutils::win_mouse::right_mouse_up;
use cursor_hero_worker::prelude::anyhow::Context;
use cursor_hero_worker::prelude::anyhow::Result;
use cursor_hero_worker::prelude::Sender;
use cursor_hero_worker::prelude::WorkerConfig;
use cursor_hero_worker::prelude::WorkerPlugin;
use leafwing_input_manager::prelude::*;
pub struct ClickToolTickPlugin;

impl Plugin for ClickToolTickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ClickToolAction>::default());
        app.add_plugins(WorkerPlugin {
            config: WorkerConfig::<ThreadboundClickMessage, GameboundClickMessage, ()> {
                name: "click".to_string(),
                handle_threadbound_message,
                ..default()
            },
        });
        app.add_systems(Update, handle_input);
    }
}

fn handle_threadbound_message(
    msg: &ThreadboundClickMessage,
    _reply_tx: &Sender<GameboundClickMessage>,
    _state: &mut (),
) -> Result<()> {
    // pos is ignored because the cursor is positioned by other logic already
    match msg {
        ThreadboundClickMessage::LeftMouse(Motion::Down, _pos) => {
            left_mouse_down().context("Failed to handle left mouse down")
        }
        ThreadboundClickMessage::LeftMouse(Motion::Up, _pos) => {
            left_mouse_up().context("Failed to handle left mouse up")
        }
        ThreadboundClickMessage::RightMouse(Motion::Down, _pos) => {
            right_mouse_down().context("Failed to handle right mouse down")
        }
        ThreadboundClickMessage::RightMouse(Motion::Up, _pos) => {
            right_mouse_up().context("Failed to handle right mouse up")
        }
    }?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
fn handle_input(
    mut commands: Commands,
    tools: Query<(&ActionState<ClickToolAction>, &Parent), (With<ActiveTool>, With<ClickTool>)>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    characters: Query<&Children, With<Character>>,
    cursors: Query<(Entity, &GlobalTransform), With<Cursor>>,
    asset_server: Res<AssetServer>,
    mut tool_click_event_writer: EventWriter<ToolClickEvent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    egui_context_query: Query<&EguiContext, With<PrimaryWindow>>,
    mut bridge: EventWriter<ThreadboundClickMessage>,
) {
    // Do nothing when clicking over egui
    let egui_wants_pointer = egui_context_query
        .get_single()
        .map(|ctx| ctx.clone().get_mut().wants_pointer_input())
        .unwrap_or(false);

    for tool in tools.iter() {
        let (tool_actions, tool_parent) = tool;
        if !ClickToolAction::variants()
            .any(|action| tool_actions.just_pressed(action) || tool_actions.just_released(action))
        {
            continue;
        }

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

        let Some(cursor) = character_children
            .iter()
            .filter_map(|x| cursors.get(*x).ok())
            .next()
        else {
            //TODO: warn if more than one cursor found
            warn!("Character {:?} missing a cursor?", toolbelt_parent.get());
            debug!("Character children: {:?}", character_children);
            continue;
        };
        let (cursor_id, cursor_transform) = cursor;
        let cursor_pos = cursor_transform.translation();

        let window = window_query.get_single().expect("Need a single window");
        if window.cursor_position().is_some() && !egui_wants_pointer {
            // The host cursor is over the window
            // Perform virtual click instead of sending a message to the worker thread
            // debug!("Performing virtual click");
            for action in ClickToolAction::variants() {
                if tool_actions.just_pressed(action) {
                    debug!("{:?} pressed", action);

                    commands.spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(cursor_pos),
                            ..default()
                        },
                        Name::new("Click sound"),
                        AudioBundle {
                            source: asset_server.load(action.get_audio_path(Motion::Down)),
                            settings: PlaybackSettings::DESPAWN
                                .with_spatial(true)
                                .with_volume(Volume::Relative(VolumeLevel::new(0.5))),
                        },
                    ));

                    tool_click_event_writer.send(ToolClickEvent::Pressed {
                        cursor_id,
                        way: action.into(),
                    });
                }
                if tool_actions.just_released(action) {
                    debug!("{:?} released", action);

                    commands.spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(cursor_pos),
                            ..default()
                        },
                        Name::new("Click sound"),
                        AudioBundle {
                            source: asset_server.load(action.get_audio_path(Motion::Up)),
                            settings: PlaybackSettings::DESPAWN
                                .with_spatial(true)
                                .with_volume(Volume::Relative(VolumeLevel::new(0.5))),
                        },
                    ));

                    tool_click_event_writer.send(ToolClickEvent::Released {
                        cursor_id,
                        way: action.into(),
                    });
                }
            }
        } else {
            // The host cursor is outside the window
            // Send a message to the worker thread
            // debug!("Performing host click");
            for action in ClickToolAction::variants() {
                if tool_actions.just_pressed(action) {
                    debug!("{:?} pressed", action);
                    bridge.send(action.get_thread_message(
                        Motion::Down,
                        IVec2::new(cursor_pos.x as i32, -cursor_pos.y as i32),
                    ));
                    commands.spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(cursor_pos),
                            ..default()
                        },
                        Name::new("Click sound"),
                        AudioBundle {
                            source: asset_server.load(action.get_audio_path(Motion::Down)),
                            settings: PlaybackSettings::DESPAWN
                                .with_spatial(true)
                                .with_volume(Volume::Relative(VolumeLevel::new(0.5))),
                        },
                    ));
                }
                if tool_actions.just_released(action) {
                    // TODO: figure out why this fails using gamepad on egui
                    debug!("{:?} released", action);
                    bridge.send(action.get_thread_message(
                        Motion::Up,
                        IVec2::new(cursor_pos.x as i32, -cursor_pos.y as i32),
                    ));
                    commands.spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(cursor_pos),
                            ..default()
                        },
                        Name::new("Click sound"),
                        AudioBundle {
                            source: asset_server.load(action.get_audio_path(Motion::Up)),
                            settings: PlaybackSettings::DESPAWN
                                .with_spatial(true)
                                .with_volume(Volume::Relative(VolumeLevel::new(0.5))),
                        },
                    ));
                }
            }
        }
    }
}
