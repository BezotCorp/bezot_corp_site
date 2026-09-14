use common::invalid_data;
use std::{io, path::Path};

use common::{PostEditorState, new_uuid, today_yyyy_mm_dd};
use serde::Deserialize;
use serde_json::Value;

use crate::ollama_client::generate_json;
use crate::post_bridge::save_post_via_core;

const FR_TITLE_PREFIX: &str = "Édito IA : ";
const FR_DISCLOSURE: &str = "Texte généré par IA, relu et publié par Bezot Corp.";
const EN_TITLE_PREFIX: &str = "AI Editorial: ";
const EN_DISCLOSURE: &str = "AI-generated, reviewed, and published by Bezot Corp.";

/// Generates an "Édito IA" piece on the given topic with a local Ollama
/// model and saves it through bezot_project_studio_core, exactly as the
/// studio UI would. "Édito IA" is Bezot Corp's transparent AI-editorial
/// column — content openly labeled as AI-written and human-reviewed, not a
/// ghostwritten post passed off as a regular human-authored one. The title
/// prefix and disclosure subtitle are applied here in code (not left to the
/// model) so every generated piece carries them consistently, and the id is
/// prefixed "ai-editorial-" so `audit` picks it up.
pub fn generate_draft(
    project_root: &Path,
    model: &str,
    topic: &str,
) -> io::Result<PostEditorState> {
    let prompt = build_prompt(topic);
    let value = generate_json(model, &prompt)?;
    let editor = editor_from_value(value)?;

    save_post_via_core(project_root, &editor)?;

    Ok(editor)
}

fn build_prompt(topic: &str) -> String {
    format!(
        "Tu rédiges pour « Édito IA », la rubrique éditoriale transparente de Bezot Corp (un studio logiciel) : l'IA écrit, Bezot Corp relit et publie ouvertement sous ce label — ce n'est pas un article générique fait passer pour un texte humain.\n\
Rédige un texte d'opinion/analyse bilingue (français et anglais) sur ce sujet : \"{topic}\". Adopte un ton de recul et d'analyse, à la première personne du pluriel (« nous », « chez Bezot Corp »), pas un article SEO générique de type tutoriel.\n\
\n\
Réponds STRICTEMENT avec un objet JSON de cette forme exacte, sans aucun texte avant ou après :\n\
{{\n\
  \"fr\": {{\"title\": \"...\", \"slug\": \"blog/...\", \"description\": \"...\", \"paragraph\": \"...\"}},\n\
  \"en\": {{\"title\": \"...\", \"slug\": \"blog/...\", \"description\": \"...\", \"paragraph\": \"...\"}}\n\
}}\n\
\n\
Contraintes :\n\
- \"title\" : le sujet de l'édito SANS préfixe (le préfixe « Édito IA : » est ajouté automatiquement ensuite), clair et spécifique, sans emoji.\n\
- \"paragraph\" : au moins 150 mots, argumenté et concret, pas de remplissage.\n\
- \"description\" : entre 80 et 180 caractères, résume la thèse de l'édito.\n\
- \"slug\" : minuscules, tirets, préfixé par \"blog/\", cohérent entre fr et en pour le même sujet."
    )
}

#[derive(Debug, Deserialize)]
struct DraftResponse {
    fr: DraftLocale,
    en: DraftLocale,
}

#[derive(Debug, Deserialize)]
struct DraftLocale {
    title: String,
    slug: String,
    description: String,
    paragraph: String,
}

pub(crate) fn editor_from_value(value: Value) -> io::Result<PostEditorState> {
    let parsed: DraftResponse = serde_json::from_value(value).map_err(invalid_data)?;

    let mut editor = PostEditorState {
        id: format!("ai-editorial-{}", new_uuid()),
        date: today_yyyy_mm_dd(),
        status: "draft".to_string(),
        author: "Bezot Corp AI".to_string(),
        ..PostEditorState::default()
    };

    editor.fr.title = format!(
        "{FR_TITLE_PREFIX}{}",
        unescape_html_entities(&parsed.fr.title)
    );
    editor.fr.subtitle = FR_DISCLOSURE.to_string();
    editor.fr.slug = parsed.fr.slug;
    editor.fr.description = unescape_html_entities(&parsed.fr.description);
    editor.fr.paragraph = unescape_html_entities(&parsed.fr.paragraph);
    editor.en.title = format!(
        "{EN_TITLE_PREFIX}{}",
        unescape_html_entities(&parsed.en.title)
    );
    editor.en.subtitle = EN_DISCLOSURE.to_string();
    editor.en.slug = parsed.en.slug;
    editor.en.description = unescape_html_entities(&parsed.en.description);
    editor.en.paragraph = unescape_html_entities(&parsed.en.paragraph);

    Ok(editor)
}

/// Some models emit HTML entities (e.g. `&#39;`) instead of the literal
/// character even when asked for plain text, presumably picked up from
/// HTML-heavy training data. Undo the common ones so drafts don't ship with
/// garbled punctuation.
pub(crate) fn unescape_html_entities(text: &str) -> String {
    text.replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&quot;", "\"")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}
