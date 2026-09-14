use common::{PostEditorState, PostLocaleEditor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Locale {
    French,
    English,
}

pub(crate) fn locale_mut(editor: &mut PostEditorState, locale: Locale) -> &mut PostLocaleEditor {
    match locale {
        Locale::French => &mut editor.fr,
        Locale::English => &mut editor.en,
    }
}
