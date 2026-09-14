use iced::{
    Element,
    widget::{button, column, row, text},
};

use crate::message::Message;
use crate::studio_state::StudioState;
use crate::widgets::panel;

pub(crate) fn real_preview_panel(state: &StudioState) -> Element<'_, Message> {
    let mut content = column![
        text("Aperçu réel (identique au site)").size(16),
        text(
            "Construit une copie isolée du site avec ce brouillon marqué publié, \
             puis lance le vrai pipeline de production dessus — jamais le contenu \
             réel. Prend le temps d’un build complet (installation, build, \
             prérendu, vérifications)."
        )
        .size(12),
    ]
    .spacing(8);

    let start_label = if state.preview_starting {
        "Construction en cours…"
    } else if state.preview.is_some() {
        "Relancer l’aperçu"
    } else {
        "Lancer l’aperçu réel"
    };

    content = content.push(
        row![
            button(start_label)
                .on_press_maybe((!state.preview_starting).then_some(Message::StartPreview)),
            button("Arrêter l’aperçu")
                .on_press_maybe(state.preview.as_ref().map(|_| Message::StopPreview)),
        ]
        .spacing(8),
    );

    if let Some(session) = &state.preview {
        content = content.push(
            row![
                button("Ouvrir FR").on_press(Message::OpenPreviewUrl(session.url_fr.clone())),
                button("Ouvrir EN").on_press(Message::OpenPreviewUrl(session.url_en.clone())),
            ]
            .spacing(8),
        );
    }

    panel(content)
}
