use std::thread;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crossbeam_channel::bounded;
use crossbeam_channel::Sender;
use cursor_hero_winutils::win_mouse::press_f23_key;
use cursor_hero_winutils::win_mouse::release_f23_key;

use cursor_hero_toolbelt::types::*;

use crate::spawn_action_tool;

pub struct TalkToolPlugin;

impl Plugin for TalkToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TalkTool>()
            .add_plugins(InputManagerPlugin::<TalkToolAction>::default())
            .add_systems(Startup, spawn_worker_thread)
            .add_systems(Update, (toolbelt_events, handle_input));
    }
}

#[derive(Component, Reflect)]
struct TalkTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        match e {
            ToolbeltEvent::PopulateDefaultToolbelt(toolbelt_id) => {
                spawn_action_tool!(
                    commands,
                    *toolbelt_id,
                    asset_server,
                    TalkTool,
                    TalkToolAction
                );
            }
            _ => {}
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum TalkToolAction {
    Listen,
}

#[derive(Debug)]
enum Motion {
    Up,
    Down,
}

#[derive(Debug)]
enum ThreadMessage {
    ListenButton(Motion),
}

#[derive(Resource)]
struct Bridge {
    pub sender: Sender<ThreadMessage>,
}

impl TalkToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Listen => GamepadButtonType::Select.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Listen => KeyCode::ShiftRight.into(),
        }
    }

    fn default_input_map() -> InputMap<TalkToolAction> {
        let mut input_map = InputMap::default();

        for variant in TalkToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

fn spawn_worker_thread(mut commands: Commands) {
    let (tx, rx) = bounded::<_>(10);
    commands.insert_resource(Bridge { sender: tx });
    thread::spawn(move || loop {
        let action = rx.recv().unwrap();
        debug!("Worker received thread message: {:?}", action);
        match match action {
            ThreadMessage::ListenButton(Motion::Down) => press_f23_key(),
            ThreadMessage::ListenButton(Motion::Up) => release_f23_key(),
        } {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to handle event {:?}: {:?}", action, e);
            }
        }
    });
}

fn handle_input(
    tools: Query<(&ActionState<TalkToolAction>, Option<&ToolActiveTag>)>,
    bridge: ResMut<Bridge>,
) {
    for (t_act, t_enabled) in tools.iter() {
        if t_enabled.is_none() {
            continue;
        }
        if t_act.just_pressed(TalkToolAction::Listen) {
            info!("Listen button pressed");
            match bridge
                .sender
                .send(ThreadMessage::ListenButton(Motion::Down))
            {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to send thread message: {:?}", e);
                }
            }
        }
        if t_act.just_released(TalkToolAction::Listen) {
            info!("Listen button released");
            match bridge.sender.send(ThreadMessage::ListenButton(Motion::Up)) {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to send thread message: {:?}", e);
                }
            }
        }
    }
}
