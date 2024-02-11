use cursor_hero_ollama_types::ollama_types::OllamaStatus;
use reqwest::Client;
use reqwest::Request;
use std::error::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    response: String,
}

pub async fn generate(prompt: &str) -> Result<String, Box<dyn Error>> {
    let payload = serde_json::json!({
        "model": "whatevs",
        "prompt": prompt,
        "stream": false
    });

    let client = Client::new();

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&payload)
        .send()
        .await?;

    if res.status().is_success() {
        let response_body = res.json::<ApiResponse>().await?;
        Ok(response_body.response)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to call API. Status: {}", res.status()),
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
        Err(_) => {
            Ok(OllamaStatus::Dead)
        }
    }
}
