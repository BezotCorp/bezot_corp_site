use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LocalizedContentSummary {
    pub locale: String,
    pub status: String,
    pub title: String,
    pub slug: String,
    pub updated_at: Option<String>,
    pub block_count: usize,
}
