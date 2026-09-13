use std::io;
use std::path::Path;

use common::invalid_data;

use crate::post_editor_state::PostEditorState;
use crate::studio_core_runner::{run_studio_core_with_stdin, studio_core_failure};

pub(crate) fn save_post_with_core(project_root: &Path, editor: &PostEditorState) -> io::Result<()> {
    let source = serde_json::to_vec(editor).map_err(invalid_data)?;
    let output = run_studio_core_with_stdin(
        &[
            project_root.as_os_str().to_owned(),
            "content".into(),
            "post".into(),
            "save".into(),
        ],
        &source,
    )?;

    if output.status.success() {
        Ok(())
    } else {
        Err(studio_core_failure(&output))
    }
}
