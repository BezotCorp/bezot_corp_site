use common::invalid_data;
use std::io;
use std::path::Path;
use std::process::Command;

use crate::content_entry::ContentEntry;
use crate::project_paths::studio_core_manifest_path;

pub fn load_content_entries(project_root: &Path) -> io::Result<Vec<ContentEntry>> {
    let output = Command::new("cargo")
        .args([
            "run",
            "--manifest-path",
            &studio_core_manifest_path().to_string_lossy(),
            "--",
            &project_root.to_string_lossy(),
            "content",
            "list",
            "--format",
            "json",
        ])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::other(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    serde_json::from_slice(&output.stdout).map_err(invalid_data)
}
