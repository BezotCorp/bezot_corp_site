use crate::ai_task::AiTask;
use crate::card_item_field::{self, CardItemField};
use crate::content_kind_filter::ContentKindFilter;
use crate::content_loader::load_content_entries;
use crate::editor_target::EditorTarget;
use crate::editorial_ai_client::{OllamaModel, PostReview};
use crate::locale::{self, Locale};
use crate::media_client::{self, MediaAsset};
use crate::media_task::MediaTask;
use crate::page::Page;
use crate::page_block_field::{self, PageBlockField};
use crate::page_field::{self, PageField};
use crate::page_reader::load_page_editor;
use crate::page_writer::{delete_page_with_core, save_page_with_core};
use crate::post_field::{self, PostField};
use crate::post_reader::load_post_editor;
use crate::post_writer::{delete_post_with_core, save_post_with_core};
use crate::preview_client::PreviewSession;
use crate::studio_state::StudioState;
use crate::{
    apply_monetized_template, apply_seo_template, clear_affiliate_fields, prepare_site,
    preview_client, save_notice, save_page_notice,
};
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

    StartPreview,
    PreviewReady(Result<PreviewSession, String>),
    StopPreview,
    OpenPreviewUrl(String),
}

impl Message {
    pub(crate) fn apply(state: &mut StudioState, message: Message) {
        match message {
            Self::GenerateDraft
            | Self::RunReview
            | Self::LoadModels
            | Self::LoadMedia
            | Self::PickAndUploadMedia
            | Self::CopyMediaPath(_)
            | Self::StartPreview => {
                unreachable!("intercepted in update() before reaching apply()")
            }
            Self::AiDraftModelChanged(value) => state.ai_draft_model = value,
            Self::AiReviewModelChanged(value) => state.ai_review_model = value,
            Self::AiVramChanged(value) => state.ai_vram_gb = value,
            Self::AiTopicChanged(value) => state.ai_topic = value,
            Self::ModelsLoaded(result) => {
                state.ai_task = AiTask::Idle;
                match result {
                    Ok(models) => {
                        state.notice = Some(format!("{} modèle(s) trouvé(s).", models.len()));
                        state.ai_models = models;
                    }
                    Err(error) => state.error = Some(error),
                }
            }
            Self::DraftGenerated(result) => {
                state.ai_task = AiTask::Idle;
                match *result {
                    Ok(editor) => {
                        state.notice = Some(format!("Brouillon IA créé : {}.", editor.id));
                        state.post_editor = editor;
                        state.editor_target = EditorTarget::Post;
                        state.selected_entry_id = None;
                        state.current_page = Page::Editor;
                        if let Ok(entries) = load_content_entries(&state.project_root) {
                            state.entries = entries;
                            state.clamp_page_index();
                        }
                    }
                    Err(error) => state.error = Some(error),
                }
            }
            Self::ReviewCompleted(result) => {
                state.ai_task = AiTask::Idle;
                match result {
                    Ok(reviews) => {
                        state.notice = Some(format!("{} analyse(s) reçue(s).", reviews.len()));
                        state.ai_reviews = reviews;
                    }
                    Err(error) => state.error = Some(error),
                }
            }
            Self::MediaLoaded(result) => {
                state.media_task = MediaTask::Idle;
                match result {
                    Ok(assets) => {
                        state.notice = Some(format!("{} fichier(s) trouvé(s).", assets.len()));
                        state.media_assets = assets;
                    }
                    Err(error) => state.error = Some(error),
                }
            }
            Self::MediaUploaded(result) => {
                state.media_task = MediaTask::Idle;
                match result {
                    Ok(Some(asset)) => {
                        state.notice = Some(format!("Fichier envoyé : {}.", asset.public_path));
                        if let Ok(assets) = media_client::list_media(&state.project_root) {
                            state.media_assets = assets;
                        } else {
                            state.media_assets.push(asset);
                        }
                    }
                    Ok(None) => {}
                    Err(error) => state.error = Some(error),
                }
            }
            Self::PreviewReady(result) => {
                state.preview_starting = false;
                match result {
                    Ok(session) => {
                        state.notice = Some("Aperçu réel prêt.".to_string());
                        state.preview = Some(session);
                    }
                    Err(error) => state.error = Some(error),
                }
            }
            Self::StopPreview => {
                if let Some(session) = state.preview.take() {
                    preview_client::stop_preview(session.pid);
                    state.notice = Some("Aperçu arrêté.".to_string());
                }
            }
            Self::OpenPreviewUrl(url) => {
                let _ = open::that(url);
            }
            Self::Navigate(page) => state.current_page = page,
            Self::ShowEditorTarget(target) => {
                state.editor_target = target;
                state.selected_entry_id = None;
                state.confirm_delete = false;
            }
            Self::ReloadContent => match load_content_entries(&state.project_root) {
                Ok(entries) => {
                    state.entries = entries;
                    state.clamp_page_index();
                    state.notice = Some("Contenu rechargé.".to_string());
                    state.error = None;
                }
                Err(error) => {
                    state.error = Some(error.to_string());
                }
            },
            Self::SelectEntry(id) => {
                let selected_kind = state
                    .entries
                    .iter()
                    .find(|entry| entry.id == id)
                    .map(|entry| entry.kind.as_str());

                state.selected_entry_id = Some(id);
                state.confirm_delete = false;
                let selected_id = state.selected_entry_id.clone().unwrap_or_default();

                match selected_kind {
                    Some("post") => match load_post_editor(&state.project_root, &selected_id) {
                        Ok(editor) => {
                            state.post_editor = editor;
                            state.editor_target = EditorTarget::Post;
                            state.notice = Some(format!("Article chargé : {selected_id}."));
                            state.current_page = Page::Editor;
                            state.error = None;
                        }
                        Err(error) => {
                            state.error = Some(error.to_string());
                        }
                    },
                    Some("page") => match load_page_editor(&state.project_root, &selected_id) {
                        Ok(editor) => {
                            state.page_editor = editor;
                            state.editor_target = EditorTarget::Page;
                            state.notice = Some(format!("Page chargée : {selected_id}."));
                            state.current_page = Page::Editor;
                            state.error = None;
                        }
                        Err(error) => {
                            state.error = Some(error.to_string());
                        }
                    },
                    _ => {
                        state.notice = Some("Contenu inconnu.".to_string());
                    }
                }
            }
            Self::NewPost => {
                state.post_editor = Default::default();
                state.editor_target = EditorTarget::Post;
                state.notice = Some("Nouveau brouillon d’article prêt.".to_string());
                state.selected_entry_id = None;
                state.current_page = Page::Editor;
                state.confirm_delete = false;
                state.error = None;
            }
            Self::NewPage => {
                state.page_editor = Default::default();
                state.editor_target = EditorTarget::Page;
                state.notice = Some("Nouvelle page prête.".to_string());
                state.selected_entry_id = None;
                state.current_page = Page::Editor;
                state.confirm_delete = false;
                state.error = None;
            }
            Self::PrepareSite => match prepare_site(&state.project_root) {
                Ok(()) => {
                    state.notice = Some("Site final préparé avec succès.".to_string());
                    state.error = None;
                }
                Err(error) => {
                    state.error = Some(error.to_string());
                }
            },
            Self::SearchQueryChanged(value) => {
                state.search_query = value;
                state.reset_page();
            }
            Self::ShowAllContent => {
                state.kind_filter = ContentKindFilter::All;
                state.reset_page();
            }
            Self::ShowPages => {
                state.kind_filter = ContentKindFilter::Pages;
                state.reset_page();
            }
            Self::ShowPosts => {
                state.kind_filter = ContentKindFilter::Posts;
                state.reset_page();
            }
            Self::PreviousPage => state.previous_page(),
            Self::NextPage => state.next_page(),
            Self::MarkDraft => state.post_editor.status = "draft".to_string(),
            Self::MarkPublished => state.post_editor.status = "published".to_string(),
            Self::MarkArchived => state.post_editor.status = "archived".to_string(),
            Self::ApplySeoTemplate => {
                apply_seo_template(&mut state.post_editor);
                state.notice = Some("Modèle SEO appliqué.".to_string());
                state.error = None;
            }
            Self::ApplyMonetizedTemplate => {
                apply_monetized_template(&mut state.post_editor);
                state.notice = Some("Modèle monétisé appliqué.".to_string());
                state.error = None;
            }
            Self::ClearAffiliateFields => {
                clear_affiliate_fields(&mut state.post_editor.fr);
                clear_affiliate_fields(&mut state.post_editor.en);
                state.notice = Some("Champs affiliation vidés.".to_string());
                state.error = None;
            }
            Self::PostDateChanged(value) => state.post_editor.date = value,
            Self::PostStatusChanged(value) => state.post_editor.status = value,
            Self::PostAuthorChanged(value) => state.post_editor.author = value,
            Self::PostFieldChanged(locale, field, value) => {
                *post_field::PostField::field_mut(
                    locale::locale_mut(&mut state.post_editor, locale),
                    field,
                ) = value;
            }
            Self::AddParagraph(locale) => {
                locale::locale_mut(&mut state.post_editor, locale)
                    .paragraphs
                    .push(String::new());
            }
            Self::RemoveParagraph(locale, index) => {
                let paragraphs = &mut locale::locale_mut(&mut state.post_editor, locale).paragraphs;
                if index < paragraphs.len() {
                    paragraphs.remove(index);
                }
            }
            Self::MoveParagraphUp(locale, index) => {
                let paragraphs = &mut locale::locale_mut(&mut state.post_editor, locale).paragraphs;
                if index > 0 && index < paragraphs.len() {
                    paragraphs.swap(index - 1, index);
                }
            }
            Self::MoveParagraphDown(locale, index) => {
                let paragraphs = &mut locale::locale_mut(&mut state.post_editor, locale).paragraphs;
                if index + 1 < paragraphs.len() {
                    paragraphs.swap(index, index + 1);
                }
            }
            Self::ParagraphChanged(locale, index, value) => {
                let paragraphs = &mut locale::locale_mut(&mut state.post_editor, locale).paragraphs;
                if let Some(paragraph) = paragraphs.get_mut(index) {
                    *paragraph = value;
                }
            }
            Self::SavePost => {
                let existed_before_save = state.edited_post_exists();
                match save_post_with_core(&state.project_root, &state.post_editor) {
                    Ok(()) => {
                        state.notice =
                            Some(save_notice(existed_before_save, &state.post_editor.id));
                        match load_content_entries(&state.project_root) {
                            Ok(entries) => {
                                state.entries = entries;
                                state.clamp_page_index();
                                state.error = None;
                            }
                            Err(error) => {
                                state.error =
                                    Some(format!("Impossible de recharger le contenu : {error}"));
                            }
                        }
                    }
                    Err(error) => {
                        state.error = Some(error.to_string());
                    }
                }
            }
            Self::DeletePost => {
                if !state.confirm_delete {
                    state.confirm_delete = true;
                    state.notice =
                        Some("Clique à nouveau pour confirmer la suppression.".to_string());
                    state.error = None;
                    return;
                }

                let post_id = state.post_editor.id.clone();
                state.confirm_delete = false;

                match delete_post_with_core(&state.project_root, &post_id) {
                    Ok(()) => {
                        state.notice = Some(format!("Article supprimé : {post_id}."));
                        state.post_editor = Default::default();
                        state.selected_entry_id = None;
                        state.current_page = Page::Library;
                        state.error = None;
                        if let Ok(entries) = load_content_entries(&state.project_root) {
                            state.entries = entries;
                            state.clamp_page_index();
                        }
                    }
                    Err(error) => {
                        state.error = Some(error.to_string());
                    }
                }
            }

            Self::PageIdChanged(value) => state.page_editor.id = value,
            Self::PageFieldChanged(locale, field, value) => {
                *page_field::field_mut(
                    locale::page_locale_mut(&mut state.page_editor, locale),
                    field,
                ) = value;
            }
            Self::MarkPageDraft(locale) => {
                locale::page_locale_mut(&mut state.page_editor, locale).status =
                    "draft".to_string();
            }
            Self::MarkPagePublished(locale) => {
                locale::page_locale_mut(&mut state.page_editor, locale).status =
                    "published".to_string();
            }
            Self::AddPageBlock(locale, kind) => {
                locale::page_locale_mut(&mut state.page_editor, locale)
                    .blocks
                    .push(kind.new_block());
            }
            Self::RemovePageBlock(locale, index) => {
                let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
                if index < blocks.len() {
                    blocks.remove(index);
                }
            }
            Self::MovePageBlockUp(locale, index) => {
                let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
                if index > 0 && index < blocks.len() {
                    blocks.swap(index - 1, index);
                }
            }
            Self::MovePageBlockDown(locale, index) => {
                let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
                if index + 1 < blocks.len() {
                    blocks.swap(index, index + 1);
                }
            }
            Self::PageBlockTextChanged(locale, index, field, value) => {
                let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
                if let Some(block) = blocks.get_mut(index)
                    && let Some(target) = page_block_field::text_field_mut(block, field)
                {
                    *target = value;
                }
            }
            Self::PageBlockNumberChanged(locale, index, field, value) => {
                let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
                if let Some(block) = blocks.get_mut(index)
                    && let Some(target) = page_block_field::number_field_mut(block, field)
                {
                    *target = value.trim().parse().ok();
                }
            }
            Self::PageBlockFlagToggled(locale, index, field) => {
                let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
                if let Some(block) = blocks.get_mut(index)
                    && let Some(target) = page_block_field::flag_mut(block, field)
                {
                    *target = !*target;
                }
            }
            Self::AddCardItem(locale, block_index) => {
                let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
                if let Some(common::PageBlock::CardGrid { items }) = blocks.get_mut(block_index) {
                    items.push(Default::default());
                }
            }
            Self::RemoveCardItem(locale, block_index, item_index) => {
                let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
                if let Some(common::PageBlock::CardGrid { items }) = blocks.get_mut(block_index)
                    && item_index < items.len()
                {
                    items.remove(item_index);
                }
            }
            Self::CardItemFieldChanged(locale, block_index, item_index, field, value) => {
                let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
                if let Some(common::PageBlock::CardGrid { items }) = blocks.get_mut(block_index)
                    && let Some(item) = items.get_mut(item_index)
                {
                    *card_item_field::CardItemField::field_mut(item, field) = value;
                }
            }
            Self::SavePage => {
                let existed_before_save = state.edited_page_exists();
                match save_page_with_core(&state.project_root, &state.page_editor) {
                    Ok(()) => {
                        state.notice =
                            Some(save_page_notice(existed_before_save, &state.page_editor.id));
                        match load_content_entries(&state.project_root) {
                            Ok(entries) => {
                                state.entries = entries;
                                state.clamp_page_index();
                                state.error = None;
                            }
                            Err(error) => {
                                state.error =
                                    Some(format!("Impossible de recharger le contenu : {error}"));
                            }
                        }
                    }
                    Err(error) => {
                        state.error = Some(error.to_string());
                    }
                }
            }
            Self::DeletePage => {
                if !state.confirm_delete {
                    state.confirm_delete = true;
                    state.notice =
                        Some("Clique à nouveau pour confirmer la suppression.".to_string());
                    state.error = None;
                    return;
                }

                let page_id = state.page_editor.id.clone();
                state.confirm_delete = false;

                match delete_page_with_core(&state.project_root, &page_id) {
                    Ok(()) => {
                        state.notice = Some(format!("Page supprimée : {page_id}."));
                        state.page_editor = Default::default();
                        state.selected_entry_id = None;
                        state.current_page = Page::Library;
                        state.error = None;
                        if let Ok(entries) = load_content_entries(&state.project_root) {
                            state.entries = entries;
                            state.clamp_page_index();
                        }
                    }
                    Err(error) => {
                        state.error = Some(error.to_string());
                    }
                }
            }
        }
    }
}
