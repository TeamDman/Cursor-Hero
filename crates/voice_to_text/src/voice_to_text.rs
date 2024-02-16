use bevy::log::debug;
use bevy::log::error;
use bevy::log::info;
use crossbeam_channel::Sender;
use cursor_hero_secret_types::secrets_types::SecretString;
use cursor_hero_voice_to_text_types::prelude::*;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use rand::distributions::Alphanumeric;
use rand::Rng;
use reqwest::Client;
use std::error::Error;
use std::process::Command;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::header::AUTHORIZATION;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::voice_to_text_worker_plugin::GameboundMessage;

pub(crate) const URL: &str = "https://localhost:9127";

pub(crate) async fn get_status() -> Result<VoiceToTextStatus, Box<dyn Error>> {
    let client = Client::new();
    match client.get(format!("{}/", URL)).send().await {
        Ok(res) => match res.status().is_success() {
            true => Ok(VoiceToTextStatus::AliveButWeDontKnowTheApiKey),
            false => Ok(VoiceToTextStatus::Dead),
        },
        Err(_) => Ok(VoiceToTextStatus::Dead),
    }
}

fn generate_api_key(len: usize) -> SecretString {
    let rng = rand::thread_rng();
    let inner = rng
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    SecretString::new(inner)
}

pub(crate) fn start() -> Result<SecretString, Box<dyn Error>> {
    let port = 9127;
    let api_key = generate_api_key(32);
    match std::process::Command::new("wt")
        .args([
            "--window",
            "0",
            "--profile",
            "PowerShell",
            "--title",
            "Voice2Text",
            "--",
            "pwsh",
            "-Command",
            r"cd D:\Repos\ml\voice2text && conda activate whisperx && python .\transcribe_hotkey_typer.py $env:port $env:api_key",
        ])
        .env("port", port.to_string())
        .env("api_key", api_key.expose_secret())
        .spawn()
    {
        Ok(_) => Ok(api_key),
        Err(e) => Err(Box::new(e)),
    }
}

pub(crate) fn start_vscode() -> Result<(), Box<dyn Error>> {
    match Command::new(r"C:\Program Files\Microsoft VS Code\Code.exe")
        .args([r"D:\Repos\ml\voice2text"])
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

pub(crate) async fn set_listening(
    listening: bool,
    api_key: SecretString,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let endpoint = match listening {
        true => format!("{}/start_listening", URL),
        false => format!("{}/stop_listening", URL),
    };
    match client
        .post(endpoint)
        .header(reqwest::header::AUTHORIZATION, api_key.expose_secret())
        .send()
        .await
    {
        Ok(res) => match res.status().is_success() {
            true => Ok(()),
            false => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Failed to set listening: {:?}",
                    match res.text().await {
                        Ok(text) => text,
                        Err(e) =>
                            format!("Failed to get response text during failure handler: {}", e),
                    }
                ),
            )))?,
        },
        Err(e) => Err(Box::new(e)),
    }
}

use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct TranscriptionResponse {
    segments: Vec<Segment>,
    // language: String,
}

#[derive(Debug, Deserialize)]
struct Segment {
    text: String,
    // start: f64,
    // end: f64,
}

pub(crate) async fn connect_receiver(
    game_tx: Sender<GameboundMessage>,
    api_key: SecretString,
) -> Result<(), Box<dyn Error>> {
    // Assuming the WebSocket endpoint is similar to HTTP but with ws(s) protocol
    let url = format!("{URL}/results").replace("http", "ws");

    // Add our auth header
    let mut req = url.into_client_request()?;
    let val = HeaderValue::from_str(api_key.expose_secret().as_str())?;
    req.headers_mut().insert(AUTHORIZATION, val);

    // Start worker to listen to responses without blocking the main thread
    tokio::spawn(async move {
        let (ws_stream, _) = match connect_async(req).await {
            Ok(conn) => {
                info!("Connected to WebSocket");
                conn
            }
            Err(e) => {
                error!("Failed to connect to WebSocket: {:?}", e);
                return;
            }
        };

        let (mut write, mut read) = ws_stream.split();
        debug!("Starting keepalive thread");
        tokio::spawn(async move {
            debug!("Keepalive thread started, entering main loop");
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                if let Err(e) = write.send(Message::text("keepalive")).await {
                    error!("Failed to send keepalive: {:?}", e);
                    break;
                }
            }
        });

        // Listening for messages
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    if let Message::Text(text) = msg {
                        // Deserialize the JSON text into TranscriptionResponse
                        debug!("Received message: {}", text);
                        match serde_json::from_str::<TranscriptionResponse>(&text) {
                            Ok(transcription) => {
                                // Concatenate the text of all segments
                                let concatenated_text = transcription
                                    .segments
                                    .iter()
                                    .map(|s| s.text.as_str())
                                    .collect::<Vec<&str>>()
                                    .join(" ");
                                let msg = GameboundMessage::TranscriptionReceived {
                                    transcription: concatenated_text,
                                };
                                debug!("Sending transcription to game: {:?}", msg);
                                if let Err(e) = game_tx.send(msg) {
                                    error!("Failed to send transcription to game: {:?}", e);
                                }
                            }
                            Err(e) => error!("Failed to deserialize message: {:?}", e),
                        }
                    }
                }
                Err(e) => {
                    error!("WebSocket error: {:?}", e);
                    break;
                }
            }
        }
    });

    Ok(())
}
