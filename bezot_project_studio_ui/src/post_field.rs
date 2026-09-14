use common::PostLocaleEditor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PostField {
    Title,
    Subtitle,
    Slug,
    Description,
    OgImage,
    AffiliateTitle,
    AffiliateText,
    AffiliateUrl,
    AffiliateLabel,
    AffiliateDisclosure,
}

impl PostField {
    pub(crate) fn field_mut(editor: &mut PostLocaleEditor, field: PostField) -> &mut String {
        match field {
            Self::Title => &mut editor.title,
            Self::Subtitle => &mut editor.subtitle,
            Self::Slug => &mut editor.slug,
            Self::Description => &mut editor.description,
            Self::OgImage => &mut editor.og_image,
            Self::AffiliateTitle => &mut editor.affiliate_title,
            Self::AffiliateText => &mut editor.affiliate_text,
            Self::AffiliateUrl => &mut editor.affiliate_url,
            Self::AffiliateLabel => &mut editor.affiliate_label,
            Self::AffiliateDisclosure => &mut editor.affiliate_disclosure,
        }
    }
}
