use serde::Deserialize;

use crate::localized_content_summary::LocalizedContentSummary;

#[derive(Debug, Clone, Deserialize)]
pub struct ContentEntry {
    pub kind: String,
    pub id: String,
    pub status: String,
    pub locales: Vec<LocalizedContentSummary>,
}
