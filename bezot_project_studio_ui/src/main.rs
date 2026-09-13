mod content_entry;
mod content_kind_filter;
mod content_loader;
mod localized_content_summary;
mod message;
mod post_editor_state;
mod post_locale_editor;
mod post_reader;
mod post_writer;
mod project_paths;
mod studio_state;
mod studio_view;

#[cfg(test)]
mod tests;

use std::{env, io, path::PathBuf};

use iced::{Result, Theme, application};

use content_kind_filter::ContentKindFilter;
use content_loader::load_content_entries;
use message::Message;
use post_reader::load_post_editor;
use post_writer::save_post_with_core;
use project_paths::resolve_project_root;
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
            state.error = None;
        }
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
        Message::PostIdChanged(value) => state.post_editor.id = value,
        Message::PostDateChanged(value) => state.post_editor.date = value,
        Message::PostStatusChanged(value) => state.post_editor.status = value,
        Message::PostAuthorChanged(value) => state.post_editor.author = value,
        Message::PostFrenchTitleChanged(value) => state.post_editor.fr.title = value,
        Message::PostFrenchSlugChanged(value) => state.post_editor.fr.slug = value,
        Message::PostFrenchDescriptionChanged(value) => state.post_editor.fr.description = value,
        Message::PostFrenchParagraphChanged(value) => state.post_editor.fr.paragraph = value,
        Message::PostEnglishTitleChanged(value) => state.post_editor.en.title = value,
        Message::PostEnglishSlugChanged(value) => state.post_editor.en.slug = value,
        Message::PostEnglishDescriptionChanged(value) => state.post_editor.en.description = value,
        Message::PostEnglishParagraphChanged(value) => state.post_editor.en.paragraph = value,
        Message::SavePost => match save_post_with_core(&state.project_root, &state.post_editor) {
            Ok(()) => match load_content_entries(&state.project_root) {
                Ok(entries) => {
                    state.entries = entries;
                    state.clamp_page_index();
                    state.notice = Some(format!("Article enregistré : {}.", state.post_editor.id));
                    state.error = None;
                }
                Err(error) => {
                    state.notice = Some(format!("Article enregistré : {}.", state.post_editor.id));
                    state.error = Some(format!("Impossible de recharger le contenu : {error}"));
                }
            },
            Err(error) => {
                state.error = Some(error.to_string());
            }
        },
    }
}

fn theme(_state: &StudioState) -> Theme {
    Theme::Dark
}

fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}
