use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Background, Border, Color, Element, Fill, Length, Shadow, Theme, Vector};

use crate::content_entry::ContentEntry;
use crate::content_kind_filter::ContentKindFilter;
use crate::message::Message;
use crate::post_editor_state::PostEditorState;
use crate::post_locale_editor::PostLocaleEditor;
use crate::post_quality::PostQualityReport;
use crate::studio_state::StudioState;

pub(crate) fn view(state: &StudioState) -> Element<'_, Message> {
    let workspace: Element<'_, Message> = if state.library_visible {
        column![content_sidebar_view(state), post_workspace_view(state)]
            .spacing(18)
            .into()
    } else {
        post_workspace_view(state)
    };

    let content = column![top_bar_view(state), status_line_view(state), workspace,]
        .spacing(18)
        .padding(22);

    container(scrollable(content))
        .width(Fill)
        .height(Fill)
        .style(shell_style)
        .into()
}

fn top_bar_view(state: &StudioState) -> Element<'_, Message> {
    let save_label = if state.edited_post_exists() {
        "Mettre à jour"
    } else {
        "Créer l’article"
    };
    let library_label = if state.library_visible {
        "Masquer bibliothèque"
    } else {
        "Voir bibliothèque"
    };
    let library_message = if state.library_visible {
        Message::HideLibrary
    } else {
        Message::ShowLibrary
    };

    card(
        row![
            column![
                text("Studio éditorial Bezot").size(34),
                text("Publier, optimiser, monétiser et préparer le site final.").size(16),
                text(state.project_root.display().to_string()).size(12),
            ]
            .spacing(6)
            .width(Length::FillPortion(2)),
            column![
                row![
                    button(library_label).on_press(library_message),
                    button("Recharger").on_press(Message::ReloadContent),
                    button("Nouvel article").on_press(Message::NewPost),
                ]
                .spacing(8),
                row![
                    button(save_label).on_press(Message::SavePost),
                    button("Préparer le site").on_press(Message::PrepareSite),
                ]
                .spacing(8),
            ]
            .spacing(8)
            .width(Length::FillPortion(1)),
        ]
        .spacing(18),
    )
}

fn status_line_view(state: &StudioState) -> Element<'_, Message> {
    let mut line = column![dashboard_view(state)].spacing(10);

    if let Some(notice) = &state.notice {
        line = line.push(notification_view("Info", notice));
    }

    if let Some(error) = &state.error {
        line = line.push(notification_view("Erreur", error));
    }

    line.into()
}

fn notification_view<'a>(label: &'a str, value: &'a str) -> Element<'a, Message> {
    card(row![text(label).size(14), text(value).size(14)].spacing(10))
}

fn dashboard_view(state: &StudioState) -> Element<'_, Message> {
    let published = state
        .entries
        .iter()
        .filter(|entry| entry.status == "published")
        .count();
    let drafts = state
        .entries
        .iter()
        .filter(|entry| entry.status == "draft")
        .count();
    let archived = state
        .entries
        .iter()
        .filter(|entry| entry.status == "archived")
        .count();
    let monetizable_posts = state
        .entries
        .iter()
        .filter(|entry| entry.kind == "post")
        .count();
    let pages = state
        .entries
        .iter()
        .filter(|entry| entry.kind == "page")
        .count();

    row![
        metric_card("Pages", pages, "site public"),
        metric_card("Articles", monetizable_posts, "blog / SEO"),
        metric_card("Publiés", published, "en ligne"),
        metric_card("Brouillons", drafts, "à compléter"),
        metric_card("Archivés", archived, "hors ligne"),
    ]
    .spacing(12)
    .into()
}

fn metric_card(label: &str, value: usize, caption: &str) -> Element<'static, Message> {
    panel(
        column![
            text(value.to_string()).size(28),
            text(label.to_string()).size(14),
            text(caption.to_string()).size(12),
        ]
        .spacing(4),
    )
}

