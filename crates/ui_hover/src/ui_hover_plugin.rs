use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use cursor_hero_bevy::prelude::NegativeYIVec2;
use cursor_hero_cursor_types::cursor_types::MainCursor;
use cursor_hero_ui_automation::prelude::find_element_at;
use cursor_hero_ui_automation::prelude::gather_single_element_info;
use cursor_hero_ui_hover_types::prelude::GameHoveredIndicatorTag;
use cursor_hero_ui_hover_types::prelude::GameboundHoverMessage;
use cursor_hero_ui_hover_types::prelude::HoverInfo;
use cursor_hero_ui_hover_types::prelude::HoveredElement;
use cursor_hero_ui_hover_types::prelude::ScreenHoveredIndicatorTag;
use cursor_hero_ui_hover_types::prelude::ThreadboundHoverMessage;
use cursor_hero_winutils::win_mouse::get_cursor_position;
use cursor_hero_worker::prelude::anyhow::Error;
use cursor_hero_worker::prelude::anyhow::Result;
use cursor_hero_worker::prelude::Sender;
use cursor_hero_worker::prelude::WorkerConfig;
use cursor_hero_worker::prelude::WorkerPlugin;

pub struct UiHoverPlugin;

impl Plugin for UiHoverPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HoverInfo::default());

        app.add_plugins(WorkerPlugin {
            config: WorkerConfig::<ThreadboundHoverMessage, GameboundHoverMessage, ()> {
                name: "hover".to_string(),
                handle_threadbound_message,
                handle_threadbound_message_error_handler,
                sleep_duration: std::time::Duration::from_millis(10),
                ..default()
            },
        });
        app.add_systems(Update, trigger_host_hover_info_update);
        app.add_systems(Update, trigger_game_hover_info_update);
        app.add_systems(Update, handle_gamebound_messages);
        app.add_systems(Update, update_visuals);
    }
}

fn handle_threadbound_message_error_handler(
    msg: &ThreadboundHoverMessage,
    reply_tx: &Sender<GameboundHoverMessage>,
    _state: &mut (),
    _error: &Error,
) -> Result<()> {
    match msg {
        ThreadboundHoverMessage::AtHostCursorPosition => {
            reply_tx.send(GameboundHoverMessage::ClearHostHoverInfo)?;
        }
        ThreadboundHoverMessage::AtPositionFromGame(_) => {
            reply_tx.send(GameboundHoverMessage::ClearGameHoverInfo)?;
        }
        _ => (),
    }
    Ok(())
}

fn handle_threadbound_message(
    msg: &ThreadboundHoverMessage,
    reply_tx: &Sender<GameboundHoverMessage>,
    _state: &mut (),
) -> Result<()> {
    let reply = match msg {
        ThreadboundHoverMessage::AtPositionFromGame(pos) => {
            let root = find_element_at(*pos)?;
            let info = gather_single_element_info(&root)?;
            GameboundHoverMessage::GameHoverInfo(info)
        }
        ThreadboundHoverMessage::AtHostCursorPosition => {
            let pos = get_cursor_position()?;
            let root = find_element_at(pos)?;
            let info = gather_single_element_info(&root)?;
            GameboundHoverMessage::HostHoverInfo(info)
        }
        ThreadboundHoverMessage::ClearHost => GameboundHoverMessage::ClearHostHoverInfo,
        ThreadboundHoverMessage::ClearGame => GameboundHoverMessage::ClearGameHoverInfo,
    };
    reply_tx.send(reply)?;
    Ok(())
}

fn trigger_host_hover_info_update(
    mut messages: EventWriter<ThreadboundHoverMessage>,
    mut cooldown: Local<Option<Timer>>,
    hover_info: Res<HoverInfo>,
    time: Res<Time>,
) {
    if !hover_info.enabled {
        return;
    }

    let Some(cooldown) = cooldown.as_mut() else {
        cooldown.replace(Timer::from_seconds(0.1, TimerMode::Repeating));
        return;
    };
    if !cooldown.tick(time.delta()).just_finished() {
        return;
    }

    let msg = ThreadboundHoverMessage::AtHostCursorPosition;
    messages.send(msg);
}

