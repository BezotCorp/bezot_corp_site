use serde_json::json;

use crate::draft_generator::editor_from_value;

#[test]
fn builds_an_editor_state_from_a_well_formed_model_reply() {
    let value = json!({
        "fr": {
            "title": "Titre FR",
            "slug": "blog/titre-fr",
            "description": "Description FR",
            "paragraph": "Paragraphe FR"
        },
        "en": {
            "title": "Title EN",
            "slug": "blog/title-en",
            "description": "Description EN",
            "paragraph": "Paragraph EN"
        }
    });

    let editor = editor_from_value(value).unwrap();

    assert!(editor.id.starts_with("ai-editorial-"));
    assert_eq!(editor.status, "draft");
    assert_eq!(editor.author, "Bezot Corp AI");
    assert_eq!(editor.fr.title, "Titre FR");
    assert_eq!(editor.fr.slug, "blog/titre-fr");
    assert_eq!(editor.en.title, "Title EN");
    assert_eq!(editor.en.paragraph, "Paragraph EN");
}

#[test]
fn rejects_a_reply_missing_a_locale() {
    let value = json!({
        "fr": {
            "title": "Titre FR",
            "slug": "blog/titre-fr",
            "description": "Description FR",
            "paragraph": "Paragraphe FR"
        }
    });

    assert!(editor_from_value(value).is_err());
}
