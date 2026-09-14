use std::{
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

use common::{PostEditorState, invalid_data};

use crate::project_paths::studio_core_manifest_path;

/// Loads a post's full editable content by shelling out to
/// `bezot_project_studio_core`, the same path the studio UI uses. Editorial
/// automation must never read post JSON files directly.
pub fn load_post_via_core(project_root: &Path, post_id: &str) -> io::Result<PostEditorState> {
    let output = studio_core_command(project_root, ["content", "post", "get", post_id]).output()?;

    if !output.status.success() {
        return Err(core_failure(&output.stderr));
    }

    serde_json::from_slice(&output.stdout).map_err(invalid_data)
}

/// Persists a post by shelling out to `bezot_project_studio_core`, so a
/// generated draft goes through the same validation and slug-redirect
/// handling as one saved from the studio UI.
pub fn save_post_via_core(project_root: &Path, editor: &PostEditorState) -> io::Result<()> {
    let mut child = studio_core_command(project_root, ["content", "post", "save"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let payload = serde_json::to_vec(editor).map_err(invalid_data)?;
    child
        .stdin
        .take()
        .expect("stdin was requested as piped")
        .write_all(&payload)?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(core_failure(&output.stderr))
    }
}

fn studio_core_command<const N: usize>(project_root: &Path, args: [&str; N]) -> Command {
    let mut command = Command::new("cargo");
    command.args([
        "run",
        "--manifest-path",
        &studio_core_manifest_path().to_string_lossy(),
        "--",
        &project_root.to_string_lossy(),
    ]);
    command.args(args);
    command
}

fn core_failure(stderr: &[u8]) -> io::Error {
    io::Error::other(String::from_utf8_lossy(stderr).to_string())
}
