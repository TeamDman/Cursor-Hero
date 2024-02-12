use cursor_hero_voice_to_text_types::prelude::*;
use reqwest::Client;
use std::error::Error;
use std::process::Command;

pub async fn transcribe(prompt: Vec<u8>) -> Result<String, Box<dyn Error>> {
    let url = "http://localhost:8756/todo";
    let client = Client::new();
    let response = client.post(url).body(prompt).send().await?;
    let bytes = response.bytes().await?;
    let text = String::from_utf8(bytes.to_vec())?;
    Ok(text)
}

pub async fn get_status() -> Result<VoiceToTextStatus, Box<dyn Error>> {
    let client = Client::new();
    match client.get("http://localhost:8756/").send().await {
        Ok(res) => match res.status().is_success() {
            true => Ok(VoiceToTextStatus::Alive),
            false => Ok(VoiceToTextStatus::Dead),
        },
        Err(_) => Ok(VoiceToTextStatus::Dead),
    }
}

pub fn start() -> Result<(), Box<dyn Error>> {
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
            "\"cd",
            r"D:\Repos\ml\voice2text",
            "&&",
            "pwsh",
            "start.ps1",
        ])
        .spawn()
    {
        Ok(_) => Ok(()),
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
