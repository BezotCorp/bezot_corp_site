use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Element, Fill, Length};

use common::{PostEditorState, PostLocaleEditor, PostQualityReport};

use crate::ai_view::ai_page_view;
use crate::content_entry::ContentEntry;
use crate::content_kind_filter::ContentKindFilter;
use crate::editor_target::EditorTarget;
use crate::locale::Locale;
use crate::message::Message;
use crate::page::Page;
use crate::page_workspace_view::page_workspace_view;
use crate::post_field::PostField;
use crate::studio_state::StudioState;
use crate::styles::{
    accent_color, accent_panel_style, card_style, danger_color, info_color, panel_style,
    shell_style, success_color, warning_color,
};
use crate::widgets::{
    card, checklist_line, labeled_input, panel, section_title, stat_bar, status_chip,
};

pub(crate) fn view(state: &StudioState) -> Element<'_, Message> {
    let page_content = match state.current_page {
        Page::Dashboard => dashboard_page_view(state),
        Page::Library => content_sidebar_view(state),
        Page::Editor => column![
            editor_target_toggle(state.editor_target),
            match state.editor_target {
                EditorTarget::Post => post_workspace_view(state),
                EditorTarget::Page => page_workspace_view(state),
            },
        ]
        .spacing(14)
        .into(),
        Page::Ai => ai_page_view(state),
        Page::SiteTools => site_tools_page_view(state),
    };

    let content = column![
        header_view(state),
        nav_view(state.current_page),
        notifications_view(state),
        page_content,
    ]
    .spacing(18)
    .padding(22);

    container(scrollable(content))
        .width(Fill)
        .height(Fill)
        .style(shell_style)
        .into()
}

fn header_view(state: &StudioState) -> Element<'_, Message> {
    card(
        column![
            text("Studio éditorial Bezot").size(34),
            text("Publier, optimiser, monétiser et préparer le site final.").size(16),
            text(state.project_root.display().to_string()).size(12),
        ]
        .spacing(6),
    )
}

fn nav_view(current_page: Page) -> Element<'static, Message> {
    let mut items = row![].spacing(8);

    for page in Page::ALL {
        let label = if page == current_page {
            format!("→ {}", page.label())
        } else {
            page.label().to_string()
        };
        items = items.push(button(text(label)).on_press(Message::Navigate(page)));
    }

    card(items)
}

fn editor_target_toggle(current: EditorTarget) -> Element<'static, Message> {
    let article_label = if current == EditorTarget::Post {
        "→ Article"
    } else {
        "Article"
    };
    let page_label = if current == EditorTarget::Page {
        "→ Page"
    } else {
        "Page"
    };

    card(
        row![
            text("Type de contenu :").size(13),
            button(article_label).on_press(Message::ShowEditorTarget(EditorTarget::Post)),
            button(page_label).on_press(Message::ShowEditorTarget(EditorTarget::Page)),
        ]
        .spacing(8),
    )
}

fn notifications_view(state: &StudioState) -> Element<'_, Message> {
    let mut line = column![].spacing(10);

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

fn dashboard_page_view(state: &StudioState) -> Element<'_, Message> {
    let published = count_entries(state, |entry| entry.status == "published");
    let drafts = count_entries(state, |entry| entry.status == "draft");
    let archived = count_entries(state, |entry| entry.status == "archived");
    let pages = count_entries(state, |entry| entry.kind == "page");
    let posts = count_entries(state, |entry| entry.kind == "post");
    let posts_total = posts.max(1);
    let posts_with_both_locales = count_entries(state, |entry| {
        entry.kind == "post" && entry.locales.len() >= 2
    });
    let posts_incomplete = posts.saturating_sub(posts_with_both_locales);
    let status_max = published.max(drafts).max(archived);
    let kind_max = pages.max(posts);

    column![
        section_title("Tableau de bord"),
        row![
            metric_card("Pages", pages, "site public"),
            metric_card("Articles", posts, "blog / SEO"),
            metric_card("Publiés", published, "en ligne"),
            metric_card("Brouillons", drafts, "à compléter"),
            metric_card("Archivés", archived, "hors ligne"),
        ]
        .spacing(12),
        row![
            panel(
                column![
                    text("Statuts des contenus").size(16),
                    stat_bar(
                        "Publiés".to_string(),
                        published,
                        status_max,
                        success_color()
                    ),
                    stat_bar(
                        "Brouillons".to_string(),
                        drafts,
                        status_max,
                        warning_color()
                    ),
                    stat_bar("Archivés".to_string(), archived, status_max, danger_color()),
                ]
                .spacing(10)
            ),
            panel(
                column![
                    text("Répartition du contenu").size(16),
                    stat_bar("Pages".to_string(), pages, kind_max, info_color()),
                    stat_bar("Articles".to_string(), posts, kind_max, accent_color()),
                ]
                .spacing(10)
            ),
            panel(
                column![
                    text("Couverture linguistique des articles").size(16),
                    stat_bar(
                        "FR + EN".to_string(),
                        posts_with_both_locales,
                        posts_total,
                        success_color()
                    ),
                    stat_bar(
                        "Incomplets".to_string(),
                        posts_incomplete,
                        posts_total,
                        warning_color()
                    ),
                ]
                .spacing(10)
            ),
        ]
        .spacing(14),
        draft_posts_panel(state),
    ]
    .spacing(18)
    .into()
}

