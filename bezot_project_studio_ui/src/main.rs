mod ai_task;
mod ai_view;
mod card_item_field;
mod content_entry;
mod content_kind_filter;
mod content_loader;
mod editor_target;
mod editorial_ai_client;
mod editorial_ai_runner;
mod locale;
mod localized_content_summary;
mod message;
mod page;
mod page_block_field;
mod page_field;
mod page_reader;
mod page_workspace_view;
mod page_writer;
mod post_field;
mod post_reader;
mod post_writer;
mod project_paths;
mod studio_core_runner;
mod studio_state;
mod studio_view;
mod styles;
mod vram_fit;
mod widgets;

#[cfg(test)]
mod tests;

use std::{env, io, path::PathBuf};

use common::{PostEditorState, PostLocaleEditor};
use iced::{Result, Task, Theme, application};

use ai_task::AiTask;
use content_kind_filter::ContentKindFilter;
use content_loader::load_content_entries;
use editor_target::EditorTarget;
use editorial_ai_client::{OllamaModel, PostReview};
use message::Message;
use page::Page;
use page_reader::load_page_editor;
use page_writer::save_page_with_core;
use post_reader::load_post_editor;
use post_writer::save_post_with_core;
use project_paths::resolve_project_root;
use studio_core_runner::{run_studio_core, studio_core_failure};
use studio_state::StudioState;
use studio_view::view;

fn main() -> Result {
    let state = match boot_state() {
        Ok(state) => state,
        Err(error) => StudioState {
            project_root: PathBuf::from("."),
            entries: Vec::new(),
            search_query: String::new(),
            kind_filter: ContentKindFilter::All,
            page_index: 0,
            selected_entry_id: None,
            post_editor: Default::default(),
            page_editor: Default::default(),
            editor_target: EditorTarget::Post,
            current_page: Page::Dashboard,
            ai_draft_model: String::new(),
            ai_review_model: String::new(),
            ai_vram_gb: String::new(),
            ai_topic: String::new(),
            ai_task: AiTask::default(),
            ai_models: Vec::new(),
            ai_reviews: Vec::new(),
            notice: None,
            error: Some(error.to_string()),
        },
    };

    application(move || state.clone(), update, view)
        .title("Bezot Project Studio")
        .theme(theme)
        .run()
}

fn boot_state() -> io::Result<StudioState> {
    let args = env::args().collect::<Vec<_>>();
    let project_root_argument = project_root_argument(&args)?;
    let project_root = resolve_project_root(project_root_argument)?;
    let entries = load_content_entries(&project_root)?;
    // Best-effort: a model catalogue that fails to load (Ollama not running
    // yet, say) must not block the rest of the studio from starting. The
    // "Actualiser la liste" button in the IA tab covers the retry.
    let ai_models = editorial_ai_client::list_models(&project_root).unwrap_or_default();

    Ok(StudioState {
        project_root,
        entries,
        search_query: String::new(),
        kind_filter: ContentKindFilter::All,
        page_index: 0,
        selected_entry_id: None,
        post_editor: Default::default(),
        page_editor: Default::default(),
        editor_target: EditorTarget::Post,
        current_page: Page::Dashboard,
        ai_draft_model: String::new(),
        ai_review_model: String::new(),
        ai_vram_gb: String::new(),
        ai_topic: String::new(),
        ai_task: AiTask::default(),
        ai_models,
        ai_reviews: Vec::new(),
        notice: None,
        error: None,
    })
}

fn project_root_argument(args: &[String]) -> io::Result<&str> {
    match args {
        [_program] => Ok("site"),
        [_program, project_root] => Ok(project_root),
        _ => Err(invalid_input(
            "usage: bezot_project_studio_ui [project-root]",
        )),
    }
}