fn content_sidebar_view(state: &StudioState) -> Element<'_, Message> {
    let page_count = state.page_count();
    let visible_entries = state.visible_entries();
    let pages = state
        .entries
        .iter()
        .filter(|entry| entry.kind == "page")
        .count();
    let posts = state
        .entries
        .iter()
        .filter(|entry| entry.kind == "post")
        .count();
    let mut list = column![
        row![
            section_title("Bibliothèque"),
            status_chip(format!("{} pages", pages)),
            status_chip(format!("{} articles", posts)),
        ]
        .spacing(8),
        text(format!(
            "{} résultat(s) sur {} contenu(s)",
            state.filtered_count(),
            state.entries.len()
        ))
        .size(13),
        text_input("Rechercher id, titre, slug…", &state.search_query)
            .on_input(Message::SearchQueryChanged)
            .padding(12),
        row![
            library_filter_card(
                "Tout",
                state.entries.len(),
                ContentKindFilter::All,
                state.kind_filter
            ),
            library_filter_card("Pages", pages, ContentKindFilter::Pages, state.kind_filter),
            library_filter_card(
                "Articles",
                posts,
                ContentKindFilter::Posts,
                state.kind_filter
            ),
        ]
        .spacing(8),
        row![
            filter_button("Tout", ContentKindFilter::All, state.kind_filter),
            filter_button("Pages", ContentKindFilter::Pages, state.kind_filter),
            filter_button("Articles", ContentKindFilter::Posts, state.kind_filter),
        ]
        .spacing(8),
    ]
    .spacing(12);

    for entry in visible_entries {
        list = list.push(entry_card(entry, state.selected_entry_id.as_deref()));
    }

    list = list.push(
        row![
            button("Précédent").on_press(Message::PreviousPage),
            text(format!("Page {} / {}", state.page_index + 1, page_count)).size(13),
            button("Suivant").on_press(Message::NextPage),
        ]
        .spacing(8),
    );

    container(list)
        .padding(18)
        .width(Fill)
        .style(card_style)
        .into()
}

fn entry_card<'a>(
    entry: &'a ContentEntry,
    selected_entry_id: Option<&str>,
) -> Element<'a, Message> {
    let selected = selected_entry_id == Some(entry.id.as_str());
    let primary_locale = entry
        .locales
        .iter()
        .find(|locale| locale.locale == "fr-fr")
        .or_else(|| entry.locales.first());
    let title = primary_locale
        .map(|locale| locale.title.as_str())
        .unwrap_or("(sans titre)");
    let slug = primary_locale
        .map(|locale| locale.slug.as_str())
        .unwrap_or("");
    let language_summary = entry
        .locales
        .iter()
        .map(|locale| locale.locale.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let mut locales = column![];

    for locale in &entry.locales {
        locales = locales.push(
            text(format!(
                "{} · {} · /{} · {} bloc(s)",
                locale.locale,
                status_label(&locale.status),
                locale.slug.trim_start_matches('/'),
                locale.block_count
            ))
            .size(12),
        );
    }

    panel(
        column![
            row![
                status_chip(content_kind_label(&entry.kind).to_string()),
                status_chip(status_label(&entry.status).to_string()),
                status_chip(language_summary),
            ]
            .spacing(8),
            text(title).size(18),
            text(format!("{} · /{}", entry.id, slug.trim_start_matches('/'))).size(12),
            locales.spacing(4),
            button(if selected { "Sélectionné" } else { "Ouvrir" })
                .on_press(Message::SelectEntry(entry.id.clone())),
        ]
        .spacing(8),
    )
}

fn library_filter_card(
    label: &str,
    count: usize,
    filter: ContentKindFilter,
    active_filter: ContentKindFilter,
) -> Element<'static, Message> {
    let title = if filter == active_filter {
        format!("✓ {label}")
    } else {
        label.to_string()
    };
    let message = match filter {
        ContentKindFilter::All => Message::ShowAllContent,
        ContentKindFilter::Pages => Message::ShowPages,
        ContentKindFilter::Posts => Message::ShowPosts,
    };

    container(
        column![
            text(count.to_string()).size(22),
            button(text(title)).on_press(message),
        ]
        .spacing(4),
    )
    .padding(10)
    .width(Length::FillPortion(1))
    .style(panel_style)
    .into()
}

