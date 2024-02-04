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
    Ok(wav)
}
