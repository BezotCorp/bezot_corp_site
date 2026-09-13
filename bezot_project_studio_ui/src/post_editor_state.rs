use crate::post_locale_editor::PostLocaleEditor;

#[derive(Debug, Clone)]
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
            id: "new-blog-post".to_string(),
            date: "2026-09-14".to_string(),
            status: "draft".to_string(),
            author: "Bezot Corp".to_string(),
            fr: PostLocaleEditor::french_default(),
            en: PostLocaleEditor::english_default(),
        }
    }
}
