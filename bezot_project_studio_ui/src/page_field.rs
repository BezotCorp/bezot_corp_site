use common::PageLocaleEditor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PageField {
    UpdatedAt,
    Slug,
    Title,
    Description,
    Robots,
    OgTitle,
    OgDescription,
    OgImage,
}

pub(crate) fn field_mut(editor: &mut PageLocaleEditor, field: PageField) -> &mut String {
    match field {
        PageField::UpdatedAt => &mut editor.updated_at,
        PageField::Slug => &mut editor.slug,
        PageField::Title => &mut editor.title,
        PageField::Description => &mut editor.description,
        PageField::Robots => &mut editor.robots,
        PageField::OgTitle => &mut editor.og_title,
        PageField::OgDescription => &mut editor.og_description,
        PageField::OgImage => &mut editor.og_image,
    }
}
