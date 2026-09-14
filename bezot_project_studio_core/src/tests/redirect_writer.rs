use std::fs;

use common::read_json;
use serde_json::Value;

use crate::redirect_writer::record_slug_redirect;
use crate::tests::support::temporary_project_root;

#[test]
fn records_a_redirect_when_the_slug_changes() {
    let project_root = temporary_project_root("redirect_writer_test");
    let content_dir = project_root.join("content");
    fs::create_dir_all(&content_dir).unwrap();
    fs::write(content_dir.join("redirects.json"), "[]\n").unwrap();

    record_slug_redirect(
        &project_root,
        "fr-fr",
        "blog/ancien-slug",
        "blog/nouveau-slug",
    )
    .unwrap();

    let redirects = read_json(&content_dir.join("redirects.json")).unwrap();
    let entries = redirects.as_array().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(
        entries[0].get("from").and_then(Value::as_str),
        Some("/fr-fr/blog/ancien-slug")
    );
    assert_eq!(
        entries[0].get("to").and_then(Value::as_str),
        Some("/fr-fr/blog/nouveau-slug")
    );
    assert_eq!(entries[0].get("status").and_then(Value::as_u64), Some(301));

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn collapses_a_second_slug_change_instead_of_chaining() {
    let project_root = temporary_project_root("redirect_writer_test");
    let content_dir = project_root.join("content");
    fs::create_dir_all(&content_dir).unwrap();
    fs::write(content_dir.join("redirects.json"), "[]\n").unwrap();

    // First edit: slug-a -> slug-b.
    record_slug_redirect(&project_root, "fr-fr", "blog/slug-a", "blog/slug-b").unwrap();
    // Second edit on the same post: slug-b -> slug-c.
    record_slug_redirect(&project_root, "fr-fr", "blog/slug-b", "blog/slug-c").unwrap();

    let redirects = read_json(&content_dir.join("redirects.json")).unwrap();
    let entries = redirects.as_array().unwrap();

    // Both historical slugs must redirect straight to the current one; no
    // entry should still point at the intermediate slug-b.
    assert_eq!(entries.len(), 2);
    assert!(
        entries
            .iter()
            .all(|entry| entry.get("to").and_then(Value::as_str) == Some("/fr-fr/blog/slug-c"))
    );
    let froms = entries
        .iter()
        .filter_map(|entry| entry.get("from").and_then(Value::as_str))
        .collect::<Vec<_>>();
    assert!(froms.contains(&"/fr-fr/blog/slug-a"));
    assert!(froms.contains(&"/fr-fr/blog/slug-b"));

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn drops_a_self_redirect_left_by_reverting_a_slug() {
    let project_root = temporary_project_root("redirect_writer_test");
    let content_dir = project_root.join("content");
    fs::create_dir_all(&content_dir).unwrap();
    fs::write(content_dir.join("redirects.json"), "[]\n").unwrap();

    // slug-a -> slug-b, then back to slug-a: slug-b was genuinely live for a
    // while, so it must still redirect to the current slug (slug-a). Only
    // the resulting slug-a -> slug-a self-redirect must be dropped.
    record_slug_redirect(&project_root, "fr-fr", "blog/slug-a", "blog/slug-b").unwrap();
    record_slug_redirect(&project_root, "fr-fr", "blog/slug-b", "blog/slug-a").unwrap();

    let redirects = read_json(&content_dir.join("redirects.json")).unwrap();
    let entries = redirects.as_array().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(
        entries[0].get("from").and_then(Value::as_str),
        Some("/fr-fr/blog/slug-b")
    );
    assert_eq!(
        entries[0].get("to").and_then(Value::as_str),
        Some("/fr-fr/blog/slug-a")
    );

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn does_nothing_when_the_slug_is_unchanged() {
    let project_root = temporary_project_root("redirect_writer_test");
    let content_dir = project_root.join("content");
    fs::create_dir_all(&content_dir).unwrap();
    fs::write(content_dir.join("redirects.json"), "[]\n").unwrap();

    record_slug_redirect(&project_root, "fr-fr", "blog/meme-slug", "blog/meme-slug").unwrap();

    let redirects = read_json(&content_dir.join("redirects.json")).unwrap();
    assert!(redirects.as_array().unwrap().is_empty());

    fs::remove_dir_all(project_root).unwrap();
}
