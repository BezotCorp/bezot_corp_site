use common::PageBlock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PageBlockField {
    Title,
    Subtitle,
    Text,
    Email,
    Label,
    Url,
    Disclosure,
    Limit,
    Page,
    ShowDescription,
    ShowAuthor,
    ShowDate,
}

pub(crate) fn text_field_mut(block: &mut PageBlock, field: PageBlockField) -> Option<&mut String> {
    match (block, field) {
        (PageBlock::Hero { title, .. }, PageBlockField::Title) => Some(title),
        (PageBlock::Hero { subtitle, .. }, PageBlockField::Subtitle) => Some(subtitle),
        (PageBlock::Paragraph { text }, PageBlockField::Text) => Some(text),
        (PageBlock::MailLink { email, .. }, PageBlockField::Email) => Some(email),
        (PageBlock::MailLink { label, .. }, PageBlockField::Label) => Some(label),
        (PageBlock::AffiliateCallout { title, .. }, PageBlockField::Title) => Some(title),
        (PageBlock::AffiliateCallout { text, .. }, PageBlockField::Text) => Some(text),
        (PageBlock::AffiliateCallout { url, .. }, PageBlockField::Url) => Some(url),
        (PageBlock::AffiliateCallout { label, .. }, PageBlockField::Label) => Some(label),
        (PageBlock::AffiliateCallout { disclosure, .. }, PageBlockField::Disclosure) => {
            Some(disclosure)
        }
        _ => None,
    }
}

pub(crate) fn number_field_mut(
    block: &mut PageBlock,
    field: PageBlockField,
) -> Option<&mut Option<u32>> {
    match (block, field) {
        (PageBlock::PostList { limit, .. }, PageBlockField::Limit) => Some(limit),
        (PageBlock::PostList { page, .. }, PageBlockField::Page) => Some(page),
        _ => None,
    }
}

pub(crate) fn flag_mut(block: &mut PageBlock, field: PageBlockField) -> Option<&mut bool> {
    match (block, field) {
        (
            PageBlock::PostList {
                show_description, ..
            },
            PageBlockField::ShowDescription,
        ) => Some(show_description),
        (PageBlock::PostList { show_author, .. }, PageBlockField::ShowAuthor) => Some(show_author),
        (PageBlock::PostList { show_date, .. }, PageBlockField::ShowDate) => Some(show_date),
        _ => None,
    }
}