fn post_workspace_view(state: &StudioState) -> Element<'_, Message> {
    let editor = &state.post_editor;
    let report = PostQualityReport::analyze(editor);
    let save_label = if state.edited_post_exists() {
        "Mettre à jour l’article existant"
    } else {
        "Créer un nouvel article brouillon"
    };

    container(
        column![
            row![
                column![
                    section_title("Article"),
                    current_article_banner(state, save_label),
                    quick_actions_view(),
                    publication_workflow_view(editor),
                    base_metadata_view(editor),
                    locale_editor_view(
                        "Version française",
                        &editor.fr,
                        LocaleEditorMessages::french(),
                    ),
                    locale_editor_view(
                        "Version anglaise",
                        &editor.en,
                        LocaleEditorMessages::english()
                    ),
                ]
                .spacing(14)
                .width(Length::FillPortion(2)),
                column![
                    section_title("Pilotage"),
                    editorial_score_view(&report),
                    publication_preview_view(editor, &report),
                    recommendations_view(&report),
                ]
                .spacing(14)
                .width(Length::FillPortion(1)),
            ]
            .spacing(18),
        ]
        .spacing(14),
    )
    .padding(18)
    .width(Fill)
    .height(Fill)
    .style(card_style)
    .into()
}

fn current_article_banner<'a>(state: &'a StudioState, save_label: &'a str) -> Element<'a, Message> {
    let selected = state
        .selected_entry_id
        .as_deref()
        .unwrap_or("aucun article sélectionné");
    let mode = if state.edited_post_exists() {
        "édition"
    } else {
        "création"
    };

    panel(
        column![
            text(format!("Mode : {mode}")).size(16),
            text(format!("Article courant : {}", state.post_editor.id)).size(13),
            text(format!("Sélection bibliothèque : {selected}")).size(13),
            text(format!("Action de sauvegarde : {save_label}")).size(13),
        ]
        .spacing(6),
    )
}

fn quick_actions_view() -> Element<'static, Message> {
    panel(
        column![
            text("Accélérateurs").size(16),
            row![
                button("Modèle SEO").on_press(Message::ApplySeoTemplate),
                button("Modèle monétisé").on_press(Message::ApplyMonetizedTemplate),
            ]
            .spacing(8),
            button("Vider affiliation").on_press(Message::ClearAffiliateFields),
            text("Les modèles donnent une structure. Le fond reste à relire et adapter.").size(12),
        ]
        .spacing(8),
    )
}

fn publication_workflow_view(editor: &PostEditorState) -> Element<'_, Message> {
    panel(
        column![
            text("Workflow de publication").size(16),
            row![
                status_chip(format!("Statut : {}", editor.status)),
                status_chip(format!("Auteur : {}", editor.author)),
            ]
            .spacing(8),
            row![
                button("Brouillon").on_press(Message::MarkDraft),
                button("Publié").on_press(Message::MarkPublished),
                button("Archivé").on_press(Message::MarkArchived),
            ]
            .spacing(8),
        ]
        .spacing(10),
    )
}

fn base_metadata_view(editor: &PostEditorState) -> Element<'_, Message> {
    panel(
        column![
            text("Métadonnées").size(16),
            row![
                metadata_value("Identifiant stable", &editor.id),
                labeled_input("Date", &editor.date, Message::PostDateChanged),
            ]
            .spacing(10),
            row![
                labeled_input("Statut", &editor.status, Message::PostStatusChanged),
                labeled_input("Auteur", &editor.author, Message::PostAuthorChanged),
            ]
            .spacing(10),
        ]
        .spacing(10),
    )
}

fn metadata_value<'a>(label: &'a str, value: &'a str) -> Element<'a, Message> {
    column![text(label).size(12), text(value).size(14),]
        .spacing(4)
        .width(Fill)
        .into()
}

