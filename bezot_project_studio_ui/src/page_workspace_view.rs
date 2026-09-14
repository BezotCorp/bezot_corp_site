use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill};

use common::{CardItem, PageBlock, PageBlockKind, PageLocaleEditor};

use crate::card_item_field::CardItemField;
use crate::locale::Locale;
use crate::message::Message;
use crate::page_block_field::PageBlockField;
use crate::page_field::PageField;
use crate::preview_view::real_preview_panel;
use crate::studio_state::StudioState;
use crate::styles::accent_panel_style;
use crate::widgets::{
    delete_button, labeled_input, panel, save_button_label, section_title, status_chip,
};

pub(crate) fn page_workspace_view(state: &StudioState) -> Element<'_, Message> {
    let editor = &state.page_editor;
    let will_publish = editor.fr.status == "published" || editor.en.status == "published";
    let save_label = save_button_label(
        state.saving,
        will_publish,
        state.edited_page_exists(),
        "Mettre à jour la page existante",
        "Enregistrer cette nouvelle page",
    );

    container(
        column![
            section_title("Page"),
            current_page_banner(state, save_label),
            real_preview_panel(state),
            page_locale_view("Version française", &editor.fr, Locale::French),
            page_locale_view("Version anglaise", &editor.en, Locale::English),
        ]
        .spacing(14),
    )
    .padding(18)
    .width(Fill)
    .height(Fill)
    .style(crate::styles::card_style)
    .into()
}

fn current_page_banner<'a>(state: &'a StudioState, save_label: String) -> Element<'a, Message> {
    let selected = state
        .selected_entry_id
        .as_deref()
        .unwrap_or("aucune page sélectionnée");
    let mode = if state.edited_page_exists() {
        "édition"
    } else {
        "création"
    };

    panel(
        column![
            text(format!("Mode : {mode}")).size(16),
            row![
                metadata_input(
                    "Identifiant (id)",
                    &state.page_editor.id,
                    Message::PageIdChanged
                ),
                metadata_value("Sélection bibliothèque", selected),
            ]
            .spacing(10),
            row![
                button("Nouvelle page").on_press(Message::NewPage),
                button(text(save_label))
                    .on_press_maybe((!state.saving).then_some(Message::SavePage)),
                delete_button(
                    state.edited_page_exists(),
                    state.confirm_delete,
                    Message::DeletePage
                ),
            ]
            .spacing(8),
        ]
        .spacing(8),
    )
}

fn metadata_input<'a>(
    label: &'a str,
    value: &'a str,
    on_input: fn(String) -> Message,
) -> Element<'a, Message> {
    labeled_input(label, value, on_input)
}

fn metadata_value<'a>(label: &'a str, value: &'a str) -> Element<'a, Message> {
    column![text(label).size(12), text(value).size(14)]
        .spacing(4)
        .width(Fill)
        .into()
}

fn page_locale_view<'a>(
    title: &'a str,
    editor: &'a PageLocaleEditor,
    locale: Locale,
) -> Element<'a, Message> {
    let (draft_message, published_message) = (
        Message::MarkPageDraft(locale),
        Message::MarkPagePublished(locale),
    );

    panel(
        column![
            row![
                text(title).size(18),
                status_chip(format!("Statut : {}", editor.status)),
            ]
            .spacing(10),
            row![
                button("Brouillon").on_press(draft_message),
                button("Publié").on_press(published_message),
            ]
            .spacing(8),
            row![
                page_field_input("Slug", &editor.slug, locale, PageField::Slug),
                page_field_input(
                    "Date de mise à jour",
                    &editor.updated_at,
                    locale,
                    PageField::UpdatedAt
                ),
            ]
            .spacing(10),
            seo_editor_view(editor, locale),
            reading_preview_view(&editor.blocks),
            block_list_view(locale, &editor.blocks),
        ]
        .spacing(10),
    )
}

fn seo_editor_view(editor: &PageLocaleEditor, locale: Locale) -> Element<'_, Message> {
    container(
        column![
            text("SEO").size(16),
            row![
                page_field_input("Titre SEO", &editor.title, locale, PageField::Title),
                page_field_input("Robots", &editor.robots, locale, PageField::Robots),
            ]
            .spacing(10),
            page_field_input(
                "Description SEO",
                &editor.description,
                locale,
                PageField::Description
            ),
            row![
                page_field_input("Titre OG", &editor.og_title, locale, PageField::OgTitle),
                page_field_input(
                    "Description OG",
                    &editor.og_description,
                    locale,
                    PageField::OgDescription
                ),
            ]
            .spacing(10),
            page_field_input("Image OG", &editor.og_image, locale, PageField::OgImage),
        ]
        .spacing(8),
    )
    .padding(14)
    .width(Fill)
    .style(accent_panel_style)
    .into()
}

