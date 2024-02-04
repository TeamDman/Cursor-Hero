use reqwest::Client;
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
