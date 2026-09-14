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

fn invalid_data(error: impl std::error::Error) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
}