fn trigger_game_hover_info_update(
    cursor_query: Query<&GlobalTransform, With<MainCursor>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut hover_info: ResMut<HoverInfo>,
    mut messages: EventWriter<ThreadboundHoverMessage>,
    mut debounce: Local<Option<ThreadboundHoverMessage>>,
    mut cooldown: Local<Option<Timer>>,
    time: Res<Time>,
) {
    if !hover_info.enabled {
        return;
    }

    let window = match window_query.get_single() {
        Ok(window) => window,
        Err(e) => {
            warn!("Expected a single primary window, got error: {:?}", e);
            return;
        }
    };

    if window.cursor_position().is_none() {
        let msg = ThreadboundHoverMessage::ClearGame;
        let check = Some(msg.clone());
        if *debounce != check {
            *debounce = check;
            hover_info.game_element = None;
        }
        return;
    }

    let Some(cooldown) = cooldown.as_mut() else {
        cooldown.replace(Timer::from_seconds(0.1, TimerMode::Repeating));
        return;
    };
    if !cooldown.tick(time.delta()).just_finished() {
        return;
    }

    let cursor = match cursor_query.get_single() {
        Ok(cursor) => cursor,
        Err(e) => {
            warn!("Expected a single main cursor, got error: {:?}", e);
            return;
        }
    };

    let cursor_pos = cursor.translation().truncate().as_ivec2().neg_y();
    let msg = ThreadboundHoverMessage::AtPositionFromGame(cursor_pos);
    let check = Some(msg.clone());
    if *debounce != check {
        *debounce = check;
        messages.send(msg);
    }
}

fn handle_gamebound_messages(
    mut messages: EventReader<GameboundHoverMessage>,
    mut hover_info: ResMut<HoverInfo>,
) {
    for msg in messages.read() {
        match msg {
            GameboundHoverMessage::HostHoverInfo(info) => {
                hover_info.host_element = Some(info.clone());
            }
            GameboundHoverMessage::ClearHostHoverInfo => {
                hover_info.host_element = None;
            }
            GameboundHoverMessage::GameHoverInfo(info) => {
                hover_info.game_element = Some(info.clone());
            }
            GameboundHoverMessage::ClearGameHoverInfo => {
                hover_info.game_element = None;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_visuals(
    mut screen_indicator: Query<
        (Entity, &mut Sprite, &mut Transform, &mut HoveredElement),
        (
            With<ScreenHoveredIndicatorTag>,
            Without<GameHoveredIndicatorTag>,
        ),
    >,
    mut game_indicator: Query<
        (Entity, &mut Sprite, &mut Transform, &mut HoveredElement),
        (
            With<GameHoveredIndicatorTag>,
            Without<ScreenHoveredIndicatorTag>,
        ),
    >,
    hovered: Res<HoverInfo>,
    mut commands: Commands,
) {
    if let Ok((entity, mut sprite, mut transform, mut element)) = screen_indicator.get_single_mut()
    {
        if let Some(info) = &hovered.host_element {
            let bounds = info.bounding_rect.as_rect();
            sprite.custom_size = Some(Vec2::new(bounds.width(), bounds.height()));
            transform.translation = Vec3::new(
                bounds.min.x + bounds.width() / 2.,
                -bounds.min.y - bounds.height() / 2.,
                0.,
            );
            element.info = info.clone();
        } else {
            commands.entity(entity).despawn_recursive();
        }
    } else if let Some(info) = &hovered.host_element {
        let bounds = info.bounding_rect.as_rect();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    bounds.min.x + bounds.width() / 2.,
                    -bounds.min.y - bounds.height() / 2.,
                    0.,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(bounds.width(), bounds.height())),
                    color: Color::rgba(0.141, 0.675, 0.949, 0.05),
                    ..default()
                },
                ..default()
            },
            Name::new("Screen Hovered Indicator"),
            ScreenHoveredIndicatorTag,
            HoveredElement { info: info.clone() },
        ));
    }

    if let Ok((entity, mut sprite, mut transform, mut element)) = game_indicator.get_single_mut() {
        if let Some(info) = &hovered.game_element {
            let bounds = info.bounding_rect.as_rect();
            sprite.custom_size = Some(Vec2::new(bounds.width(), bounds.height()));
            transform.translation = Vec3::new(
                bounds.min.x + bounds.width() / 2.,
                -bounds.min.y - bounds.height() / 2.,
                0.,
            );
            element.info = info.clone();
        } else {
            commands.entity(entity).despawn_recursive();
        }
    } else if let Some(info) = &hovered.game_element {
        let bounds = info.bounding_rect.as_rect();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    bounds.min.x + bounds.width() / 2.,
                    -bounds.min.y - bounds.height() / 2.,
                    0.,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(bounds.width(), bounds.height())),
                    color: Color::rgba(0.641, 0.275, 0.649, 0.05),
                    ..default()
                },
                ..default()
            },
            Name::new("Game Hovered Indicator"),
            GameHoveredIndicatorTag,
            HoveredElement { info: info.clone() },
        ));
    }
}
