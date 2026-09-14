use std::path::PathBuf;

use crate::ai_task::AiTask;
use crate::content_entry::ContentEntry;
use crate::content_kind_filter::ContentKindFilter;
use crate::editor_target::EditorTarget;
use crate::editorial_ai_client::{OllamaModel, PostReview};
use crate::media_client::MediaAsset;
use crate::media_task::MediaTask;
use crate::page::Page;
use common::{PageEditorState, PostEditorState};

const PAGE_SIZE: usize = 6;

#[derive(Debug, Clone)]
pub struct StudioState {
    pub project_root: PathBuf,
    pub entries: Vec<ContentEntry>,
    pub search_query: String,
    pub kind_filter: ContentKindFilter,
    pub page_index: usize,
    pub selected_entry_id: Option<String>,
    pub post_editor: PostEditorState,
    pub page_editor: PageEditorState,
    pub editor_target: EditorTarget,
    pub current_page: Page,
    pub confirm_delete: bool,
    pub ai_draft_model: String,
    pub ai_review_model: String,
    pub ai_vram_gb: String,
    pub ai_topic: String,
    pub ai_task: AiTask,
    pub ai_models: Vec<OllamaModel>,
    pub ai_reviews: Vec<PostReview>,
    pub media_task: MediaTask,
    pub media_assets: Vec<MediaAsset>,
    pub notice: Option<String>,
    pub error: Option<String>,
}

impl StudioState {
    pub(crate) fn filtered_entries(&self) -> Vec<&ContentEntry> {
        let query = self.search_query.trim().to_lowercase();

        self.entries
            .iter()
            .filter(|entry| self.kind_filter.matches(&entry.kind))
            .filter(|entry| query.is_empty() || entry_matches_query(entry, &query))
            .collect()
    }

    pub(crate) fn visible_entries(&self) -> Vec<&ContentEntry> {
        let start = self.page_index.saturating_mul(PAGE_SIZE);

        self.filtered_entries()
            .into_iter()
            .skip(start)
            .take(PAGE_SIZE)
            .collect()
    }

    pub(crate) fn page_count(&self) -> usize {
        let filtered_count = self.filtered_entries().len();
        filtered_count.div_ceil(PAGE_SIZE).max(1)
    }

    pub(crate) fn filtered_count(&self) -> usize {
        self.filtered_entries().len()
    }

    pub(crate) fn reset_page(&mut self) {
        self.page_index = 0;
    }

    pub(crate) fn previous_page(&mut self) {
        self.page_index = self.page_index.saturating_sub(1);
    }

    pub(crate) fn next_page(&mut self) {
        let last_page_index = self.page_count().saturating_sub(1);
        self.page_index = (self.page_index + 1).min(last_page_index);
    }

    pub(crate) fn clamp_page_index(&mut self) {
        let last_page_index = self.page_count().saturating_sub(1);
        self.page_index = self.page_index.min(last_page_index);
    }

    pub(crate) fn edited_post_exists(&self) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.kind == "post" && entry.id == self.post_editor.id)
    }

    pub(crate) fn edited_page_exists(&self) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.kind == "page" && entry.id == self.page_editor.id)
    }

    pub(crate) fn ai_vram_gb(&self) -> Option<f64> {
        self.ai_vram_gb.trim().replace(',', ".").parse().ok()
    }
}

fn entry_matches_query(entry: &ContentEntry, query: &str) -> bool {
    entry.kind.to_lowercase().contains(query)
        || entry.id.to_lowercase().contains(query)
        || entry.status.to_lowercase().contains(query)
        || entry.locales.iter().any(|locale| {
            locale.locale.to_lowercase().contains(query)
                || locale.status.to_lowercase().contains(query)
                || locale.title.to_lowercase().contains(query)
                || locale.slug.to_lowercase().contains(query)
        })
}