fn update(state: &mut StudioState, message: Message) -> Task<Message> {
    match message {
        Message::GenerateDraft => {
            state.ai_task = AiTask::GeneratingDraft;
            state.error = None;
            state.notice = None;
            let project_root = state.project_root.clone();
            let model = state.ai_draft_model.clone();
            let topic = state.ai_topic.clone();
            Task::perform(generate_draft_async(project_root, model, topic), |result| {
                Message::DraftGenerated(Box::new(result))
            })
        }
        Message::RunReview => {
            state.ai_task = AiTask::RunningReview;
            state.error = None;
            state.notice = None;
            let project_root = state.project_root.clone();
            let model = state.ai_review_model.clone();
            Task::perform(
                run_review_async(project_root, model),
                Message::ReviewCompleted,
            )
        }
        Message::LoadModels => {
            state.ai_task = AiTask::LoadingModels;
            state.error = None;
            state.notice = None;
            let project_root = state.project_root.clone();
            Task::perform(list_models_async(project_root), Message::ModelsLoaded)
        }
        other => {
            apply(state, other);
            Task::none()
        }
    }
}

async fn generate_draft_async(
    project_root: PathBuf,
    model: String,
    topic: String,
) -> std::result::Result<PostEditorState, String> {
    editorial_ai_client::generate_draft(&project_root, &model, &topic)
        .map_err(|error| error.to_string())
}

async fn run_review_async(
    project_root: PathBuf,
    model: String,
) -> std::result::Result<Vec<PostReview>, String> {
    editorial_ai_client::run_review(&project_root, &model).map_err(|error| error.to_string())
}

async fn list_models_async(project_root: PathBuf) -> std::result::Result<Vec<OllamaModel>, String> {
    editorial_ai_client::list_models(&project_root).map_err(|error| error.to_string())
}

