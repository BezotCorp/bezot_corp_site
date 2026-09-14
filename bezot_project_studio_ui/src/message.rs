use crate::locale::Locale;
use crate::page::Page;
use crate::post_field::PostField;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Navigate(Page),
    ReloadContent,
    SelectEntry(String),
    NewPost,
    SavePost,
    PrepareSite,
    SearchQueryChanged(String),
    ShowAllContent,
    ShowPages,
    ShowPosts,
    PreviousPage,
    NextPage,
    MarkDraft,
    MarkPublished,
    MarkArchived,
    ApplySeoTemplate,
    ApplyMonetizedTemplate,
    ClearAffiliateFields,
    PostDateChanged(String),
    PostStatusChanged(String),
    PostAuthorChanged(String),
    PostFieldChanged(Locale, PostField, String),
}
