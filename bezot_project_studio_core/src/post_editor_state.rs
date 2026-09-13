use serde::{Deserialize, Serialize};

use crate::post_locale_editor::PostLocaleEditor;
use common::{new_uuid, today_yyyy_mm_dd};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostEditorState {
    pub id: String,
    pub date: String,
    pub status: String,
    pub author: String,
    pub fr: PostLocaleEditor,
    pub en: PostLocaleEditor,
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
