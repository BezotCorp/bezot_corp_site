use iced::Element;
use iced::widget::{button, column, row, text};

use crate::ai_task::AiTask;
use crate::editorial_ai_client::PostReview;
use crate::message::Message;
use crate::studio_state::StudioState;
use crate::widgets::{labeled_input, panel, section_title, status_chip};

pub(crate) fn ai_page_view(state: &StudioState) -> Element<'_, Message> {
    column![
        section_title("IA locale (Ollama)"),
        settings_panel(state),
        actions_panel(state),
        reviews_panel(&state.ai_reviews),
    ]
    .spacing(18)
    .into()
}

fn settings_panel(state: &StudioState) -> Element<'_, Message> {
    panel(
        column![
            text("Le modèle doit déjà être disponible localement (`ollama list`).").size(12),
            row![
                labeled_input("Modèle Ollama", &state.ai_model, Message::AiModelChanged),
                labeled_input(
                    "Sujet du brouillon",
                    &state.ai_topic,
                    Message::AiTopicChanged
                ),
            ]
            .spacing(10),
        ]
        .spacing(8),
    )
}

fn actions_panel(state: &StudioState) -> Element<'_, Message> {
    let busy = state.ai_task.is_busy();
    let model_ready = !state.ai_model.trim().is_empty();
    let topic_ready = !state.ai_topic.trim().is_empty();

    let draft_label = match state.ai_task {
        AiTask::GeneratingDraft => "Génération en cours…",
        _ => "Générer un brouillon",
    };
    let review_label = match state.ai_task {
        AiTask::RunningReview => "Audit en cours…",
        _ => "Lancer l’audit IA",
    };

    panel(
        column![
            row![
                button(draft_label).on_press_maybe(
                    (!busy && model_ready && topic_ready).then_some(Message::GenerateDraft)
                ),
                button(review_label)
                    .on_press_maybe((!busy && model_ready).then_some(Message::RunReview)),
            ]
            .spacing(8),
            text("Un brouillon généré est enregistré et ouvert directement dans l’éditeur.")
                .size(12),
        ]
        .spacing(8),
    )
}

fn reviews_panel(reviews: &[PostReview]) -> Element<'_, Message> {
    let mut list = column![text("Dernier audit").size(16)].spacing(10);

    if reviews.is_empty() {
        list = list.push(text("Aucun audit lancé pour le moment.").size(13));
        return panel(list);
    }

    let mut current_id: Option<&str> = None;

    for review in reviews {
        if current_id != Some(review.id.as_str()) {
            list = list.push(text(review.id.clone()).size(15));
            current_id = Some(&review.id);
        }

        let update_marker = if review.needs_update {
            "à mettre à jour"
        } else {
            "à jour"
        };

        let mut entry = column![
            row![
                status_chip(review.locale.clone()),
                status_chip(format!("score {} / 100", review.seo_score)),
                status_chip(update_marker.to_string()),
            ]
            .spacing(8),
        ]
        .spacing(6);

        for suggestion in &review.suggestions {
            entry = entry.push(text(format!("• {suggestion}")).size(12));
        }

        list = list.push(entry);
    }

    panel(list)
}
