use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Element, Fill, Length};

use crate::content_entry::ContentEntry;
use crate::content_kind_filter::ContentKindFilter;
use crate::message::Message;
use crate::post_editor_state::PostEditorState;
use crate::post_locale_editor::PostLocaleEditor;
use crate::studio_state::StudioState;

pub(crate) fn view(state: &StudioState) -> Element<'_, Message> {
    let mut content = column![
        text("Studio Projet Bezot").size(32),
        text(state.project_root.display().to_string()).size(14),
        row![
            button("Recharger").on_press(Message::ReloadContent),
            button("Nouvel article").on_press(Message::NewPost),
            button("Enregistrer l’article").on_press(Message::SavePost),
        ]
        .spacing(8),
    ]
    .spacing(12);

    if let Some(notice) = &state.notice {
        content = content.push(text(notice).size(16));
    }

    if let Some(error) = &state.error {
        content = content.push(text(format!("Erreur : {error}")).size(18));
    }

    content = content.push(
        row![
            content_list_view(state),
            post_editor_view(&state.post_editor),
        ]
        .spacing(16),
    );

    container(scrollable(content.padding(24).spacing(16)))
        .width(Fill)
        .height(Fill)
        .into()
}

fn content_list_view(state: &StudioState) -> Element<'_, Message> {
    let page_count = state.page_count();
    let visible_entries = state.visible_entries();
    let mut list = column![
        text("Contenus").size(20),
        text(format!(
            "{} résultat(s) sur {} contenu(s)",
            state.filtered_count(),
            state.entries.len()
        ))
        .size(14),
        text_input("Rechercher un id, un titre, un slug…", &state.search_query)
            .on_input(Message::SearchQueryChanged),
        row![
            filter_button("Tout", ContentKindFilter::All, state.kind_filter),
            filter_button("Pages", ContentKindFilter::Pages, state.kind_filter),
            filter_button("Articles", ContentKindFilter::Posts, state.kind_filter),
        ]
        .spacing(8),
        row![
            button("Précédent").on_press(Message::PreviousPage),
            text(format!("Page {} / {}", state.page_index + 1, page_count)),
            button("Suivant").on_press(Message::NextPage),
        ]
        .spacing(8),
    ]
    .spacing(10)
    .width(Length::FillPortion(1));

    for entry in visible_entries {
        list = list.push(entry_view(entry, state.selected_entry_id.as_deref()));
    }

    container(list).width(Length::FillPortion(1)).into()
}

fn entry_view<'a>(
    entry: &'a ContentEntry,
    selected_entry_id: Option<&str>,
) -> Element<'a, Message> {
    let selected = selected_entry_id == Some(entry.id.as_str());
    let mut block = column![
        row![
            text(content_kind_label(&entry.kind)).size(14),
            text(&entry.id).size(22),
            text(format!("[{}]", entry.status)).size(14),
            button(if selected {
                "Sélectionné"
            } else {
                "Sélectionner"
            })
            .on_press(Message::SelectEntry(entry.id.clone())),
        ]
        .spacing(12)
    ]
    .spacing(6);

    for locale in &entry.locales {
        block = block.push(text(format!(
            "{} [{}] {} /{} blocs:{}",
            locale.locale,
            locale.status,
            locale.title,
            locale.slug.trim_start_matches('/'),
            locale.block_count
        )));
    }

    container(block).padding(12).width(Fill).into()
}