fn page_field_input<'a>(
    label: &'a str,
    value: &'a str,
    locale: Locale,
    field: PageField,
) -> Element<'a, Message> {
    labeled_input(label, value, move |value| {
        Message::PageFieldChanged(locale, field, value)
    })
}

/// A structured reading preview of every block in order, e.g. so an editor
/// can proofread flow and framing without switching to the block-editing
/// form below. Not the site's actual CSS-styled output — the build pipeline
/// excludes unpublished pages from prerendering entirely, so there is no
/// live URL to preview a draft page at yet.
fn reading_preview_view(blocks: &[PageBlock]) -> Element<'_, Message> {
    let mut list = column![text("Aperçu de lecture").size(16)].spacing(8);

    if blocks.is_empty() {
        list = list.push(text("Aucun bloc pour le moment.").size(13));
        return panel(list);
    }

    for block in blocks {
        list = list.push(block_reading_preview(block));
    }

    panel(list)
}

fn block_reading_preview(block: &PageBlock) -> Element<'_, Message> {
    match block {
        PageBlock::Hero { title, subtitle } => {
            let mut content = column![text(title.clone()).size(20)].spacing(4);
            if !subtitle.trim().is_empty() {
                content = content.push(text(subtitle.clone()).size(13));
            }
            content.into()
        }
        PageBlock::Paragraph { text: value } => text(value.clone()).size(13).into(),
        PageBlock::MailLink { email, label } => {
            let shown_label = if label.trim().is_empty() {
                email
            } else {
                label
            };
            text(format!("✉ {shown_label} ({email})")).size(13).into()
        }
        PageBlock::CardGrid { items } => {
            let mut grid = column![].spacing(6);
            for item in items {
                grid = grid.push(
                    column![
                        text(item.title.clone()).size(14),
                        text(item.text.clone()).size(12),
                    ]
                    .spacing(2),
                );
            }
            grid.into()
        }
        PageBlock::AffiliateCallout {
            title,
            text: body,
            label,
            disclosure,
            ..
        } => container(
            column![
                text(disclosure.clone()).size(11),
                text(title.clone()).size(15),
                text(body.clone()).size(13),
                status_chip(label.clone()),
            ]
            .spacing(6),
        )
        .padding(10)
        .width(Fill)
        .style(accent_panel_style)
        .into(),
        PageBlock::PostList { .. } => text("[Liste d’articles publiés]").size(12).into(),
    }
}

fn block_list_view(locale: Locale, blocks: &[PageBlock]) -> Element<'_, Message> {
    let mut list = column![text("Blocs de contenu").size(16)].spacing(10);

    if blocks.is_empty() {
        list = list.push(text("Aucun bloc pour le moment.").size(13));
    }

    for (index, block) in blocks.iter().enumerate() {
        list = list.push(block_editor_view(locale, index, block, blocks.len()));
    }

    let mut add_row = row![text("Ajouter :").size(13)].spacing(8);
    for kind in PageBlockKind::ALL {
        add_row = add_row.push(button(kind.label()).on_press(Message::AddPageBlock(locale, kind)));
    }

    list = list.push(add_row);

    list.into()
}

fn block_editor_view(
    locale: Locale,
    index: usize,
    block: &PageBlock,
    block_count: usize,
) -> Element<'_, Message> {
    let fields = block_fields_view(locale, index, block);

    panel(
        column![
            row![
                status_chip(block.kind().label().to_string()),
                block_move_controls(locale, index, block_count),
                button("Supprimer").on_press(Message::RemovePageBlock(locale, index)),
            ]
            .spacing(8),
            fields,
        ]
        .spacing(10),
    )
}

fn block_move_controls(
    locale: Locale,
    index: usize,
    block_count: usize,
) -> Element<'static, Message> {
    let mut controls = row![].spacing(6);

    if index > 0 {
        controls = controls.push(button("↑").on_press(Message::MovePageBlockUp(locale, index)));
    }

    if index + 1 < block_count {
        controls = controls.push(button("↓").on_press(Message::MovePageBlockDown(locale, index)));
    }

    controls.into()
}

