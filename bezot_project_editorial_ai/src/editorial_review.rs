use std::io;
use std::path::Path;

use common::invalid_data;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::content_loader::load_content_entries;
use crate::ollama_client::generate_json;
use crate::post_bridge::load_post_via_core;

#[derive(Debug, Serialize)]
pub(crate) struct PostReview {
    pub(crate) id: String,
    pub(crate) locale: String,
    pub(crate) seo_score: u8,
    pub(crate) needs_update: bool,
    pub(crate) suggestions: Vec<String>,
}

/// Reviews every published post's full content (fetched through
/// bezot_project_studio_core, not read from disk) with a local Ollama model.
/// Prints a human-readable report by default, or a JSON array when `format`
/// is `"json"` (used by callers like the studio UI that need to parse it).
pub fn review_editorial_content(project_root: &Path, model: &str, format: &str) -> io::Result<()> {
    let reviews = collect_reviews(project_root, model)?;

    if format == "json" {
        println!(
            "{}",
            serde_json::to_string_pretty(&reviews).map_err(invalid_data)?
        );
    } else {
        print_text_reviews(&reviews);
    }

    Ok(())
}

fn collect_reviews(project_root: &Path, model: &str) -> io::Result<Vec<PostReview>> {
    let entries = load_content_entries(project_root)?;
    let published_posts = entries
        .iter()
        .filter(|entry| entry.kind == "post" && entry.status == "published");

    let mut reviews = Vec::new();

    for entry in published_posts {
        let editor = match load_post_via_core(project_root, &entry.id) {
            Ok(editor) => editor,
            Err(error) => {
                eprintln!("could not load {}: {error}", entry.id);
                continue;
            }
        };

        let locales = [
            (
                "fr-fr",
                &editor.fr.title,
                &editor.fr.description,
                &editor.fr.paragraph,
            ),
            (
                "en-us",
                &editor.en.title,
                &editor.en.description,
                &editor.en.paragraph,
            ),
        ];

        for (locale, title, description, paragraph) in locales {
            match review_locale(model, title, description, paragraph) {
                Ok(review) => reviews.push(PostReview {
                    id: entry.id.clone(),
                    locale: locale.to_string(),
                    seo_score: review.seo_score,
                    needs_update: review.needs_update,
                    suggestions: review.suggestions,
                }),
                Err(error) => eprintln!("[{}/{locale}] review failed: {error}", entry.id),
            }
        }
    }

    Ok(reviews)
}

fn review_locale(
    model: &str,
    title: &str,
    description: &str,
    paragraph: &str,
) -> io::Result<EditorialReview> {
    let prompt = build_review_prompt(title, description, paragraph);
    generate_json(model, &prompt).and_then(parse_review)
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
    serde_json::from_value(value).map_err(invalid_data)
}

fn print_text_reviews(reviews: &[PostReview]) {
    if reviews.is_empty() {
        println!("no published posts to review");
        return;
    }

    let mut current_id: Option<&str> = None;

    for review in reviews {
        if current_id != Some(review.id.as_str()) {
            println!("=== {} ===", review.id);
            current_id = Some(&review.id);
        }

        let update_marker = if review.needs_update { "⚠" } else { "✓" };
        println!(
            "  [{}] score {} / 100 {update_marker}",
            review.locale, review.seo_score
        );

        for suggestion in &review.suggestions {
            println!("    - {suggestion}");
        }
    }
}
