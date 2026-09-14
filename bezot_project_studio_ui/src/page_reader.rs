use std::io;
use std::path::Path;

use common::PageEditorState;
use common::invalid_data;

use crate::studio_core_runner::{run_studio_core, studio_core_failure};

pub(crate) fn load_page_editor(project_root: &Path, page_id: &str) -> io::Result<PageEditorState> {
    let output = run_studio_core(&[
        project_root.as_os_str().to_owned(),
        "content".into(),
        "page".into(),
        "get".into(),
        page_id.into(),
    ])?;

    if !output.status.success() {
        return Err(studio_core_failure(&output));
    }

    serde_json::from_slice(&output.stdout).map_err(invalid_data)
}