fn count_entries(state: &StudioState, predicate: impl Fn(&ContentEntry) -> bool) -> usize {
    state
        .entries
        .iter()
        .filter(|entry| predicate(entry))
        .count()
}

fn draft_posts_panel(state: &StudioState) -> Element<'_, Message> {
    let drafts = state
        .entries
        .iter()
        .filter(|entry| entry.kind == "post" && entry.status == "draft");

    let mut list = column![text("Brouillons à terminer").size(16)].spacing(8);
    let mut has_drafts = false;

    for entry in drafts {
        has_drafts = true;
        let title = entry
            .locales
            .iter()
            .find(|locale| locale.locale == "fr-fr")
            .or_else(|| entry.locales.first())
            .map(|locale| locale.title.as_str())
            .unwrap_or("(sans titre)");

        list = list.push(
            row![
                text(format!("{title} · {}", entry.id)).size(13),
                button("Ouvrir").on_press(Message::SelectEntry(entry.id.clone())),
            ]
            .spacing(10),
        );
    }

    if !has_drafts {
        list = list.push(text("Aucun brouillon en attente.").size(13));
    }

    panel(list)
}

fn site_tools_page_view(state: &StudioState) -> Element<'_, Message> {
    panel(
        column![
            section_title("Outils du site"),
            text(format!(
                "Projet : {}",
                state.project_root.display()
            ))
            .size(13),
            text("Génère un site final prêt pour la mise en production : build complet, prérendu de chaque route, puis vérifications HTML, SEO et accessibilité.").size(13),
            button("Préparer le site").on_press(Message::PrepareSite),
        ]
        .spacing(12),
    )
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
            button("Recharger").on_press(Message::ReloadContent),
            button("Nouveau brouillon").on_press(Message::NewPost),
            button("Nouvelle page").on_press(Message::NewPage),
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
        "Enregistrer ce nouveau brouillon"
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
                    locale_editor_view("Version française", &editor.fr, Locale::French),
                    locale_editor_view("Version anglaise", &editor.en, Locale::English),
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
            row![
                button("Nouveau brouillon").on_press(Message::NewPost),
                button(save_label).on_press(Message::SavePost),
            ]
            .spacing(8),
        ]
        .spacing(8),
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
    locale: Locale,
) -> Element<'a, Message> {
    panel(
        column![
            text(title).size(18),
            row![
                field_input("Titre", &editor.title, locale, PostField::Title),
                field_input("Slug", &editor.slug, locale, PostField::Slug),
            ]
            .spacing(10),
            field_input(
                "Sous-titre (affiché sous le titre — utilisé pour la mention « généré par IA » sur les édito IA)",
                &editor.subtitle,
                locale,
                PostField::Subtitle
            ),
            field_input(
                "Description SEO",
                &editor.description,
                locale,
                PostField::Description
            ),
            paragraphs_editor_view(&editor.paragraphs, locale),
            monetization_editor_view(editor, locale),
        ]
        .spacing(10),
    )
}

fn paragraphs_editor_view(paragraphs: &[String], locale: Locale) -> Element<'_, Message> {
    let mut list = column![text("Paragraphes").size(14)].spacing(8);
    let count = paragraphs.len();

    for (index, paragraph) in paragraphs.iter().enumerate() {
        let mut controls = row![].spacing(6);

        if index > 0 {
            controls = controls.push(button("↑").on_press(Message::MoveParagraphUp(locale, index)));
        }
        if index + 1 < count {
            controls =
                controls.push(button("↓").on_press(Message::MoveParagraphDown(locale, index)));
        }
        controls =
            controls.push(button("Supprimer").on_press(Message::RemoveParagraph(locale, index)));

        list = list.push(
            column![
                labeled_input(
                    format!("Paragraphe {}", index + 1),
                    paragraph,
                    move |value| Message::ParagraphChanged(locale, index, value)
                ),
                controls,
            ]
            .spacing(4),
        );
    }

    list = list.push(button("Ajouter un paragraphe").on_press(Message::AddParagraph(locale)));

    list.into()
}

fn monetization_editor_view<'a>(
    editor: &'a PostLocaleEditor,
    locale: Locale,
) -> Element<'a, Message> {
    container(
        column![
            text("Monétisation / affiliation").size(16),
            row![
                field_input(
                    "Titre encart",
                    &editor.affiliate_title,
                    locale,
                    PostField::AffiliateTitle
                ),
                field_input(
                    "Libellé bouton",
                    &editor.affiliate_label,
                    locale,
                    PostField::AffiliateLabel
                ),
            ]
            .spacing(10),
            field_input(
                "URL affiliée ou sponsorisée",
                &editor.affiliate_url,
                locale,
                PostField::AffiliateUrl
            ),
            field_input(
                "Texte encart",
                &editor.affiliate_text,
                locale,
                PostField::AffiliateText
            ),
            field_input(
                "Mention visible",
                &editor.affiliate_disclosure,
                locale,
                PostField::AffiliateDisclosure
            ),
        ]
        .spacing(8),
    )
    .padding(14)
    .width(Fill)
    .style(accent_panel_style)
    .into()
}

fn field_input<'a>(
    label: &'a str,
    value: &'a str,
    locale: Locale,
    field: PostField,
) -> Element<'a, Message> {
    labeled_input(label, value, move |value| {
        Message::PostFieldChanged(locale, field, value)
    })
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
