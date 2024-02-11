use cursor_hero_glados_tts_types::glados_tts_types::GladosTtsStatus;
use reqwest::Client;
use std::error::Error;

pub async fn generate(prompt: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    // Construct the URL for the TTS endpoint
    let url = "http://localhost:8124/synthesize";

    // Create an instance of the reqwest client
    let client = Client::new();

    // Send a GET request to the server
    let response = client.post(url).body(prompt.to_string()).send().await?;

    // Ensure the request was successful and extract the bytes
    let bytes = response.bytes().await?;

    let wav = bytes.to_vec();

    // Ensure the audio decoder won't crash the freaking game
    rodio::Decoder::new(std::io::Cursor::new(wav.clone()))?;

    Ok(wav)
}

pub async fn get_status() -> Result<GladosTtsStatus, Box<dyn Error>> {
    let client = Client::new();
    match client.get("http://localhost:8124/").send().await {
        Ok(res) => match res.status().is_success() {
            true => Ok(GladosTtsStatus::Alive),
            false => Ok(GladosTtsStatus::Dead),
        },
        Err(_) => Ok(GladosTtsStatus::Dead),
    }
}

pub async fn start() -> Result<(), Box<dyn Error>> {
    // wt --window 0 --profile PowerShell -- pwsh -Command "cd G:\ml\glados-tts-upstream && conda activate gladostts && python .\engine.py"
    match std::process::Command::new("wt")
        .args(&[
            "--window",
            "0",
            "--profile",
            "PowerShell",
            "--title",
            "GLaDOS TTS",
            "--",
            "pwsh",
            "-Command",
            "\"cd",
            "G:\\ml\\glados-tts-upstream",
            "&&",
            "conda",
            "activate",
            "gladostts",
            "&&",
            "python",
            ".\\engine.py\"",
        ])
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
