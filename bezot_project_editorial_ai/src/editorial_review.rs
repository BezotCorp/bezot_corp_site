use std::io;
use std::path::Path;

use serde::Deserialize;
use serde_json::Value;

use crate::content_loader::load_content_entries;
use crate::ollama_client::generate_json;
use crate::post_bridge::load_post_via_core;

/// Reviews every published post's full content (fetched through
/// bezot_project_studio_core, not read from disk) with a local Ollama model
/// and prints structured, actionable suggestions per locale.
pub fn review_editorial_content(project_root: &Path, model: &str) -> io::Result<()> {
    let entries = load_content_entries(project_root)?;
    let published_posts = entries
        .iter()
        .filter(|entry| entry.kind == "post" && entry.status == "published");

    let mut reviewed = 0;

    for entry in published_posts {
        reviewed += 1;
        println!("=== {} ===", entry.id);

        let editor = match load_post_via_core(project_root, &entry.id) {
            Ok(editor) => editor,
            Err(error) => {
                eprintln!("  could not load: {error}");
                continue;
            }
        };

        review_locale(
            model,
            "fr-fr",
            &editor.fr.title,
            &editor.fr.description,
            &editor.fr.paragraph,
        );
        review_locale(
            model,
            "en-us",
            &editor.en.title,
            &editor.en.description,
            &editor.en.paragraph,
        );
    }

    if reviewed == 0 {
        println!("no published posts to review");
    }

    Ok(())
}

fn review_locale(model: &str, locale: &str, title: &str, description: &str, paragraph: &str) {
    let prompt = build_review_prompt(title, description, paragraph);

    match generate_json(model, &prompt).and_then(parse_review) {
        Ok(review) => print_review(locale, &review),
        Err(error) => eprintln!("  [{locale}] review failed: {error}"),
    }
}

fn build_review_prompt(title: &str, description: &str, paragraph: &str) -> String {
    format!(
        "Tu es un expert SEO et éditorial. Analyse cet article de blog :\n\
Titre : {title}\n\
Description SEO : {description}\n\
Contenu : {paragraph}\n\
\n\
Réponds STRICTEMENT avec un objet JSON de cette forme exacte, sans aucun texte avant ou après :\n\
{{\n\
  \"seo_score\": <entier 0-100>,\n\
  \"needs_update\": <true ou false>,\n\
  \"suggestions\": [\"<suggestion concrète 1>\", \"<suggestion concrète 2>\"]\n\
}}\n\
\n\
\"needs_update\" doit être true si le contenu semble daté, générique, ou incomplet. \
Les suggestions doivent être concrètes et actionnables, pas des généralités."
    )
}

#[derive(Debug, Deserialize)]
pub(crate) struct EditorialReview {
    pub(crate) seo_score: u8,
    pub(crate) needs_update: bool,
    pub(crate) suggestions: Vec<String>,
}

pub(crate) fn parse_review(value: Value) -> io::Result<EditorialReview> {
    serde_json::from_value(value)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))
}

fn print_review(locale: &str, review: &EditorialReview) {
    let update_marker = if review.needs_update { "⚠" } else { "✓" };
    println!(
        "  [{locale}] score {} / 100 {update_marker}",
        review.seo_score
    );

    for suggestion in &review.suggestions {
        println!("    - {suggestion}");
    }
}
