use std::thread;

use bevy::prelude::*;
use cursor_hero_voice_to_text_types::voice_to_text_types::VoiceToTextCommandEvent;
use cursor_hero_voice_to_text_types::voice_to_text_types::VoiceToTextStatus;
use cursor_hero_voice_to_text_types::voice_to_text_types::VoiceToTextStatusEvent;
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
        app.register_type::<TalkTool>();
        app.add_plugins(InputManagerPlugin::<TalkToolAction>::default());
        app.add_systems(Startup, spawn_worker_thread);
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, handle_input);
    }
}

#[derive(Component, Reflect, Default)]
struct TalkTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for event in reader.read() {
        let (ToolbeltLoadout::Keyboard | ToolbeltLoadout::Default) = event.loadout else {
            continue;
        };
        {
            ToolSpawnConfig::<TalkTool, TalkToolAction>::new(TalkTool, event.id, event)
                .with_src_path(file!().into())
                .guess_name(file!())
                .guess_image(file!(), &asset_server, "png")
                .with_description("Presses F23")
                .spawn(&mut commands);
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum TalkToolAction {
    PushToTalk,
    ToggleAlwaysOn,
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
            Self::PushToTalk => GamepadButtonType::Select.into(),
            Self::ToggleAlwaysOn => GamepadButtonType::Start.into(),
        }
    }

    fn default_wheel_mkb_binding(&self) -> UserInput {
        match self {
            Self::PushToTalk => KeyCode::ShiftRight.into(),
            Self::ToggleAlwaysOn => KeyCode::Scroll.into(),
        }
    }
    fn talk_wheel_gamepad_binding(&self) -> UserInput {
        match self {
            Self::PushToTalk => GamepadButtonType::Select.into(),
            Self::ToggleAlwaysOn => GamepadButtonType::Start.into(),
        }
    }

    fn talk_wheel_mkb_binding(&self) -> UserInput {
        match self {
            Self::PushToTalk => KeyCode::ShiftRight.into(),
            Self::ToggleAlwaysOn => KeyCode::Scroll.into(),
        }
    }
}
impl ToolAction for TalkToolAction {
    fn default_input_map(event: &ToolbeltPopulateEvent) -> Option<InputMap<TalkToolAction>> {
        match event.loadout {
            ToolbeltLoadout::Default => Some(Self::with_defaults(
                Self::default_wheel_gamepad_binding,
                Self::default_wheel_mkb_binding,
            )),
            ToolbeltLoadout::Keyboard => Some(Self::with_defaults(
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
    tools: Query<&ActionState<TalkToolAction>, With<ActiveTool>>,
    bridge: ResMut<Bridge>,
    mut voice_command_events: EventWriter<VoiceToTextCommandEvent>,
    mut voice_status_events: EventWriter<VoiceToTextStatusEvent>,
    mut voice_status: ResMut<VoiceToTextStatus>,
) {
    for t_act in tools.iter() {
        if t_act.just_pressed(TalkToolAction::PushToTalk) {
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
        if t_act.just_released(TalkToolAction::PushToTalk) {
            info!("Listen button released");
            match bridge.sender.send(ThreadMessage::ListenButton(Motion::Up)) {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to send thread message: {:?}", e);
                }
            }
        }
        if t_act.just_pressed(TalkToolAction::ToggleAlwaysOn) {
            let VoiceToTextStatus::Alive { api_key, listening } = voice_status.clone() else {
                warn!("VoiceToTextStatus not Alive, ignoring event");
                continue;
            };
            let new_status = VoiceToTextStatus::Alive {
                api_key: api_key.clone(),
                listening: !listening,
            };

            let event = VoiceToTextCommandEvent::SetListening {
                listening: !listening,
                api_key,
            };
            info!("Sending event: {:?}", event);
            voice_command_events.send(event);

            let event = VoiceToTextStatusEvent::Changed {
                old_status: voice_status.clone(),
                new_status: new_status.clone(),
            };
            info!("Sending event: {:?}", event);
            voice_status_events.send(event);

            *voice_status = new_status;
        }
    }
}
