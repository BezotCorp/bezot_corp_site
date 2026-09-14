use std::io;
use std::path::Path;

use common::invalid_data;
use serde::Deserialize;

use crate::studio_core_runner::{run_studio_core, studio_core_failure};

/// Mirrors bezot_project_studio_core's own `MediaAsset`. Not shared via
/// `common`: this is just the wire shape of the `content media` commands'
/// JSON output, the same way `ContentEntry` is duplicated per crate.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct MediaAsset {
    pub(crate) name: String,
    #[serde(rename = "publicPath")]
    pub(crate) public_path: String,
    #[serde(rename = "sizeBytes")]
    pub(crate) size_bytes: u64,
}

pub(crate) fn list_media(project_root: &Path) -> io::Result<Vec<MediaAsset>> {
    let output = run_studio_core(&[
        project_root.as_os_str().to_owned(),
        "content".into(),
        "media".into(),
        "list".into(),
        "--format".into(),
        "json".into(),
    ])?;

    if !output.status.success() {
        return Err(studio_core_failure(&output));
    }

    serde_json::from_slice(&output.stdout).map_err(invalid_data)
}

pub(crate) fn upload_media(project_root: &Path, source_path: &Path) -> io::Result<MediaAsset> {
    let output = run_studio_core(&[
        project_root.as_os_str().to_owned(),
        "content".into(),
        "media".into(),
        "upload".into(),
        source_path.as_os_str().to_owned(),
    ])?;

    if !output.status.success() {
        return Err(studio_core_failure(&output));
    }

    serde_json::from_slice(&output.stdout).map_err(invalid_data)
}