fn block_fields_view(locale: Locale, index: usize, block: &PageBlock) -> Element<'_, Message> {
    match block {
        PageBlock::Hero { title, subtitle } => column![
            block_text_input("Titre", title, locale, index, PageBlockField::Title),
            block_text_input(
                "Sous-titre",
                subtitle,
                locale,
                index,
                PageBlockField::Subtitle
            ),
        ]
        .spacing(8)
        .into(),
        PageBlock::Paragraph { text: value } => {
            block_text_input("Texte", value, locale, index, PageBlockField::Text)
        }
        PageBlock::MailLink { email, label } => row![
            block_text_input("Email", email, locale, index, PageBlockField::Email),
            block_text_input("Libellé", label, locale, index, PageBlockField::Label),
        ]
        .spacing(10)
        .into(),
        PageBlock::AffiliateCallout {
            title,
            text: value,
            url,
            label,
            disclosure,
        } => column![
            row![
                block_text_input("Titre", title, locale, index, PageBlockField::Title),
                block_text_input(
                    "Libellé bouton",
                    label,
                    locale,
                    index,
                    PageBlockField::Label
                ),
            ]
            .spacing(10),
            block_text_input("URL (https://)", url, locale, index, PageBlockField::Url),
            block_text_input("Texte", value, locale, index, PageBlockField::Text),
            block_text_input(
                "Mention visible",
                disclosure,
                locale,
                index,
                PageBlockField::Disclosure
            ),
        ]
        .spacing(8)
        .into(),
        PageBlock::CardGrid { items } => card_grid_editor_view(locale, index, items),
        PageBlock::PostList {
            limit,
            page,
            show_description,
            show_author,
            show_date,
        } => post_list_editor_view(
            locale,
            index,
            *limit,
            *page,
            *show_description,
            *show_author,
            *show_date,
        ),
    }
}

fn block_text_input<'a>(
    label: &'a str,
    value: &'a str,
    locale: Locale,
    block_index: usize,
    field: PageBlockField,
) -> Element<'a, Message> {
    labeled_input(label, value, move |value| {
        Message::PageBlockTextChanged(locale, block_index, field, value)
    })
}

fn card_grid_editor_view(
    locale: Locale,
    block_index: usize,
    items: &[CardItem],
) -> Element<'_, Message> {
    let mut list = column![text("Cartes").size(14)].spacing(8);

    for (item_index, item) in items.iter().enumerate() {
        list = list.push(
            row![
                card_item_input(
                    "Titre",
                    &item.title,
                    locale,
                    block_index,
                    item_index,
                    CardItemField::Title
                ),
                card_item_input(
                    "Texte",
                    &item.text,
                    locale,
                    block_index,
                    item_index,
                    CardItemField::Text
                ),
                button("Retirer").on_press(Message::RemoveCardItem(
                    locale,
                    block_index,
                    item_index
                )),
            ]
            .spacing(10),
        );
    }

    list =
        list.push(button("Ajouter une carte").on_press(Message::AddCardItem(locale, block_index)));

    list.into()
}

fn card_item_input<'a>(
    label: &'a str,
    value: &'a str,
    locale: Locale,
    block_index: usize,
    item_index: usize,
    field: CardItemField,
) -> Element<'a, Message> {
    labeled_input(label, value, move |value| {
        Message::CardItemFieldChanged(locale, block_index, item_index, field, value)
    })
}

#[allow(clippy::too_many_arguments)]
fn post_list_editor_view(
    locale: Locale,
    index: usize,
    limit: Option<u32>,
    page: Option<u32>,
    show_description: bool,
    show_author: bool,
    show_date: bool,
) -> Element<'static, Message> {
    column![
        row![
            number_input("Limite", limit, locale, index, PageBlockField::Limit),
            number_input("Page", page, locale, index, PageBlockField::Page),
        ]
        .spacing(10),
        row![
            flag_toggle(
                "Afficher la description",
                show_description,
                locale,
                index,
                PageBlockField::ShowDescription
            ),
            flag_toggle(
                "Afficher l’auteur",
                show_author,
                locale,
                index,
                PageBlockField::ShowAuthor
            ),
            flag_toggle(
                "Afficher la date",
                show_date,
                locale,
                index,
                PageBlockField::ShowDate
            ),
        ]
        .spacing(8),
    ]
    .spacing(10)
    .into()
}

fn number_input(
    label: &str,
    value: Option<u32>,
    locale: Locale,
    block_index: usize,
    field: PageBlockField,
) -> Element<'static, Message> {
    let text_value = value.map(|value| value.to_string()).unwrap_or_default();

    labeled_input(label, text_value, move |value| {
        Message::PageBlockNumberChanged(locale, block_index, field, value)
    })
}

fn flag_toggle(
    label: &str,
    value: bool,
    locale: Locale,
    block_index: usize,
    field: PageBlockField,
) -> Element<'static, Message> {
    let marker = if value { "✓" } else { "—" };

    button(text(format!("{marker} {label}")))
        .on_press(Message::PageBlockFlagToggled(locale, block_index, field))
        .into()
}
