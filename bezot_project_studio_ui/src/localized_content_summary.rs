use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LocalizedContentSummary {
    pub locale: String,
    pub status: String,
    pub title: String,
    pub slug: String,
    pub block_count: usize,
}
