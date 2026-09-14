use std::io;

use common::invalid_data;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

const OLLAMA_GENERATE_ENDPOINT: &str = "http://localhost:11434/api/generate";
const OLLAMA_TAGS_ENDPOINT: &str = "http://localhost:11434/api/tags";

/// A model already pulled locally (`ollama list`), as reported by Ollama's
/// `/api/tags` endpoint. `parameter_size` is surfaced so a caller can show
/// the operator roughly how much VRAM a model needs before they pick one.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OllamaModel {
    pub name: String,
    #[serde(default)]
    pub parameter_size: String,
}

/// Lists every model already pulled locally. Never contacts a hosted API.
pub fn list_models() -> io::Result<Vec<OllamaModel>> {
    let response = ureq::get(OLLAMA_TAGS_ENDPOINT).call().map_err(tags_error)?;
    let body: OllamaTagsResponse = response.into_json().map_err(invalid_data)?;

    Ok(body
        .models
        .into_iter()
        .map(|entry| OllamaModel {
            name: entry.name,
            parameter_size: entry
                .details
                .map(|details| details.parameter_size)
                .unwrap_or_default(),
        })
        .collect())
}

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Vec<OllamaTagsModel>,
}

#[derive(Debug, Deserialize)]
struct OllamaTagsModel {
    name: String,
    #[serde(default)]
    details: Option<OllamaTagsModelDetails>,
}

#[derive(Debug, Deserialize)]
struct OllamaTagsModelDetails {
    #[serde(default)]
    parameter_size: String,
}

fn tags_error(error: ureq::Error) -> io::Error {
    io::Error::other(format!(
        "could not reach Ollama at {OLLAMA_TAGS_ENDPOINT} (is `ollama serve` running?): {error}"
    ))
}

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