fn apply(state: &mut StudioState, message: Message) {
    match message {
        Message::GenerateDraft | Message::RunReview | Message::LoadModels => {
            unreachable!("intercepted in update() before reaching apply()")
        }
        Message::AiDraftModelChanged(value) => state.ai_draft_model = value,
        Message::AiReviewModelChanged(value) => state.ai_review_model = value,
        Message::AiVramChanged(value) => state.ai_vram_gb = value,
        Message::AiTopicChanged(value) => state.ai_topic = value,
        Message::ModelsLoaded(result) => {
            state.ai_task = AiTask::Idle;
            match result {
                Ok(models) => {
                    state.notice = Some(format!("{} modèle(s) trouvé(s).", models.len()));
                    state.ai_models = models;
                }
                Err(error) => state.error = Some(error),
            }
        }
        Message::DraftGenerated(result) => {
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
        Message::ReviewCompleted(result) => {
            state.ai_task = AiTask::Idle;
            match result {
                Ok(reviews) => {
                    state.notice = Some(format!("{} analyse(s) reçue(s).", reviews.len()));
                    state.ai_reviews = reviews;
                }
                Err(error) => state.error = Some(error),
            }
        }
        Message::Navigate(page) => state.current_page = page,
        Message::ShowEditorTarget(target) => {
            state.editor_target = target;
            state.selected_entry_id = None;
        }
        Message::ReloadContent => match load_content_entries(&state.project_root) {
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
        Message::SelectEntry(id) => {
            let selected_kind = state
                .entries
                .iter()
                .find(|entry| entry.id == id)
                .map(|entry| entry.kind.as_str());

            state.selected_entry_id = Some(id);
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
        Message::NewPost => {
            state.post_editor = Default::default();
            state.editor_target = EditorTarget::Post;
            state.notice = Some("Nouveau brouillon d’article prêt.".to_string());
            state.selected_entry_id = None;
            state.current_page = Page::Editor;
            state.error = None;
        }
        Message::NewPage => {
            state.page_editor = Default::default();
            state.editor_target = EditorTarget::Page;
            state.notice = Some("Nouvelle page prête.".to_string());
            state.selected_entry_id = None;
            state.current_page = Page::Editor;
            state.error = None;
        }
        Message::PrepareSite => match prepare_site(&state.project_root) {
            Ok(()) => {
                state.notice = Some("Site final préparé avec succès.".to_string());
                state.error = None;
            }
            Err(error) => {
                state.error = Some(error.to_string());
            }
        },
        Message::SearchQueryChanged(value) => {
            state.search_query = value;
            state.reset_page();
        }
        Message::ShowAllContent => {
            state.kind_filter = ContentKindFilter::All;
            state.reset_page();
        }
        Message::ShowPages => {
            state.kind_filter = ContentKindFilter::Pages;
            state.reset_page();
        }
        Message::ShowPosts => {
            state.kind_filter = ContentKindFilter::Posts;
            state.reset_page();
        }
        Message::PreviousPage => state.previous_page(),
        Message::NextPage => state.next_page(),
        Message::MarkDraft => state.post_editor.status = "draft".to_string(),
        Message::MarkPublished => state.post_editor.status = "published".to_string(),
        Message::MarkArchived => state.post_editor.status = "archived".to_string(),
        Message::ApplySeoTemplate => {
            apply_seo_template(&mut state.post_editor);
            state.notice = Some("Modèle SEO appliqué.".to_string());
            state.error = None;
        }
        Message::ApplyMonetizedTemplate => {
            apply_monetized_template(&mut state.post_editor);
            state.notice = Some("Modèle monétisé appliqué.".to_string());
            state.error = None;
        }
        Message::ClearAffiliateFields => {
            clear_affiliate_fields(&mut state.post_editor.fr);
            clear_affiliate_fields(&mut state.post_editor.en);
            state.notice = Some("Champs affiliation vidés.".to_string());
            state.error = None;
        }
        Message::PostDateChanged(value) => state.post_editor.date = value,
        Message::PostStatusChanged(value) => state.post_editor.status = value,
        Message::PostAuthorChanged(value) => state.post_editor.author = value,
        Message::PostFieldChanged(locale, field, value) => {
            *post_field::field_mut(locale::locale_mut(&mut state.post_editor, locale), field) =
                value;
        }
        Message::AddParagraph(locale) => {
            locale::locale_mut(&mut state.post_editor, locale)
                .paragraphs
                .push(String::new());
        }
        Message::RemoveParagraph(locale, index) => {
            let paragraphs = &mut locale::locale_mut(&mut state.post_editor, locale).paragraphs;
            if index < paragraphs.len() {
                paragraphs.remove(index);
            }
        }
        Message::MoveParagraphUp(locale, index) => {
            let paragraphs = &mut locale::locale_mut(&mut state.post_editor, locale).paragraphs;
            if index > 0 && index < paragraphs.len() {
                paragraphs.swap(index - 1, index);
            }
        }
        Message::MoveParagraphDown(locale, index) => {
            let paragraphs = &mut locale::locale_mut(&mut state.post_editor, locale).paragraphs;
            if index + 1 < paragraphs.len() {
                paragraphs.swap(index, index + 1);
            }
        }
        Message::ParagraphChanged(locale, index, value) => {
            let paragraphs = &mut locale::locale_mut(&mut state.post_editor, locale).paragraphs;
            if let Some(paragraph) = paragraphs.get_mut(index) {
                *paragraph = value;
            }
        }
        Message::SavePost => {
            let existed_before_save = state.edited_post_exists();
            match save_post_with_core(&state.project_root, &state.post_editor) {
                Ok(()) => {
                    state.notice = Some(save_notice(existed_before_save, &state.post_editor.id));
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

        Message::PageIdChanged(value) => state.page_editor.id = value,
        Message::PageFieldChanged(locale, field, value) => {
            *page_field::field_mut(
                locale::page_locale_mut(&mut state.page_editor, locale),
                field,
            ) = value;
        }
        Message::MarkPageDraft(locale) => {
            locale::page_locale_mut(&mut state.page_editor, locale).status = "draft".to_string();
        }
        Message::MarkPagePublished(locale) => {
            locale::page_locale_mut(&mut state.page_editor, locale).status =
                "published".to_string();
        }
        Message::AddPageBlock(locale, kind) => {
            locale::page_locale_mut(&mut state.page_editor, locale)
                .blocks
                .push(kind.new_block());
        }
        Message::RemovePageBlock(locale, index) => {
            let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
            if index < blocks.len() {
                blocks.remove(index);
            }
        }
        Message::MovePageBlockUp(locale, index) => {
            let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
            if index > 0 && index < blocks.len() {
                blocks.swap(index - 1, index);
            }
        }
        Message::MovePageBlockDown(locale, index) => {
            let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
            if index + 1 < blocks.len() {
                blocks.swap(index, index + 1);
            }
        }
        Message::PageBlockTextChanged(locale, index, field, value) => {
            let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
            if let Some(block) = blocks.get_mut(index)
                && let Some(target) = page_block_field::text_field_mut(block, field)
            {
                *target = value;
            }
        }
        Message::PageBlockNumberChanged(locale, index, field, value) => {
            let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
            if let Some(block) = blocks.get_mut(index)
                && let Some(target) = page_block_field::number_field_mut(block, field)
            {
                *target = value.trim().parse().ok();
            }
        }
        Message::PageBlockFlagToggled(locale, index, field) => {
            let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
            if let Some(block) = blocks.get_mut(index)
                && let Some(target) = page_block_field::flag_mut(block, field)
            {
                *target = !*target;
            }
        }
        Message::AddCardItem(locale, block_index) => {
            let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
            if let Some(common::PageBlock::CardGrid { items }) = blocks.get_mut(block_index) {
                items.push(Default::default());
            }
        }
        Message::RemoveCardItem(locale, block_index, item_index) => {
            let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
            if let Some(common::PageBlock::CardGrid { items }) = blocks.get_mut(block_index)
                && item_index < items.len()
            {
                items.remove(item_index);
            }
        }
        Message::CardItemFieldChanged(locale, block_index, item_index, field, value) => {
            let blocks = &mut locale::page_locale_mut(&mut state.page_editor, locale).blocks;
            if let Some(common::PageBlock::CardGrid { items }) = blocks.get_mut(block_index)
                && let Some(item) = items.get_mut(item_index)
            {
                *card_item_field::field_mut(item, field) = value;
            }
        }
        Message::SavePage => {
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
    }
}

fn theme(_state: &StudioState) -> Theme {
    Theme::Dark
}

fn apply_seo_template(editor: &mut PostEditorState) {
    editor.status = "draft".to_string();
    editor.author = "Bezot Corp".to_string();
    editor.fr.description =
        "Un article Bezot Corp conçu pour répondre clairement à une question précise et améliorer la visibilité organique du site.".to_string();
    editor.en.description =
        "A Bezot Corp article designed to answer a focused question clearly and improve the site's organic visibility.".to_string();
    editor.fr.paragraphs = vec![
        "Commence par une réponse directe au problème du lecteur, puis développe les critères de décision, les limites et les étapes concrètes à suivre. L’objectif est de publier un contenu utile, compréhensible et assez précis pour être référencé sur une requête longue traîne.".to_string(),
    ];
    editor.en.paragraphs = vec![
        "Start with a direct answer to the reader's problem, then explain the decision criteria, the limits, and the concrete next steps. The goal is to publish useful, understandable, and precise content that can rank for a long-tail query.".to_string(),
    ];
}

fn apply_monetized_template(editor: &mut PostEditorState) {
    apply_seo_template(editor);
    editor.fr.affiliate_title = "Ressource recommandée".to_string();
    editor.fr.affiliate_text =
        "Un outil ou une ressource complémentaire pour passer plus vite de l’idée à l’action."
            .to_string();
    editor.fr.affiliate_label = "Voir l’offre".to_string();
    editor.fr.affiliate_disclosure = "Lien affilié ou sponsorisé.".to_string();
    editor.en.affiliate_title = "Recommended resource".to_string();
    editor.en.affiliate_text =
        "A complementary tool or resource to move faster from idea to action.".to_string();
    editor.en.affiliate_label = "View offer".to_string();
    editor.en.affiliate_disclosure = "Affiliate or sponsored link.".to_string();
}

fn clear_affiliate_fields(editor: &mut PostLocaleEditor) {
    editor.affiliate_title.clear();
    editor.affiliate_text.clear();
    editor.affiliate_url.clear();
    editor.affiliate_label.clear();
    editor.affiliate_disclosure.clear();
}

fn prepare_site(project_root: &std::path::Path) -> io::Result<()> {
    let output = run_studio_core(&[project_root.as_os_str().to_owned(), "production".into()])?;

    if output.status.success() {
        Ok(())
    } else {
        Err(studio_core_failure(&output))
    }
}

fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}

fn save_notice(existed_before_save: bool, post_id: &str) -> String {
    if existed_before_save {
        format!("Article mis à jour : {post_id}.")
    } else {
        format!("Article créé : {post_id}.")
    }
}

fn save_page_notice(existed_before_save: bool, page_id: &str) -> String {
    if existed_before_save {
        format!("Page mise à jour : {page_id}.")
    } else {
        format!("Page créée : {page_id}.")
    }
}
