use std::path::PathBuf;

use crate::content_entry::ContentEntry;

#[derive(Debug, Clone)]
pub struct StudioState {
    pub project_root: PathBuf,
    pub entries: Vec<ContentEntry>,
    pub error: Option<String>,
}
