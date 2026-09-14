use std::fs;

use common::{PostEditorState, read_json};
use serde_json::Value;

use crate::post_writer::save_post;
use crate::tests::support::temporary_project_root;

#[test]
fn saves_post_and_registers_it_in_blog_index() {
    let project_root = temporary_project_root("post_writer_test");
    let blog_dir = project_root.join("content/blog");
    fs::create_dir_all(&blog_dir).unwrap();
    fs::write(
        blog_dir.join("index.json"),
        r#"{
  "status": "enabled",
  "entryPageId": "blog",
  "postPaths": []
}
"#,
    )
    .unwrap();

    let editor = PostEditorState::default();

    save_post(&project_root, &editor).unwrap();

    let post_path = project_root
        .join("content/blog/posts")
        .join(&editor.date)
        .join(format!("{}.json", editor.id));
    assert!(post_path.exists());

    let index = read_json(&blog_dir.join("index.json")).unwrap();
    let post_paths = index.get("postPaths").and_then(Value::as_array).unwrap();
    let expected_path = format!("posts/{}/{}.json", editor.date, editor.id);
    assert_eq!(
        post_paths.first().and_then(Value::as_str),
        Some(expected_path.as_str())
    );

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn saves_every_paragraph_as_its_own_block() {
    let project_root = temporary_project_root("post_writer_test");
    let blog_dir = project_root.join("content/blog");
    fs::create_dir_all(&blog_dir).unwrap();
    fs::write(
        blog_dir.join("index.json"),
        r#"{
  "status": "enabled",
  "entryPageId": "blog",
  "postPaths": []
}
"#,
    )
    .unwrap();

    let mut editor = PostEditorState::default();
    editor.fr.paragraphs = vec![
        "Premier paragraphe.".to_string(),
        "Deuxième paragraphe.".to_string(),
        "Troisième paragraphe.".to_string(),
    ];

    save_post(&project_root, &editor).unwrap();

    let post_path = project_root
        .join("content/blog/posts")
        .join(&editor.date)
        .join(format!("{}.json", editor.id));
    let document = read_json(&post_path).unwrap();
    let fr_blocks = document
        .get("locales")
        .and_then(|locales| locales.get("fr-fr"))
        .and_then(|locale| locale.get("blocks"))
        .and_then(Value::as_array)
        .unwrap();
    let paragraph_texts = fr_blocks
        .iter()
        .filter(|block| block.get("type").and_then(Value::as_str) == Some("paragraph"))
        .filter_map(|block| block.get("props")?.get("text")?.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        paragraph_texts,
        vec![
            "Premier paragraphe.",
            "Deuxième paragraphe.",
            "Troisième paragraphe."
        ]
    );

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn rejects_a_post_with_no_paragraphs() {
    let project_root = temporary_project_root("post_writer_test");
    let blog_dir = project_root.join("content/blog");
    fs::create_dir_all(&blog_dir).unwrap();
    fs::write(
        blog_dir.join("index.json"),
        r#"{
  "status": "enabled",
  "entryPageId": "blog",
  "postPaths": []
}
"#,
    )
    .unwrap();

    let mut editor = PostEditorState::default();
    editor.fr.paragraphs.clear();

    assert!(save_post(&project_root, &editor).is_err());

    fs::remove_dir_all(project_root).unwrap();
}
