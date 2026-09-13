use std::io;
use std::path::Path;
use std::process::Command;

use common::invalid_data;

use crate::post_editor_state::PostEditorState;
use crate::project_paths::studio_core_manifest_path;

pub(crate) fn load_post_editor(project_root: &Path, post_id: &str) -> io::Result<PostEditorState> {
    let output = Command::new("cargo")
        .args([
            "run",
            "--manifest-path",
            &studio_core_manifest_path().to_string_lossy(),
            "--",
            &project_root.to_string_lossy(),
            "content",
            "post",
            "get",
            post_id,
        ])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::other(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    serde_json::from_slice(&output.stdout).map_err(invalid_data)
}
