use std::io;

use common::invalid_data;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

const OLLAMA_GENERATE_ENDPOINT: &str = "http://localhost:11434/api/generate";
const OLLAMA_TAGS_ENDPOINT: &str = "http://localhost:11434/api/tags";

/// A model already pulled locally (`ollama list`), as reported by Ollama's
/// `/api/tags` endpoint. `size_bytes` is the on-disk size of the quantized
/// weights, which is also the closest available proxy for the VRAM it needs
/// once loaded — good enough to warn a caller before they pick a model that
/// will not fit and swap. `capabilities` and `family` let a caller flag
/// models specialized for code (fill-in-the-middle "insert" capability, or
/// a "coder" name) as a weaker fit for prose generation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OllamaModel {
    pub name: String,
    #[serde(default)]
    pub parameter_size: String,
    #[serde(default)]
    pub family: String,
    #[serde(default)]
    pub size_bytes: u64,
    #[serde(default)]
    pub capabilities: Vec<String>,
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
                .as_ref()
                .map(|details| details.parameter_size.clone())
                .unwrap_or_default(),
            family: entry
                .details
                .map(|details| details.family)
                .unwrap_or_default(),
            size_bytes: entry.size,
            capabilities: entry.capabilities,
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
    size: u64,
    #[serde(default)]
    details: Option<OllamaTagsModelDetails>,
    #[serde(default)]
    capabilities: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct OllamaTagsModelDetails {
    #[serde(default)]
    parameter_size: String,
    #[serde(default)]
    family: String,
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
