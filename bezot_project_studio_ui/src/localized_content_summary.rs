use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct LocalizedContentSummary {
    pub(crate) locale: String,
    pub(crate) status: String,
    pub(crate) title: String,
    pub(crate) slug: String,
    pub(crate) block_count: usize,
}
