use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use bevy_xpbd_2d::components::Sensor;
use cursor_hero_bevy::prelude::NegativeYIVec2;
use cursor_hero_cursor_types::cursor_click_types::ClickEvent;
use cursor_hero_cursor_types::cursor_click_types::Clickable;
use cursor_hero_cursor_types::cursor_click_types::Way;
use cursor_hero_cursor_types::cursor_types::MainCursor;
use cursor_hero_ui_automation::prelude::find_element_at;
use cursor_hero_ui_automation::prelude::gather_single_element_info;
use cursor_hero_ui_hover_types::prelude::GameHoverIndicator;
use cursor_hero_ui_hover_types::prelude::GameboundHoverMessage;
use cursor_hero_ui_hover_types::prelude::HostHoverIndicator;
use cursor_hero_ui_hover_types::prelude::HoverIndicator;
use cursor_hero_ui_hover_types::prelude::HoverInfo;
use cursor_hero_ui_hover_types::prelude::InspectorHoverIndicator;
use cursor_hero_ui_hover_types::prelude::ThreadboundHoverMessage;
use cursor_hero_ui_inspector_types::prelude::InspectorEvent;
use cursor_hero_ui_inspector_types::prelude::UIData;
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
        app.add_systems(Update, hovered_click_listener);
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
        ThreadboundHoverMessage::AtPositionFromGame(cursor_pos) => {
            let root = find_element_at(*cursor_pos)?;
            let info = gather_single_element_info(&root)?;
            GameboundHoverMessage::GameHoverInfo {
                info,
                cursor_pos: *cursor_pos,
            }
        }
        ThreadboundHoverMessage::AtHostCursorPosition => {
            let cursor_pos = get_cursor_position()?;
            let root = find_element_at(cursor_pos)?;
            let info = gather_single_element_info(&root)?;
            GameboundHoverMessage::HostHoverInfo { info, cursor_pos }
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

#[allow(clippy::too_many_arguments)]
fn trigger_game_hover_info_update(
    cursor_query: Query<&GlobalTransform, With<MainCursor>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut hover_info: ResMut<HoverInfo>,
    mut messages: EventWriter<ThreadboundHoverMessage>,
    mut debounce: Local<Option<ThreadboundHoverMessage>>,
    mut cooldown: Local<Option<Timer>>,
    time: Res<Time>,
    egui_context_query: Query<&EguiContext, With<PrimaryWindow>>,
) {
    // Do nothing when disabled
    if !hover_info.enabled {
        return;
    }

    // Do nothing when hovering over egui
    if egui_context_query
        .get_single()
        .map(|ctx| ctx.clone().get_mut().is_pointer_over_area())
        .unwrap_or(false)
    {
        hover_info.game_hover_indicator = None;
        return;
    }

    // Get window
    let window = match window_query.get_single() {
        Ok(window) => window,
        Err(e) => {
            warn!("Expected a single primary window, got error: {:?}", e);
            return;
        }
    };

    // Clear game hover indicator when cursor is outside of window
    if window.cursor_position().is_none() {
        hover_info.game_hover_indicator = None;
    }

    // Delay between updates
    let Some(cooldown) = cooldown.as_mut() else {
        cooldown.replace(Timer::from_seconds(0.1, TimerMode::Repeating));
        return;
    };
    if !cooldown.tick(time.delta()).just_finished() {
        return;
    }

    // Get cursor position
    let cursor = match cursor_query.get_single() {
        Ok(cursor) => cursor,
        Err(e) => {
            warn!("Expected a single main cursor, got error: {:?}", e);
            return;
        }
    };

    // Prepare message
    let cursor_pos = cursor.translation().truncate().as_ivec2().neg_y();
    let msg = ThreadboundHoverMessage::AtPositionFromGame(cursor_pos);

    // Debounce
    let check = Some(msg.clone());
    if *debounce == check {
        return;
    }

    // Send message
    messages.send(msg);
    *debounce = check;
}

fn handle_gamebound_messages(
    mut messages: EventReader<GameboundHoverMessage>,
    mut hover_info: ResMut<HoverInfo>,
) {
    for msg in messages.read() {
        match msg {
            GameboundHoverMessage::HostHoverInfo { info, cursor_pos } => {
                if info.name == "Program Manager" && info.class_name == "Progman" {
                    return;
                }
                hover_info.host_hover_indicator = Some(HostHoverIndicator {
                    info: info.clone(),
                    cursor_pos: *cursor_pos,
                });
            }
            GameboundHoverMessage::ClearHostHoverInfo => {
                hover_info.host_hover_indicator = None;
            }
            GameboundHoverMessage::GameHoverInfo { info, cursor_pos } => {
                if info.name == "Program Manager" && info.class_name == "Progman" {
                    return;
                }
                hover_info.game_hover_indicator = Some(GameHoverIndicator {
                    info: info.clone(),
                    cursor_pos: *cursor_pos,
                });
            }
            GameboundHoverMessage::ClearGameHoverInfo => {
                hover_info.game_hover_indicator = None;
            }
        }
    }
}

struct IndicatorParams {
    color: Color,
    name: &'static str,
}
#[allow(clippy::type_complexity)]
fn update_indicator<
    T: Component + Clone + HoverIndicator + PartialEq,
    A: Component,
    B: Component,
>(
    indicator_query: &mut Query<
        (Entity, &mut Sprite, &mut Transform, &mut Collider, &mut T),
        (Without<A>, Without<B>),
    >,
    hovered_indicator_option: &Option<T>,
    commands: &mut Commands,
    params: IndicatorParams,
) {
    // host indicator
    if let Ok(indicator) = indicator_query.get_single_mut() {
        // indicator exists
        let (entity, mut sprite, mut transform, mut collider, mut indicator) = indicator;
        // if let Some(existing) = hovered_indicator_option && existing.get_info() == indicator.get_info() {
        //     // no change
        //     // do nothing
        // } else {
        //     // despawn indicator
        //     commands.entity(entity).despawn_recursive();
        // }

        if let Some(existing) = hovered_indicator_option {
            // hovered exists
            // update indicator
            let bounds = existing.get_bounds();
            sprite.custom_size = Some(Vec2::new(bounds.width(), bounds.height()));
            transform.translation = Vec3::new(
                bounds.min.x + bounds.width() / 2.,
                -bounds.min.y - bounds.height() / 2.,
                0.,
            );
            *collider = Collider::cuboid(bounds.width(), bounds.height());
            *indicator = existing.clone();
        } else {
            // hovered does not exist
            // despawn indicator
            commands.entity(entity).despawn_recursive();
        }
    } else if let Some(existing) = hovered_indicator_option {
        // indicator does not exist
        // spawn indicator
        let bounds = existing.get_bounds();
        let indicator = existing.clone();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    bounds.min.x + bounds.width() / 2.,
                    -bounds.min.y - bounds.height() / 2.,
                    0.,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(bounds.width(), bounds.height())),
                    color: params.color,
                    ..default()
                },
                ..default()
            },
            Clickable,
            RigidBody::Kinematic,
            Sensor,
            Collider::cuboid(bounds.width(), bounds.height()),
            Name::new(params.name),
            indicator,
        ));
    }
}

