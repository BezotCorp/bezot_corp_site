use serde::Serialize;

use crate::localized_content_summary::LocalizedContentSummary;

#[derive(Debug, Serialize)]
pub struct ContentEntry {
    pub kind: &'static str,
    pub id: String,
    pub status: String,
    pub locales: Vec<LocalizedContentSummary>,
}
