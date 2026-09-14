use std::ffi::OsString;
use std::io;
use std::path::Path;

use common::PostEditorState;
use serde::Deserialize;

use crate::editorial_ai_runner::{editorial_ai_failure, run_editorial_ai};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct PostReview {
    pub(crate) id: String,
    pub(crate) locale: String,
    pub(crate) seo_score: u8,
    pub(crate) needs_update: bool,
    pub(crate) suggestions: Vec<String>,
}

/// Mirrors bezot_project_editorial_ai's own `OllamaModel`. Not shared via
/// `common`: this is just the wire shape of the `models` command's JSON
/// output, the same way `ContentEntry` is duplicated per crate rather than
/// shared for `content list`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub(crate) struct OllamaModel {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) parameter_size: String,
    #[serde(default)]
    pub(crate) family: String,
    #[serde(default)]
    pub(crate) size_bytes: u64,
    #[serde(default)]
    pub(crate) capabilities: Vec<String>,
}

impl OllamaModel {
    pub(crate) fn size_gb(&self) -> f64 {
        self.size_bytes as f64 / 1_000_000_000.0
    }

    /// A model whose name says "coder" or that advertises the
    /// fill-in-the-middle "insert" capability is tuned for source code, not
    /// prose — a weaker fit for drafting a blog article.
    pub(crate) fn is_code_specialized(&self) -> bool {
        self.name.to_lowercase().contains("coder")
            || self
                .capabilities
                .iter()
                .any(|capability| capability == "insert")
    }
}

pub(crate) fn generate_draft(
    project_root: &Path,
    model: &str,
    topic: &str,
) -> io::Result<PostEditorState> {
    let args: Vec<OsString> = vec![
        project_root.as_os_str().to_owned(),
        "draft".into(),
        "--model".into(),
        model.into(),
        "--topic".into(),
        topic.into(),
    ];

    let output = run_editorial_ai(&args)?;

    if !output.status.success() {
        return Err(editorial_ai_failure(&output));
    }

    serde_json::from_slice(&output.stdout).map_err(invalid_data)
}

pub(crate) fn run_review(project_root: &Path, model: &str) -> io::Result<Vec<PostReview>> {
    let args: Vec<OsString> = vec![
        project_root.as_os_str().to_owned(),
        "review".into(),
        "--model".into(),
        model.into(),
        "--format".into(),
        "json".into(),
    ];

    let output = run_editorial_ai(&args)?;

    if !output.status.success() {
        return Err(editorial_ai_failure(&output));
    }

    serde_json::from_slice(&output.stdout).map_err(invalid_data)
}

pub(crate) fn list_models(project_root: &Path) -> io::Result<Vec<OllamaModel>> {
    let args: Vec<OsString> = vec![project_root.as_os_str().to_owned(), "models".into()];

    let output = run_editorial_ai(&args)?;

    if !output.status.success() {
        return Err(editorial_ai_failure(&output));
    }

    serde_json::from_slice(&output.stdout).map_err(invalid_data)
}

fn invalid_data(error: impl std::error::Error) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
}