fn locale_editor_view<'a>(
    title: &'a str,
    editor: &'a PostLocaleEditor,
    messages: LocaleEditorMessages,
) -> Element<'a, Message> {
    panel(
        column![
            text(title).size(18),
            row![
                labeled_input("Titre", &editor.title, messages.title_changed),
                labeled_input("Slug", &editor.slug, messages.slug_changed),
            ]
            .spacing(10),
            labeled_input(
                "Description SEO",
                &editor.description,
                messages.description_changed
            ),
            labeled_input(
                "Paragraphe principal",
                &editor.paragraph,
                messages.paragraph_changed
            ),
            monetization_editor_view(editor, messages),
        ]
        .spacing(10),
    )
}

fn monetization_editor_view<'a>(
    editor: &'a PostLocaleEditor,
    messages: LocaleEditorMessages,
) -> Element<'a, Message> {
    container(
        column![
            text("Monétisation / affiliation").size(16),
            row![
                labeled_input(
                    "Titre encart",
                    &editor.affiliate_title,
                    messages.affiliate_title_changed
                ),
                labeled_input(
                    "Libellé bouton",
                    &editor.affiliate_label,
                    messages.affiliate_label_changed
                ),
            ]
            .spacing(10),
            labeled_input(
                "URL affiliée ou sponsorisée",
                &editor.affiliate_url,
                messages.affiliate_url_changed
            ),
            labeled_input(
                "Texte encart",
                &editor.affiliate_text,
                messages.affiliate_text_changed
            ),
            labeled_input(
                "Mention visible",
                &editor.affiliate_disclosure,
                messages.affiliate_disclosure_changed
            ),
        ]
        .spacing(8),
    )
    .padding(14)
    .width(Fill)
    .style(accent_panel_style)
    .into()
}

fn editorial_score_view(report: &PostQualityReport) -> Element<'static, Message> {
    let checks = report
        .checks
        .iter()
        .map(|check| checklist_line(check.label, check.passed))
        .collect::<Vec<_>>()
        .join("\n");
    let reading = format!(
        "{} min · FR {} mots · EN {} mots",
        report.reading_minutes, report.french_words, report.english_words
    );

    panel(
        column![
            text(format!("Score éditorial {}", report.score)).size(30),
            text("/ 100").size(14),
            text(reading).size(13),
            text(checks).size(13),
        ]
        .spacing(8),
    )
}

fn publication_preview_view(
    editor: &PostEditorState,
    report: &PostQualityReport,
) -> Element<'static, Message> {
    let advised_state = if report.score >= 75 {
        "publiable après relecture"
    } else {
        "à renforcer avant publication"
    };

    panel(
        column![
            text("Aperçu publication").size(16),
            status_chip(format!("FR /{}", editor.fr.slug.trim_start_matches('/'))),
            status_chip(format!("EN /{}", editor.en.slug.trim_start_matches('/'))),
            status_chip(format!("Monétisation : {}", monetization_label(editor))),
            text(format!("Conseil : {advised_state}")).size(13),
        ]
        .spacing(8),
    )
}

fn recommendations_view(report: &PostQualityReport) -> Element<'static, Message> {
    let mut list = column![text("Recommandations").size(16)].spacing(8);

    for recommendation in &report.recommendations {
        list = list.push(text(format!("• {recommendation}")).size(13));
    }

    panel(list)
}

fn labeled_input<'a>(
    label: &'a str,
    value: &'a str,
    on_input: fn(String) -> Message,
) -> Element<'a, Message> {
    column![
        text(label).size(12),
        text_input(label, value).on_input(on_input).padding(12),
    ]
    .spacing(5)
    .width(Fill)
    .into()
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

fn section_title(label: &str) -> Element<'static, Message> {
    text(label.to_string()).size(20).into()
}

fn status_chip(label: String) -> Element<'static, Message> {
    container(text(label).size(12))
        .padding([6, 10])
        .style(chip_style)
        .into()
}

fn checklist_line(label: &str, valid: bool) -> String {
    let marker = if valid { "✓" } else { "!" };
    format!("{marker} {label}")
}

fn monetization_label(editor: &PostEditorState) -> &'static str {
    let fr = !editor.fr.affiliate_url.trim().is_empty();
    let en = !editor.en.affiliate_url.trim().is_empty();

    match (fr, en) {
        (true, true) => "FR + EN",
        (true, false) => "FR uniquement",
        (false, true) => "EN uniquement",
        (false, false) => "aucun encart",
    }
}

