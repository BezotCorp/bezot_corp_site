use serde::Deserialize;

use crate::localized_content_summary::LocalizedContentSummary;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ContentEntry {
    pub(crate) kind: String,
    pub(crate) id: String,
    pub(crate) status: String,
    pub(crate) locales: Vec<LocalizedContentSummary>,
}
