use serde_json::json;

use crate::draft_generator::{editor_from_value, unescape_html_entities};

#[test]
fn builds_an_editor_state_from_a_well_formed_model_reply() {
    let value = json!({
        "fr": {
            "title": "Titre FR",
            "slug": "blog/titre-fr",
            "description": "Description FR",
            "paragraphs": ["Paragraphe FR un", "Paragraphe FR deux"]
        },
        "en": {
            "title": "Title EN",
            "slug": "blog/title-en",
            "description": "Description EN",
            "paragraphs": ["Paragraph EN"]
        }
    });

    let editor = editor_from_value(value).unwrap();

    assert!(editor.id.starts_with("ai-editorial-"));
    assert_eq!(editor.status, "draft");
    assert_eq!(editor.author, "Bezot Corp AI");
    assert_eq!(editor.fr.title, "Édito IA : Titre FR");
    assert_eq!(
        editor.fr.subtitle,
        "Texte généré par IA, relu et publié par Bezot Corp."
    );
    assert_eq!(editor.fr.slug, "blog/titre-fr");
    assert_eq!(
        editor.fr.paragraphs,
        vec![
            "Paragraphe FR un".to_string(),
            "Paragraphe FR deux".to_string()
        ]
    );
    assert_eq!(editor.en.title, "AI Editorial: Title EN");
    assert_eq!(
        editor.en.subtitle,
        "AI-generated, reviewed, and published by Bezot Corp."
    );
    assert_eq!(editor.en.paragraphs, vec!["Paragraph EN".to_string()]);
}

#[test]
fn unescapes_html_entities_left_by_the_model() {
    assert_eq!(
        unescape_html_entities("L&#39;un des outils &amp; frameworks"),
        "L'un des outils & frameworks"
    );
    assert_eq!(
        unescape_html_entities("Rien à changer ici."),
        "Rien à changer ici."
    );
}

#[test]
fn draft_content_is_unescaped() {
    let value = json!({
        "fr": {
            "title": "Titre FR",
            "slug": "blog/titre-fr",
            "description": "Description FR",
            "paragraphs": ["L&#39;article parle de Rust &amp; WebAssembly."]
        },
        "en": {
            "title": "Title EN",
            "slug": "blog/title-en",
            "description": "Description EN",
            "paragraphs": ["Paragraph EN"]
        }
    });

    let editor = editor_from_value(value).unwrap();

    assert_eq!(
        editor.fr.paragraphs,
        vec!["L'article parle de Rust & WebAssembly.".to_string()]
    );
}

#[test]
fn rejects_a_reply_missing_a_locale() {
    let value = json!({
        "fr": {
            "title": "Titre FR",
            "slug": "blog/titre-fr",
            "description": "Description FR",
            "paragraphs": ["Paragraphe FR"]
        }
    });

    assert!(editor_from_value(value).is_err());
}
