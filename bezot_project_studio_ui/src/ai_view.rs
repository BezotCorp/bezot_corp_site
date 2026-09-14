use iced::{
    Element,
    widget::{button, column, row, text},
};

use crate::ai_task::AiTask;
use crate::editorial_ai_client::{OllamaModel, PostReview};
use crate::message::Message;
use crate::studio_state::StudioState;
use crate::task_hint::TaskHint;
use crate::vram_fit::VramFit;
use crate::widgets::{labeled_input, panel, section_title, status_chip};

pub(crate) fn ai_page_view(state: &StudioState) -> Element<'_, Message> {
    column![
        section_title("IA locale (Ollama)"),
        settings_panel(state),
        model_catalog_panel(
            "Modèle pour générer un brouillon",
            &state.ai_models,
            &state.ai_draft_model,
            state.ai_vram_gb(),
            TaskHint::Prose,
            Message::AiDraftModelChanged,
        ),
        model_catalog_panel(
            "Modèle pour l’audit éditorial",
            &state.ai_models,
            &state.ai_review_model,
            state.ai_vram_gb(),
            TaskHint::Analysis,
            Message::AiReviewModelChanged,
        ),
        actions_panel(state),
        reviews_panel(&state.ai_reviews),
    ]
    .spacing(18)
    .into()
}

fn settings_panel(state: &StudioState) -> Element<'_, Message> {
    let loading_models = state.ai_task == AiTask::LoadingModels;
    let load_models_label = if loading_models {
        "Chargement…"
    } else if state.ai_models.is_empty() {
        "Charger les modèles"
    } else {
        "Actualiser la liste"
    };

    panel(
        column![
            row![
                labeled_input(
                    "VRAM disponible (Go)",
                    &state.ai_vram_gb,
                    Message::AiVramChanged
                ),
                labeled_input(
                    "Sujet du brouillon",
                    &state.ai_topic,
                    Message::AiTopicChanged
                ),
            ]
            .spacing(10),
            row![
                button(load_models_label)
                    .on_press_maybe((!state.ai_task.is_busy()).then_some(Message::LoadModels)),
                text("Renseigne ta VRAM pour voir quels modèles tiennent sans swap.").size(12),
            ]
            .spacing(10),
        ]
        .spacing(8),
    )
}
fn model_catalog_panel<'a>(
    title: &'a str,
    models: &'a [OllamaModel],
    selected_name: &'a str,
    vram_gb: Option<f64>,
    task_hint: TaskHint,
    on_select: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    let mut list = column![text(title).size(16)].spacing(8);

    if models.is_empty() {
        list = list.push(
            text(
                "Aucun modèle chargé. Clique sur « Charger les modèles » ci-dessus, ou vérifie qu’Ollama tourne (`ollama serve`).",
            )
            .size(12),
        );
        return panel(list);
    }

    for model in models {
        let is_selected = model.name == selected_name;
        let fit = VramFit::assess(model.size_gb(), vram_gb);
        let profile = if model.is_code_specialized() {
            "code"
        } else {
            "généraliste"
        };
        let caution = if task_hint == TaskHint::Prose && model.is_code_specialized() {
            " · ⚠ orienté code, moins adapté à la rédaction"
        } else {
            ""
        };

        let label = format!(
            "{} · {} · {profile} · {}{caution}",
            model.name,
            model.parameter_size,
            fit.label()
        );
        let select_label = if is_selected {
            "✓ sélectionné"
        } else {
            "Choisir"
        };
        let model_name = model.name.clone();

        list = list.push(
            row![
                text(label).size(12).width(iced::Fill),
                button(select_label).on_press_maybe((!is_selected).then(|| on_select(model_name))),
            ]
            .spacing(10),
        );
    }

    panel(list)
}

fn actions_panel(state: &StudioState) -> Element<'_, Message> {
    let busy = state.ai_task.is_busy();
    let draft_ready = !state.ai_draft_model.trim().is_empty() && !state.ai_topic.trim().is_empty();
    let review_ready = !state.ai_review_model.trim().is_empty();

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
                button(draft_label)
                    .on_press_maybe((!busy && draft_ready).then_some(Message::GenerateDraft)),
                button(review_label)
                    .on_press_maybe((!busy && review_ready).then_some(Message::RunReview)),
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
