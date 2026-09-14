use common::{PageEditorState, PageLocaleEditor, PostEditorState, PostLocaleEditor};

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

pub(crate) fn page_locale_mut(
    editor: &mut PageEditorState,
    locale: Locale,
) -> &mut PageLocaleEditor {
    match locale {
        Locale::French => &mut editor.fr,
        Locale::English => &mut editor.en,
    }
}
