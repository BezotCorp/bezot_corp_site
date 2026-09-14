use std::io;
use std::path::Path;

use common::invalid_data;

use crate::content_entry::ContentEntry;
use crate::studio_core_runner::{run_studio_core, studio_core_failure};

pub(crate) fn load_content_entries(project_root: &Path) -> io::Result<Vec<ContentEntry>> {
    let output = run_studio_core(&[
        project_root.as_os_str().to_owned(),
        "content".into(),
        "list".into(),
        "--format".into(),
        "json".into(),
    ])?;

    if !output.status.success() {
        return Err(studio_core_failure(&output));
    }

    serde_json::from_slice(&output.stdout).map_err(invalid_data)
}
