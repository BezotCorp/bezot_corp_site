use common::PostLocaleEditor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PostField {
    Title,
    Slug,
    Description,
    Paragraph,
    AffiliateTitle,
    AffiliateText,
    AffiliateUrl,
    AffiliateLabel,
    AffiliateDisclosure,
}

pub(crate) fn field_mut(editor: &mut PostLocaleEditor, field: PostField) -> &mut String {
    match field {
        PostField::Title => &mut editor.title,
        PostField::Slug => &mut editor.slug,
        PostField::Description => &mut editor.description,
        PostField::Paragraph => &mut editor.paragraph,
        PostField::AffiliateTitle => &mut editor.affiliate_title,
        PostField::AffiliateText => &mut editor.affiliate_text,
        PostField::AffiliateUrl => &mut editor.affiliate_url,
        PostField::AffiliateLabel => &mut editor.affiliate_label,
        PostField::AffiliateDisclosure => &mut editor.affiliate_disclosure,
    }
}
