use crate::card_item_field::CardItemField;
use crate::editor_target::EditorTarget;
use crate::editorial_ai_client::{OllamaModel, PostReview};
use crate::locale::Locale;
use crate::media_client::MediaAsset;
use crate::page::Page;
use crate::page_block_field::PageBlockField;
use crate::page_field::PageField;
use crate::post_field::PostField;
use common::{PageBlockKind, PostEditorState};

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
    AddParagraph(Locale),
    RemoveParagraph(Locale, usize),
    MoveParagraphUp(Locale, usize),
    MoveParagraphDown(Locale, usize),
    ParagraphChanged(Locale, usize, String),
    DeletePost,

    NewPage,
    SavePage,
    DeletePage,
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

    AiDraftModelChanged(String),
    AiReviewModelChanged(String),
    AiVramChanged(String),
    AiTopicChanged(String),
    LoadModels,
    ModelsLoaded(Result<Vec<OllamaModel>, String>),
    GenerateDraft,
    DraftGenerated(Box<Result<PostEditorState, String>>),
    RunReview,
    ReviewCompleted(Result<Vec<PostReview>, String>),

    LoadMedia,
    MediaLoaded(Result<Vec<MediaAsset>, String>),
    PickAndUploadMedia,
    MediaUploaded(Result<Option<MediaAsset>, String>),
    CopyMediaPath(String),
}
