use cursor_hero_inference_types::inference_types::TextInferenceOptions;
use cursor_hero_ollama_types::ollama_types::OllamaStatus;
use reqwest::Client;
use std::error::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    response: String,
}

pub async fn generate(
    prompt: &str,
    options: Option<TextInferenceOptions>,
) -> Result<String, Box<dyn Error>> {
    let mut payload = serde_json::json!({
        "model": "whatevs",
        "prompt": prompt,
        "stream": false
    });
    if let Some(options) = options {
        // create empty object
        let mut options_json = serde_json::json!({});

        if let Some(num_predict) = options.num_predict {
            options_json["num_predict"] = serde_json::json!(num_predict);
        }

        if let Some(stop) = options.stop {
            options_json["stop"] = serde_json::json!(stop);
        }

        payload["options"] = options_json;
    }

    let client = Client::new();

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&payload)
        .send()
        .await?;

    if res.status().is_success() {
        let api_response = res.json::<ApiResponse>().await?;
        let mut text = api_response.response.as_str();
        text = text.trim_end_matches("<dummy32000>");
        text = text.trim();
        Ok(text.to_string())
    } else {
        let status = res.status();
        let body = res.text().await?;
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to call API. Status: {} Body: {}", status, body),
        )))
    }
}

pub async fn get_status() -> Result<OllamaStatus, Box<dyn Error>> {
    let client = Client::new();
    match client.get("http://localhost:11434/").send().await {
        Ok(res) => match res.status().is_success() {
            true => Ok(OllamaStatus::Alive),
            false => Ok(OllamaStatus::Dead),
        },
        Err(_) => Ok(OllamaStatus::Dead),
    }
}

pub fn start() -> Result<(), Box<dyn Error>> {
    // wt --window 0 --profile "Ubuntu-22.04" --colorScheme "Ubuntu-22.04-ColorScheme" --title "Ollama Serve" wsl -d Ubuntu-22.04 -- ollama serve
    match std::process::Command::new("wt")
        .args([
            "--window",
            "0",
            "--profile",
            "Ubuntu 22.04.3 LTS",
            "--colorScheme",
            "Ubuntu-22.04-ColorScheme",
            "--title",
            "Ollama Serve",
            "wsl",
            "-d",
            "Ubuntu-22.04",
            "--",
            "ollama",
            "serve",
        ])
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn start_terminal() -> Result<(), Box<dyn Error>> {
    match std::process::Command::new("wt")
        .args([
            "--window",
            "0",
            "--profile",
            "Ubuntu 22.04.3 LTS",
            "--colorScheme",
            "Ubuntu-22.04-ColorScheme",
            "--title",
            "Ollama",
            "wsl",
            "-d",
            "Ubuntu-22.04",
            "--",
            "bash",
            "-l",
            "-c",
            "cd ~ && exec pwsh",
        ])
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
