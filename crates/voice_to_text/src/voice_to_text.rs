use cursor_hero_voice_to_text_types::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use reqwest::Client;
use std::error::Error;
use std::process::Command;

pub const URL: &str = "https://localhost:9127";

pub async fn transcribe(prompt: Vec<u8>) -> Result<String, Box<dyn Error>> {
    let url = format!("{}/transcribe", URL);
    let client = Client::new();
    let response = client.post(url).body(prompt).send().await?;
    let bytes = response.bytes().await?;
    let text = String::from_utf8(bytes.to_vec())?;
    Ok(text)
}

pub async fn get_status() -> Result<VoiceToTextStatus, Box<dyn Error>> {
    let client = Client::new();
    match client.get(format!("{}/", URL)).send().await {
        Ok(res) => match res.status().is_success() {
            true => Ok(VoiceToTextStatus::AliveButWeDontKnowTheApiKey),
            false => Ok(VoiceToTextStatus::Dead),
        },
        Err(_) => Ok(VoiceToTextStatus::Dead),
    }
}

fn generate_api_key(len: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn start() -> Result<String, Box<dyn Error>> {
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
            format!(r"cd D:\Repos\ml\voice2text && conda activate whisperx && python .\transcribe_hotkey_typer.py {} {}", port, api_key).as_str(),
        ])
        .spawn()
    {
        Ok(_) => Ok(api_key),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn start_vscode() -> Result<(), Box<dyn Error>> {
    match Command::new(r"C:\Program Files\Microsoft VS Code\Code.exe")
        .args([r"D:\Repos\ml\voice2text"])
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
