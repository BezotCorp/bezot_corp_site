use std::io;
use std::path::Path;

use common::invalid_data;

use crate::studio_core_runner::{run_studio_core, studio_core_failure};
use common::PostEditorState;

pub(crate) fn load_post_editor(project_root: &Path, post_id: &str) -> io::Result<PostEditorState> {
    let output = run_studio_core(&[
        project_root.as_os_str().to_owned(),
        "content".into(),
        "post".into(),
        "get".into(),
        post_id.into(),
    ])?;

    if !output.status.success() {
        return Err(studio_core_failure(&output));
    }

    serde_json::from_slice(&output.stdout).map_err(invalid_data)
}