fn content_kind_label(kind: &str) -> &'static str {
    match kind {
        "page" => "Page",
        "post" => "Article",
        _ => "Contenu",
    }
}

fn status_label(status: &str) -> &'static str {
    match status {
        "published" => "Publié",
        "draft" => "Brouillon",
        "archived" => "Archivé",
        _ => "Inconnu",
    }
}

fn card<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .padding(18)
        .width(Fill)
        .style(card_style)
        .into()
}

fn panel<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .padding(14)
        .width(Fill)
        .style(panel_style)
        .into()
}

fn shell_style(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(color(226, 232, 240)),
        background: Some(Background::Color(color(15, 23, 42))),
        ..Default::default()
    }
}

fn card_style(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(color(226, 232, 240)),
        background: Some(Background::Color(color_alpha(30, 41, 59, 0.74))),
        border: Border::default()
            .rounded(24)
            .width(1)
            .color(color_alpha(148, 163, 184, 0.22)),
        shadow: Shadow {
            color: color_alpha(2, 6, 23, 0.24),
            offset: Vector::new(0.0, 18.0),
            blur_radius: 42.0,
        },
        ..Default::default()
    }
}

fn panel_style(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(color(226, 232, 240)),
        background: Some(Background::Color(color_alpha(15, 23, 42, 0.64))),
        border: Border::default()
            .rounded(18)
            .width(1)
            .color(color_alpha(148, 163, 184, 0.16)),
        ..Default::default()
    }
}

fn accent_panel_style(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(color(254, 243, 199)),
        background: Some(Background::Color(color_alpha(120, 53, 15, 0.24))),
        border: Border::default()
            .rounded(18)
            .width(1)
            .color(color_alpha(251, 191, 36, 0.32)),
        ..Default::default()
    }
}

fn chip_style(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(color(203, 213, 225)),
        background: Some(Background::Color(color_alpha(2, 6, 23, 0.42))),
        border: Border::default()
            .rounded(999)
            .width(1)
            .color(color_alpha(148, 163, 184, 0.2)),
        ..Default::default()
    }
}

fn color(red: u8, green: u8, blue: u8) -> Color {
    Color::from_rgb8(red, green, blue)
}

fn color_alpha(red: u8, green: u8, blue: u8, alpha: f32) -> Color {
    Color {
        a: alpha,
        ..Color::from_rgb8(red, green, blue)
    }
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

impl LocaleEditorMessages {
    fn french() -> Self {
        Self {
            title_changed: Message::PostFrenchTitleChanged,
            slug_changed: Message::PostFrenchSlugChanged,
            description_changed: Message::PostFrenchDescriptionChanged,
            paragraph_changed: Message::PostFrenchParagraphChanged,
            affiliate_title_changed: Message::PostFrenchAffiliateTitleChanged,
            affiliate_text_changed: Message::PostFrenchAffiliateTextChanged,
            affiliate_url_changed: Message::PostFrenchAffiliateUrlChanged,
            affiliate_label_changed: Message::PostFrenchAffiliateLabelChanged,
            affiliate_disclosure_changed: Message::PostFrenchAffiliateDisclosureChanged,
        }
    }

    fn english() -> Self {
        Self {
            title_changed: Message::PostEnglishTitleChanged,
            slug_changed: Message::PostEnglishSlugChanged,
            description_changed: Message::PostEnglishDescriptionChanged,
            paragraph_changed: Message::PostEnglishParagraphChanged,
            affiliate_title_changed: Message::PostEnglishAffiliateTitleChanged,
            affiliate_text_changed: Message::PostEnglishAffiliateTextChanged,
            affiliate_url_changed: Message::PostEnglishAffiliateUrlChanged,
            affiliate_label_changed: Message::PostEnglishAffiliateLabelChanged,
            affiliate_disclosure_changed: Message::PostEnglishAffiliateDisclosureChanged,
        }
    }
}
