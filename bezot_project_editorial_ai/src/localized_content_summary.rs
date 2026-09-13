use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LocalizedContentSummary {
    pub locale: String,
    pub title: String,
}
