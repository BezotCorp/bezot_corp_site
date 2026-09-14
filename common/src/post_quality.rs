use serde::Serialize;

use crate::post_editor_state::PostEditorState;
use crate::post_locale_editor::PostLocaleEditor;

#[derive(Debug, Clone, Serialize)]
pub struct PostQualityReport {
    pub score: u8,
    pub reading_minutes: usize,
    pub french_words: usize,
    pub english_words: usize,
    pub checks: Vec<PostQualityCheck>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostQualityCheck {
    pub label: &'static str,
    pub passed: bool,
}

impl PostQualityReport {
    pub fn analyze(editor: &PostEditorState) -> Self {
        let french_words = word_count(&editor.fr.paragraphs.join(" "));
        let english_words = word_count(&editor.en.paragraphs.join(" "));
        let total_words = french_words + english_words;
        let reading_minutes = (total_words / 220).max(1);

        let checks = vec![
            PostQualityCheck::new("Titre FR", !editor.fr.title.trim().is_empty()),
            PostQualityCheck::new("Titre EN", !editor.en.title.trim().is_empty()),
            PostQualityCheck::new("Slug FR/EN", has_slug(&editor.fr) && has_slug(&editor.en)),
            PostQualityCheck::new(
                "Descriptions SEO",
                seo_description_ready(&editor.fr) && seo_description_ready(&editor.en),
            ),
            PostQualityCheck::new(
                "Contenu éditorial",
                french_words >= 120 && english_words >= 120,
            ),
            PostQualityCheck::new("Auteur", !editor.author.trim().is_empty()),
            PostQualityCheck::new("Date", editor.date.len() == 10),
            PostQualityCheck::new(
                "Monétisation",
                has_affiliate(&editor.fr) || has_affiliate(&editor.en),
            ),
        ];
        let score = ((checks.iter().filter(|check| check.passed).count() * 100) / checks.len())
            .try_into()
            .unwrap_or(0);
        let recommendations = recommendations(editor, french_words, english_words);

        Self {
            score,
            reading_minutes,
            french_words,
            english_words,
            checks,
            recommendations,
        }
    }
}

impl PostQualityCheck {
    fn new(label: &'static str, passed: bool) -> Self {
        Self { label, passed }
    }
}

fn recommendations(
    editor: &PostEditorState,
    french_words: usize,
    english_words: usize,
) -> Vec<String> {
    let mut recommendations = Vec::new();

    if editor.fr.description.chars().count() < 80 {
        recommendations.push("Allonger la description SEO française.".to_string());
    }

    if editor.en.description.chars().count() < 80 {
        recommendations.push("Allonger la description SEO anglaise.".to_string());
    }

    if french_words < 120 {
        recommendations.push("Ajouter du contenu français avant publication.".to_string());
    }

    if english_words < 120 {
        recommendations.push("Ajouter du contenu anglais avant publication.".to_string());
    }

    if !has_affiliate(&editor.fr) && !has_affiliate(&editor.en) {
        recommendations
            .push("Ajouter un encart affilié ou sponsorisé si l’article s’y prête.".to_string());
    }

    if recommendations.is_empty() {
        recommendations.push("Article prêt pour une relecture finale.".to_string());
    }

    recommendations
}

fn seo_description_ready(editor: &PostLocaleEditor) -> bool {
    let length = editor.description.chars().count();
    (80..=180).contains(&length)
}

fn has_slug(editor: &PostLocaleEditor) -> bool {
    !editor.slug.trim().is_empty()
}

fn has_affiliate(editor: &PostLocaleEditor) -> bool {
    !editor.affiliate_url.trim().is_empty()
}

fn word_count(value: &str) -> usize {
    value.split_whitespace().count()
}
