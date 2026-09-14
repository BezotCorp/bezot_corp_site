use std::{io, path::Path};

use common::{PageEditorState, invalid_data};

use crate::studio_core_runner::{run_studio_core, run_studio_core_with_stdin, studio_core_failure};

pub(crate) fn save_page_with_core(project_root: &Path, editor: &PageEditorState) -> io::Result<()> {
    let source = serde_json::to_vec(editor).map_err(invalid_data)?;
    let output = run_studio_core_with_stdin(
        &[
            project_root.as_os_str().to_owned(),
            "content".into(),
            "page".into(),
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

pub(crate) fn delete_page_with_core(project_root: &Path, page_id: &str) -> io::Result<()> {
    let output = run_studio_core(&[
        project_root.as_os_str().to_owned(),
        "content".into(),
        "page".into(),
        "delete".into(),
        page_id.into(),
    ])?;

    if output.status.success() {
        Ok(())
    } else {
        Err(studio_core_failure(&output))
    }
}
