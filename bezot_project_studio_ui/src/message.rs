use crate::card_item_field::CardItemField;
use crate::editor_target::EditorTarget;
use crate::locale::Locale;
use crate::page::Page;
use crate::page_block_field::PageBlockField;
use crate::page_field::PageField;
use crate::post_field::PostField;
use common::PageBlockKind;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Navigate(Page),
    ShowEditorTarget(EditorTarget),
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

    NewPage,
    SavePage,
    PageIdChanged(String),
    PageFieldChanged(Locale, PageField, String),
    MarkPageDraft(Locale),
    MarkPagePublished(Locale),
    AddPageBlock(Locale, PageBlockKind),
    RemovePageBlock(Locale, usize),
    MovePageBlockUp(Locale, usize),
    MovePageBlockDown(Locale, usize),
    PageBlockTextChanged(Locale, usize, PageBlockField, String),
    PageBlockNumberChanged(Locale, usize, PageBlockField, String),
    PageBlockFlagToggled(Locale, usize, PageBlockField),
    AddCardItem(Locale, usize),
    RemoveCardItem(Locale, usize, usize),
    CardItemFieldChanged(Locale, usize, usize, CardItemField, String),
}
