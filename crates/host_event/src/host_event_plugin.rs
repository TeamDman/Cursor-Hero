use bevy::prelude::*;

use crossbeam_channel::Receiver;
use cursor_hero_host_event_types::prelude::HostEvent;
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

fn process_events(bridge: ResMut<EventBridge>, mut host_events: EventWriter<HostEvent>) {
    for event in bridge.receiver.try_iter() {
        if let ProcMessage::MouseMoved { .. } = event {
            host_events.send(HostEvent::MousePhysicallyMoved);
        }
    }
}