#[allow(clippy::type_complexity)]
fn update_visuals(
    mut host_indicator: Query<
        (
            Entity,
            &mut Sprite,
            &mut Transform,
            &mut Collider,
            &mut HostHoverIndicator,
        ),
        (
            Without<GameHoverIndicator>,
            Without<InspectorHoverIndicator>,
        ),
    >,
    mut game_indicator: Query<
        (
            Entity,
            &mut Sprite,
            &mut Transform,
            &mut Collider,
            &mut GameHoverIndicator,
        ),
        (
            Without<HostHoverIndicator>,
            Without<InspectorHoverIndicator>,
        ),
    >,
    mut inspector_indicator: Query<
        (
            Entity,
            &mut Sprite,
            &mut Transform,
            &mut Collider,
            &mut InspectorHoverIndicator,
        ),
        (Without<HostHoverIndicator>, Without<GameHoverIndicator>),
    >,
    hovered: Res<HoverInfo>,
    mut commands: Commands,
) {
    // Define parameters for each indicator type
    let host_params = IndicatorParams {
        color: Color::rgba(0.141, 0.675, 0.949, 0.05),
        name: "Host Hovered Indicator",
    };
    let game_params = IndicatorParams {
        color: Color::rgba(0.641, 0.275, 0.649, 0.05),
        name: "Game Hovered Indicator",
    };
    let inspector_params = IndicatorParams {
        color: Color::rgba(1.0, 0.855, 0.431, 0.05),
        name: "Inspector Hovered Indicator",
    };

    // Call `update_indicator` for each type
    update_indicator(
        &mut host_indicator,
        &hovered.host_hover_indicator,
        &mut commands,
        host_params,
    );
    update_indicator(
        &mut game_indicator,
        &hovered.game_hover_indicator,
        &mut commands,
        game_params,
    );
    update_indicator(
        &mut inspector_indicator,
        &hovered.inspector_hover_indicator,
        &mut commands,
        inspector_params,
    );
}

fn hovered_click_listener(
    mut click_events: EventReader<ClickEvent>,
    game_hover_query: Query<&GameHoverIndicator>,
    host_hover_query: Query<&HostHoverIndicator>,
    mut ui_data: ResMut<UIData>,
    mut inspector_events: EventWriter<InspectorEvent>,
) {
    for event in click_events.read() {
        let ClickEvent::Clicked {
            target_id,
            cursor_id: _,
            way,
        } = event
        else {
            continue;
        };
        if way == &Way::Left {
            if game_hover_query.get(*target_id).is_ok() || host_hover_query.get(*target_id).is_ok()
            {
                ui_data.paused ^= true;
                info!("Hover indicator clicked, paused set to {}", ui_data.paused);
            }
        } else if way == &Way::Right {
            inspector_events.send(InspectorEvent::PushScratchPad);
        }
    }
}
