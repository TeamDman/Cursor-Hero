use bevy::prelude::*;

use crossbeam_channel::Receiver;
use cursor_hero_input::active_input_state_plugin::ActiveInput;
use cursor_hero_winutils::win_events::create_os_event_listener;
use cursor_hero_winutils::win_events::ProcMessage;

pub struct HostEventPlugin;

impl Plugin for HostEventPlugin {
    #![allow(unused_variables)]
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_worker);
        app.add_systems(Update, process_events);
    }
}

#[derive(Resource)]
struct EventBridge {
    receiver: Receiver<ProcMessage>,
}

fn start_worker(mut commands: Commands) {
    info!("Starting worker thread");
    let Ok(rx) = create_os_event_listener() else {
        error!("Failed to create OS event listener");
        return;
    };
    commands.insert_resource(EventBridge { receiver: rx });
}

fn process_events(
    bridge: ResMut<EventBridge>,
    mut next_state: ResMut<NextState<ActiveInput>>,
    mut active_input: ResMut<ActiveInput>,
) {
    for event in bridge.receiver.try_iter() {
        if *active_input != ActiveInput::MouseAndKeyboard
            && matches!(event, ProcMessage::MouseMoved { .. })
        {
            info!("Switching to mouse and keyboard input because of {:?}", event);
            next_state.set(ActiveInput::MouseAndKeyboard);
            *active_input = ActiveInput::MouseAndKeyboard;
        }
    }
}