fn post_editor_view(editor: &PostEditorState) -> Element<'_, Message> {
    let form = column![
        text("Éditeur d’article").size(20),
        labeled_input("Identifiant", &editor.id, Message::PostIdChanged),
        labeled_input("Date", &editor.date, Message::PostDateChanged),
        labeled_input("Statut", &editor.status, Message::PostStatusChanged),
        labeled_input("Auteur", &editor.author, Message::PostAuthorChanged),
        locale_editor_view(
            "Français",
            &editor.fr,
            LocaleEditorMessages {
                title_changed: Message::PostFrenchTitleChanged,
                slug_changed: Message::PostFrenchSlugChanged,
                description_changed: Message::PostFrenchDescriptionChanged,
                paragraph_changed: Message::PostFrenchParagraphChanged,
                affiliate_title_changed: Message::PostFrenchAffiliateTitleChanged,
                affiliate_text_changed: Message::PostFrenchAffiliateTextChanged,
                affiliate_url_changed: Message::PostFrenchAffiliateUrlChanged,
                affiliate_label_changed: Message::PostFrenchAffiliateLabelChanged,
                affiliate_disclosure_changed: Message::PostFrenchAffiliateDisclosureChanged,
            },
        ),
        locale_editor_view(
            "Anglais",
            &editor.en,
            LocaleEditorMessages {
                title_changed: Message::PostEnglishTitleChanged,
                slug_changed: Message::PostEnglishSlugChanged,
                description_changed: Message::PostEnglishDescriptionChanged,
                paragraph_changed: Message::PostEnglishParagraphChanged,
                affiliate_title_changed: Message::PostEnglishAffiliateTitleChanged,
                affiliate_text_changed: Message::PostEnglishAffiliateTextChanged,
                affiliate_url_changed: Message::PostEnglishAffiliateUrlChanged,
                affiliate_label_changed: Message::PostEnglishAffiliateLabelChanged,
                affiliate_disclosure_changed: Message::PostEnglishAffiliateDisclosureChanged,
            },
        ),
    ]
    .spacing(10)
    .width(Length::FillPortion(2));

    container(form)
        .padding(12)
        .width(Length::FillPortion(2))
        .into()
}

fn locale_editor_view<'a>(
    title: &'a str,
    editor: &'a PostLocaleEditor,
    messages: LocaleEditorMessages,
) -> Element<'a, Message> {
    column![
        text(title).size(18),
        labeled_input("Titre", &editor.title, messages.title_changed),
        labeled_input("Slug", &editor.slug, messages.slug_changed),
        labeled_input(
            "Description SEO",
            &editor.description,
            messages.description_changed
        ),
        labeled_input("Paragraphe", &editor.paragraph, messages.paragraph_changed),
        text("Monétisation / affiliation").size(16),
        labeled_input(
            "Titre encart affilié",
            &editor.affiliate_title,
            messages.affiliate_title_changed
        ),
        labeled_input(
            "Texte encart affilié",
            &editor.affiliate_text,
            messages.affiliate_text_changed
        ),
        labeled_input(
            "URL affiliée ou sponsorisée",
            &editor.affiliate_url,
            messages.affiliate_url_changed
        ),
        labeled_input(
            "Libellé du bouton",
            &editor.affiliate_label,
            messages.affiliate_label_changed
        ),
        labeled_input(
            "Mention visible",
            &editor.affiliate_disclosure,
            messages.affiliate_disclosure_changed
        ),
    ]
    .spacing(8)
    .into()
}

#[derive(Clone, Copy)]
struct LocaleEditorMessages {
    title_changed: fn(String) -> Message,
    slug_changed: fn(String) -> Message,
    description_changed: fn(String) -> Message,
    paragraph_changed: fn(String) -> Message,
    affiliate_title_changed: fn(String) -> Message,
    affiliate_text_changed: fn(String) -> Message,
    affiliate_url_changed: fn(String) -> Message,
    affiliate_label_changed: fn(String) -> Message,
    affiliate_disclosure_changed: fn(String) -> Message,
}

fn filter_button(
    label: &str,
    filter: ContentKindFilter,
    active_filter: ContentKindFilter,
) -> Element<'_, Message> {
    let label = if filter == active_filter {
        format!("✓ {label}")
    } else {
        label.to_string()
    };
    let message = match filter {
        ContentKindFilter::All => Message::ShowAllContent,
        ContentKindFilter::Pages => Message::ShowPages,
        ContentKindFilter::Posts => Message::ShowPosts,
    };

    button(text(label)).on_press(message).into()
}

fn content_kind_label(kind: &str) -> &'static str {
    match kind {
        "page" => "PAGE",
        "post" => "ARTICLE",
        _ => "CONTENU",
    }
}

fn labeled_input<'a>(
    label: &'a str,
    value: &'a str,
    on_input: fn(String) -> Message,
) -> Element<'a, Message> {
    column![
        text(label).size(14),
        text_input(label, value).on_input(on_input)
    ]
    .spacing(4)
    .into()
}
