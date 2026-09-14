use common::invalid_data;
use std::io;
use std::path::Path;

use common::{PostEditorState, new_uuid, today_yyyy_mm_dd};
use serde::Deserialize;
use serde_json::Value;

use crate::ollama_client::generate_json;
use crate::post_bridge::save_post_via_core;

/// Generates a bilingual blog post draft on the given topic with a local
/// Ollama model and saves it through bezot_project_studio_core, exactly as
/// the studio UI would. The id is prefixed "ai-editorial-" so it is
/// identifiable as AI-authored content and picked up by `audit`.
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
        "Tu es rédacteur SEO pour Bezot Corp, un studio logiciel. \
Rédige un brouillon d'article de blog bilingue (français et anglais) sur ce sujet : \"{topic}\".\n\
\n\
Réponds STRICTEMENT avec un objet JSON de cette forme exacte, sans aucun texte avant ou après :\n\
{{\n\
  \"fr\": {{\"title\": \"...\", \"slug\": \"blog/...\", \"description\": \"...\", \"paragraph\": \"...\"}},\n\
  \"en\": {{\"title\": \"...\", \"slug\": \"blog/...\", \"description\": \"...\", \"paragraph\": \"...\"}}\n\
}}\n\
\n\
Contraintes :\n\
- \"paragraph\" : au moins 150 mots, contenu utile et concret, pas de remplissage.\n\
- \"description\" : entre 80 et 180 caractères, résume l'article pour un extrait de recherche.\n\
- \"slug\" : minuscules, tirets, préfixé par \"blog/\", cohérent entre fr et en pour le même sujet.\n\
- \"title\" : clair, spécifique, sans emoji ni ponctuation excessive."
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

    editor.fr.title = unescape_html_entities(&parsed.fr.title);
    editor.fr.slug = parsed.fr.slug;
    editor.fr.description = unescape_html_entities(&parsed.fr.description);
    editor.fr.paragraph = unescape_html_entities(&parsed.fr.paragraph);
    editor.en.title = unescape_html_entities(&parsed.en.title);
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
