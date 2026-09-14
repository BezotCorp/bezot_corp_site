use iced::Element;
use iced::widget::{button, column, row, text};

use crate::media_task::MediaTask;
use crate::message::Message;
use crate::studio_state::StudioState;
use crate::widgets::{panel, section_title};

pub(crate) fn media_page_view(state: &StudioState) -> Element<'_, Message> {
    column![
        section_title("Médias"),
        actions_panel(state),
        assets_panel(state),
    ]
    .spacing(18)
    .into()
}

fn actions_panel(state: &StudioState) -> Element<'_, Message> {
    let busy = state.media_task.is_busy();
    let upload_label = if state.media_task == MediaTask::Uploading {
        "Envoi en cours…"
    } else {
        "Uploader un fichier"
    };
    let refresh_label = if state.media_task == MediaTask::LoadingList {
        "Chargement…"
    } else {
        "Actualiser la liste"
    };

    panel(
        column![
            row![
                button(upload_label).on_press_maybe((!busy).then_some(Message::PickAndUploadMedia)),
                button(refresh_label).on_press_maybe((!busy).then_some(Message::LoadMedia)),
            ]
            .spacing(8),
            text("Formats acceptés : PNG, JPG, GIF, WEBP, SVG. Le chemin public copié peut être collé directement dans un champ « Image de partage » ou « Image OG ».")
                .size(12),
        ]
        .spacing(8),
    )
}

fn assets_panel(state: &StudioState) -> Element<'_, Message> {
    let mut list =
        column![text(format!("{} fichier(s)", state.media_assets.len())).size(16)].spacing(8);

    if state.media_assets.is_empty() {
        list = list.push(text("Aucun média téléversé pour le moment.").size(13));
        return panel(list);
    }

    for asset in &state.media_assets {
        let size_label = format_size(asset.size_bytes);
        let label = format!("{} — {}", asset.name, asset.public_path);
        list = list.push(
            row![
                text(label).size(13).width(iced::Fill),
                text(size_label).size(11),
                button("Copier le chemin")
                    .on_press(Message::CopyMediaPath(asset.public_path.clone())),
            ]
            .spacing(10),
        );
    }

    panel(list)
}

fn format_size(size_bytes: u64) -> String {
    if size_bytes >= 1_000_000 {
        format!("{:.1} Mo", size_bytes as f64 / 1_000_000.0)
    } else if size_bytes >= 1_000 {
        format!("{:.1} Ko", size_bytes as f64 / 1_000.0)
    } else {
        format!("{size_bytes} o")
    }
}
