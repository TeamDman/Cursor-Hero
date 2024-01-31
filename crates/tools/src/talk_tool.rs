use std::thread;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crossbeam_channel::bounded;
use crossbeam_channel::Sender;
use cursor_hero_winutils::win_mouse::press_f23_key;
use cursor_hero_winutils::win_mouse::release_f23_key;

use cursor_hero_toolbelt_types::prelude::*;

use crate::prelude::*;

pub struct TalkToolPlugin;

impl Plugin for TalkToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TalkTool>()
            .add_plugins(InputManagerPlugin::<TalkToolAction>::default())
            .add_systems(Startup, spawn_worker_thread)
            .add_systems(Update, (toolbelt_events, handle_input));
    }
}

#[derive(Component, Reflect, Default)]
struct TalkTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Keyboard { toolbelt_id } = event {
            ToolSpawnConfig::<TalkTool, TalkToolAction>::new(TalkTool, *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Presses F23")
                .spawn(&mut commands);
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
    fn default_wheel_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Listen => GamepadButtonType::Select.into(),
        }
    }

    fn default_wheel_mkb_binding(&self) -> UserInput {
        match self {
            Self::Listen => KeyCode::ShiftRight.into(),
        }
    }
    fn talk_wheel_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Listen => GamepadButtonType::LeftTrigger2.into(),
        }
    }

    fn talk_wheel_mkb_binding(&self) -> UserInput {
        match self {
            Self::Listen => KeyCode::ShiftRight.into(),
        }
    }
}
impl ToolAction for TalkToolAction {
    fn default_input_map(event: &PopulateToolbeltEvent) -> Option<InputMap<TalkToolAction>> {
        match event {
            PopulateToolbeltEvent::Default { .. } => Some(Self::with_defaults(
                Self::default_wheel_gamepad_binding,
                Self::default_wheel_mkb_binding,
            )),
            PopulateToolbeltEvent::Keyboard { .. } => Some(Self::with_defaults(
                Self::talk_wheel_gamepad_binding,
                Self::talk_wheel_mkb_binding,
            )),
            _ => None,
        }
    }
}

fn spawn_worker_thread(mut commands: Commands) {
    let (tx, rx) = bounded::<_>(10);
    commands.insert_resource(Bridge { sender: tx });
    thread::spawn(move || loop {
        let action = match rx.recv() {
            Ok(action) => action,
            Err(e) => {
                error!("Failed to receive thread message, exiting: {:?}", e);
                break;
            }
        };
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
    tools: Query<&ActionState<TalkToolAction>,With<ActiveTool>>,
    bridge: ResMut<Bridge>,
) {
    for t_act in tools.iter() {
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
