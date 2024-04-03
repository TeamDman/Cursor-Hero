use bevy::prelude::*;
use bevy::sprite::Anchor;
use cursor_hero_cursor_types::prelude::*;
use cursor_hero_winutils::win_mouse::get_cursor_position;
use cursor_hero_worker::prelude::anyhow::Result;
use cursor_hero_worker::prelude::WorkerMessage;
use cursor_hero_worker::prelude::Sender;
use cursor_hero_worker::prelude::WorkerConfig;
use cursor_hero_worker::prelude::WorkerPlugin;

pub struct CursorMirroringPlugin;
impl Plugin for CursorMirroringPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorkerPlugin {
            config: WorkerConfig::<ThreadboundCursorMessage, GameboundCursorMessage, ()> {
                name: "cursor_mirroring".to_string(),
                handle_threadbound_message: handle_threadbound_message,
                threadbound_message_receiver: |_thread_rx, _state| {
                    // Keep the thread working on the task without waiting for a message
                    Ok(ThreadboundCursorMessage::CaptureCursorPosition)
                },
                sleep_duration: std::time::Duration::from_nanos(500),
                ..default()
            },
        });
        app.add_systems(Startup, spawn_cursor);
        app.add_systems(Update, (handle_gamebound_messages, update_visuals).chain());
        app.insert_resource(HostCursorPosition::default());
    }
}

#[derive(Debug, Reflect, Clone, Event)]
enum ThreadboundCursorMessage {
    CaptureCursorPosition,
}
impl WorkerMessage for ThreadboundCursorMessage {}

// This can be made public in the types crate if the need arises
#[derive(Debug, Reflect, Clone, Event)]
enum GameboundCursorMessage {
    HostCursorPosition(IVec2),
}
impl WorkerMessage for GameboundCursorMessage {}

fn handle_threadbound_message(
    msg: &ThreadboundCursorMessage,
    reply_tx: &Sender<GameboundCursorMessage>,
    _state: &mut (),
) -> Result<()> {
    let ThreadboundCursorMessage::CaptureCursorPosition = msg;
    let pos = get_cursor_position()?;
    reply_tx.send(GameboundCursorMessage::HostCursorPosition(pos))?;
    Ok(())
}

fn handle_gamebound_messages(
    mut rx: EventReader<GameboundCursorMessage>,
    mut res: ResMut<HostCursorPosition>,
) {
    if rx.is_empty() {
        return;
    }
    let mut latest = None;
    for msg in rx.read() {
        latest = Some(msg);
    }
    if let Some(GameboundCursorMessage::HostCursorPosition(pos)) = latest {
        res.0 = pos.clone();
    }
}

fn spawn_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 20.0),
            sprite: Sprite {
                // custom_size: Some(Vec2::new(20.0, 20.0)),
                anchor: Anchor::TopLeft,
                ..default()
            },

            texture: asset_server.load("textures/cursor.png"),
            ..default()
        },
        CursorMirror,
        Name::new("Cursor Mirror"),
    ));
}

fn update_visuals(
    mut cursor_mirrors: Query<(&mut Transform, &CursorMirror)>,
    cursor_position: Res<HostCursorPosition>,
) {
    for (mut transform, _) in &mut cursor_mirrors.iter_mut() {
        transform.translation.x = cursor_position.x as f32;
        transform.translation.y = -cursor_position.y as f32;
    }
}
