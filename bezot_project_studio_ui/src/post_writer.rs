use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use common::invalid_data;

use crate::post_editor_state::PostEditorState;
use crate::project_paths::studio_core_manifest_path;

pub(crate) fn save_post_with_core(project_root: &Path, editor: &PostEditorState) -> io::Result<()> {
    let mut child = Command::new("cargo")
        .args([
            "run",
            "--manifest-path",
            &studio_core_manifest_path().to_string_lossy(),
            "--",
            &project_root.to_string_lossy(),
            "content",
            "post",
            "save",
        ])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let source = serde_json::to_vec(editor).map_err(invalid_data)?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("could not open studio_core stdin"))?;
    stdin.write_all(&source)?;
    drop(stdin);

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::other(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}
