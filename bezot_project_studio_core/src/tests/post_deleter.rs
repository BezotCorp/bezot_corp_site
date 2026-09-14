use std::fs;

use common::{PostEditorState, read_json};
use serde_json::Value;

use crate::post_deleter::delete_post;
use crate::post_writer::save_post;
use crate::tests::support::temporary_project_root;

#[test]
fn deletes_a_draft_post_without_recording_a_gone_route() {
    let project_root = temporary_project_root("post_deleter_test");
    let blog_dir = project_root.join("content/blog");
    fs::create_dir_all(&blog_dir).unwrap();
    fs::write(
        blog_dir.join("index.json"),
        r#"{"status":"enabled","entryPageId":"blog","postPaths":[]}
"#,
    )
    .unwrap();
    fs::write(project_root.join("content/gone-routes.json"), "[]\n").unwrap();

    let editor = PostEditorState::default();
    save_post(&project_root, &editor).unwrap();

    let post_path = project_root
        .join("content/blog/posts")
        .join(&editor.date)
        .join(format!("{}.json", editor.id));
    assert!(post_path.exists());

    delete_post(&project_root, &editor.id).unwrap();

    assert!(!post_path.exists());
    let index = read_json(&blog_dir.join("index.json")).unwrap();
    assert!(
        index
            .get("postPaths")
            .and_then(Value::as_array)
            .unwrap()
            .is_empty()
    );
    let gone = read_json(&project_root.join("content/gone-routes.json")).unwrap();
    assert!(gone.as_array().unwrap().is_empty());

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn deleting_a_published_post_records_gone_routes() {
    let project_root = temporary_project_root("post_deleter_test");
    let blog_dir = project_root.join("content/blog");
    fs::create_dir_all(&blog_dir).unwrap();
    fs::write(
        blog_dir.join("index.json"),
        r#"{"status":"enabled","entryPageId":"blog","postPaths":[]}
"#,
    )
    .unwrap();
    fs::write(project_root.join("content/gone-routes.json"), "[]\n").unwrap();

    let mut editor = PostEditorState::default();
    editor.status = "published".to_string();
    save_post(&project_root, &editor).unwrap();

    delete_post(&project_root, &editor.id).unwrap();

    let gone = read_json(&project_root.join("content/gone-routes.json")).unwrap();
    let routes = gone.as_array().unwrap();
    assert!(
        routes
            .iter()
            .any(|value| value.as_str() == Some(format!("/fr-fr/{}", editor.fr.slug).as_str()))
    );
    assert!(
        routes
            .iter()
            .any(|value| value.as_str() == Some(format!("/en-us/{}", editor.en.slug).as_str()))
    );

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn deleting_an_unknown_post_fails() {
    let project_root = temporary_project_root("post_deleter_test");
    let blog_dir = project_root.join("content/blog");
    fs::create_dir_all(&blog_dir).unwrap();
    fs::write(
        blog_dir.join("index.json"),
        r#"{"status":"enabled","entryPageId":"blog","postPaths":[]}
"#,
    )
    .unwrap();

    assert!(delete_post(&project_root, "does-not-exist").is_err());

    fs::remove_dir_all(project_root).unwrap();
}
