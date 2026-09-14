use std::io;

use common::invalid_data;
use serde::Deserialize;
use serde_json::{Value, json};

const OLLAMA_GENERATE_ENDPOINT: &str = "http://localhost:11434/api/generate";

/// Sends a prompt to a local Ollama server and parses the model's reply as
/// JSON. Ollama's `format: "json"` option guarantees syntactically valid
/// JSON output but not any particular shape, so callers still need to parse
/// the returned `Value` into whatever structure they asked the model for in
/// the prompt.
pub fn generate_json(model: &str, prompt: &str) -> io::Result<Value> {
    let request_body = json!({
        "model": model,
        "prompt": prompt,
        "format": "json",
        "stream": false,
    });

    let response = ureq::post(OLLAMA_GENERATE_ENDPOINT)
        .send_json(request_body)
        .map_err(ollama_error)?;

    let body: OllamaGenerateResponse = response.into_json().map_err(invalid_data)?;

    serde_json::from_str(&body.response).map_err(|error| {
        invalid_data(format!(
            "model \"{model}\" did not return the expected JSON shape: {error} (raw reply: {})",
            body.response
        ))
    })
}

#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
}

fn ollama_error(error: ureq::Error) -> io::Error {
    io::Error::other(format!(
        "could not reach Ollama at {OLLAMA_GENERATE_ENDPOINT} (is `ollama serve` running and is the model pulled?): {error}"
    ))
}
