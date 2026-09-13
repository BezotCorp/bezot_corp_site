use iced::widget::{column, container, row, scrollable, text};
use iced::{Element, Fill};

use crate::content_entry::ContentEntry;
use crate::message::Message;
use crate::studio_state::StudioState;

pub(crate) fn view(state: &StudioState) -> Element<'_, Message> {
    let mut content = column![
        text("Bezot Project Studio").size(32),
        text(state.project_root.display().to_string()).size(14),
    ]
    .spacing(12);

    if let Some(error) = &state.error {
        content = content.push(text(format!("Error: {error}")).size(18));
    } else {
        content = content.push(text(format!("{} content entries", state.entries.len())));

        for entry in &state.entries {
            content = content.push(entry_view(entry));
        }
    }

    container(scrollable(content.padding(24).spacing(16)))
        .width(Fill)
        .height(Fill)
        .into()
}

fn entry_view(entry: &ContentEntry) -> Element<'_, Message> {
    let mut block = column![row![
        text(entry.kind.to_uppercase()).size(14),
        text(&entry.id).size(22),
        text(format!("[{}]", entry.status)).size(14),
    ]
    .spacing(12)]
    .spacing(6);

    for locale in &entry.locales {
        block = block.push(text(format!(
            "{} [{}] {} /{} blocks:{}",
            locale.locale,
            locale.status,
            locale.title,
            locale.slug.trim_start_matches('/'),
            locale.block_count
        )));
    }

    container(block).padding(12).width(Fill).into()
}
