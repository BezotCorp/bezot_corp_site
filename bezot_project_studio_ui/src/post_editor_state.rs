use serde::{Deserialize, Serialize};

use crate::post_locale_editor::PostLocaleEditor;
use common::{new_uuid, today_yyyy_mm_dd};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PostEditorState {
    pub(crate) id: String,
    pub(crate) date: String,
    pub(crate) status: String,
    pub(crate) author: String,
    pub(crate) fr: PostLocaleEditor,
    pub(crate) en: PostLocaleEditor,
}

impl Default for PostEditorState {
    fn default() -> Self {
        Self {
            id: new_uuid(),
            date: today_yyyy_mm_dd(),
            status: "draft".to_string(),
            author: "Bezot Corp".to_string(),
            fr: PostLocaleEditor::french_default(),
            en: PostLocaleEditor::english_default(),
        }
    }
}
