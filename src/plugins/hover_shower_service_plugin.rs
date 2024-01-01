use bevy::prelude::*;
use crossbeam_channel::{bounded, Receiver};
use serde::Deserialize;
use std::thread;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::windows::named_pipe::ClientOptions;

pub struct HoverShowerServicePlugin;
impl Plugin for HoverShowerServicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_event::<StreamEvent>()
            .add_systems(Update, read_stream);
    }
}

#[derive(Resource, Deref)]
struct StreamReceiver(pub Receiver<ReceivedData>);

#[derive(Event)]
pub struct StreamEvent(pub ReceivedData);

#[derive(Debug, Deserialize, Reflect, Clone)]
pub struct ElementDetails {
    pub name: String,
    pub bounding_rect: Vec<i32>,
    pub control_type: String,
    pub class_name: String,
    pub automation_id: String,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Reflect, Clone)]
pub struct InterestingElement {
    pub details: ElementDetails,
    pub depth: i32,
    pub relationship: String,
}

#[derive(Debug, Deserialize, Reflect, Clone)]
pub struct ReceivedData {
    pub cursor_position: (i32, i32),
    pub element_details: ElementDetails,
    pub interesting_elements: Vec<InterestingElement>,
}

pub fn start_service_process() -> Result<(), std::io::Error> {
    // spawn the dotnet process
    let mut command = std::process::Command::new("wt");
    command.args(vec![
        "pwsh",
        "-NoProfile",
        "-c",
        r"D:\Repos\Games\Cursor-Hero\other\start-hovershower.ps1",
    ]);
    command.spawn()?;
    Ok(())
}

fn setup(mut commands: Commands) {
    // spawn the listener thread

    let (tx, rx) = bounded::<ReceivedData>(10);
    commands.insert_resource(StreamReceiver(rx));

    // let mut rng = StdRng::seed_from_u64(19878367467713);
    const ERROR_PIPE_BUSY: i32 = 231;
    const PIPE_NAME: &str = r"\\.\pipe\hovershower";

    // Tokio thread
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            'new_connection: loop {
                let client = loop {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    match ClientOptions::new().open(PIPE_NAME) {
                        Ok(client) => break client,
                        Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY) => (),
                        Err(e) => {
                            info!("Error opening client: {}", e);
                            continue;
                        }
                    }
                };

                let mut reader = BufReader::new(client); // Pass ownership
                let mut line = String::new();

                // Reading the incoming Fibonacci numbers
                loop {
                    reader
                        .read_line(&mut line)
                        .await
                        .expect("Couldn't read line");

                    let received_data: Result<ReceivedData, serde_json::Error> =
                        serde_json::from_str(&line);
                    match received_data {
                        Ok(recv) => {
                            debug!("Received data! Cursor position: {:?}", recv.cursor_position);
                            tx.send(recv).unwrap();
                        }
                        Err(e) => {
                            warn!("Couldn't deserialize data: {}", e);
                            if line.is_empty() {
                                println!("Pipe closed");
                                continue 'new_connection;
                            }
                        }
                    }

                    line.clear();
                }
            }
        });
    });
}

// This system reads from the receiver and sends events to Bevy
fn read_stream(receiver: Res<StreamReceiver>, mut events: EventWriter<StreamEvent>) {
    for from_stream in receiver.try_iter() {
        events.send(StreamEvent(from_stream));
    }
}
