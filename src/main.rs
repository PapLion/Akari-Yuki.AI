use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    prompt: String,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Obtén la clave de la API de las variables de entorno
    let api_key = env::var("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").expect("OPENAI_API_KEY not set");

    // Crea un cliente HTTP
    let client = Client::new();

    // Configura la solicitud
    let request = OpenAIRequest {
        model: "text-davinci-003".to_string(),
        prompt: "¿Qué es la inteligencia artificial?".to_string(),
        max_tokens: 150,
    };

    // Haz la solicitud POST a la API de OpenAI
    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?;

    // Analiza la respuesta JSON
    let response_body: OpenAIResponse = response.json().await?;

    // Imprime la respuesta
    if let Some(choice) = response_body.choices.first() {
        println!("Respuesta de OpenAI: {}", choice.text);
    }

    Ok(())
}
