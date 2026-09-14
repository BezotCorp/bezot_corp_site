use serde_json::json;

use crate::editorial_review::parse_review;

#[test]
fn parses_a_well_formed_review() {
    let value = json!({
        "seo_score": 72,
        "needs_update": true,
        "suggestions": ["Allonger l'introduction.", "Ajouter un lien interne."]
    });

    let review = parse_review(value).unwrap();

    assert_eq!(review.seo_score, 72);
    assert!(review.needs_update);
    assert_eq!(review.suggestions.len(), 2);
}

#[test]
fn rejects_a_reply_missing_a_field() {
    let value = json!({
        "seo_score": 72,
        "needs_update": true
    });

    assert!(parse_review(value).is_err());
}
