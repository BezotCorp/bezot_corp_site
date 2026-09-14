mod content_entry;
mod content_kind_filter;
mod content_loader;
mod locale;
mod localized_content_summary;
mod message;
mod page;
mod post_field;
mod post_reader;
mod post_writer;
mod project_paths;
mod studio_core_runner;
mod studio_state;
mod studio_view;
mod styles;
mod widgets;

#[cfg(test)]
mod tests;

use std::{env, io, path::PathBuf};

use common::{PostEditorState, PostLocaleEditor};
use iced::{Result, Theme, application};

use content_kind_filter::ContentKindFilter;
use content_loader::load_content_entries;
use message::Message;
use page::Page;
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
            current_page: Page::Dashboard,
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

    Ok(StudioState {
        project_root,
        entries,
        search_query: String::new(),
        kind_filter: ContentKindFilter::All,
        page_index: 0,
        selected_entry_id: None,
        post_editor: Default::default(),
        current_page: Page::Dashboard,
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

fn update(state: &mut StudioState, message: Message) {
    match message {
        Message::Navigate(page) => state.current_page = page,
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

            if selected_kind == Some("post") {
                let selected_id = state.selected_entry_id.as_deref().unwrap_or_default();
                match load_post_editor(&state.project_root, selected_id) {
                    Ok(editor) => {
                        state.post_editor = editor;
                        state.notice = Some(format!("Article chargé : {selected_id}."));
                        state.current_page = Page::Editor;
                        state.error = None;
                    }
                    Err(error) => {
                        state.error = Some(error.to_string());
                    }
                }
            } else {
                state.notice =
                    Some("L’édition des pages n’est pas encore dans cette tranche.".to_string());
            }
        }
        Message::NewPost => {
            state.post_editor = Default::default();
            state.notice = Some("Nouveau brouillon d’article prêt.".to_string());
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
    editor.fr.paragraph =
        "Commence par une réponse directe au problème du lecteur, puis développe les critères de décision, les limites et les étapes concrètes à suivre. L’objectif est de publier un contenu utile, compréhensible et assez précis pour être référencé sur une requête longue traîne.".to_string();
    editor.en.paragraph =
        "Start with a direct answer to the reader's problem, then explain the decision criteria, the limits, and the concrete next steps. The goal is to publish useful, understandable, and precise content that can rank for a long-tail query.".to_string();
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
