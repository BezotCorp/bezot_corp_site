use std::fs;

use common::{PageEditorState, read_json};
use serde_json::Value;

use crate::page_writer::save_page;
use crate::tests::support::temporary_project_root;

#[test]
fn saves_page_and_registers_it_in_pages_index() {
    let project_root = temporary_project_root("page_writer_test");
    fs::create_dir_all(project_root.join("content/pages")).unwrap();
    fs::write(
        project_root.join("content/pages/index.json"),
        r#"{"pageIds": []}
"#,
    )
    .unwrap();

    let mut editor = PageEditorState::default();
    editor.id = "about".to_string();
    editor.fr.slug = "a-propos".to_string();
    editor.en.slug = "about".to_string();

    save_page(&project_root, &editor).unwrap();

    let page_dir = project_root.join("content/pages/about");
    assert!(page_dir.join("index.json").exists());
    assert!(page_dir.join("fr-fr.json").exists());
    assert!(page_dir.join("en-us.json").exists());

    let index = read_json(&project_root.join("content/pages/index.json")).unwrap();
    let page_ids = index.get("pageIds").and_then(Value::as_array).unwrap();
    assert_eq!(page_ids.first().and_then(Value::as_str), Some("about"));

    let fr_content = read_json(&page_dir.join("fr-fr.json")).unwrap();
    assert_eq!(
        fr_content.get("slug").and_then(Value::as_str),
        Some("a-propos")
    );

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn rejects_a_page_with_no_blocks() {
    let project_root = temporary_project_root("page_writer_test");
    fs::create_dir_all(project_root.join("content/pages")).unwrap();
    fs::write(
        project_root.join("content/pages/index.json"),
        r#"{"pageIds": []}
"#,
    )
    .unwrap();

    let mut editor = PageEditorState::default();
    editor.id = "empty-page".to_string();
    editor.fr.blocks.clear();

    assert!(save_page(&project_root, &editor).is_err());

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn rejects_an_invalid_page_id() {
    let project_root = temporary_project_root("page_writer_test");
    fs::create_dir_all(project_root.join("content/pages")).unwrap();
    fs::write(
        project_root.join("content/pages/index.json"),
        r#"{"pageIds": []}
"#,
    )
    .unwrap();

    let mut editor = PageEditorState::default();
    editor.id = "Invalid Id!".to_string();

    assert!(save_page(&project_root, &editor).is_err());

    fs::remove_dir_all(project_root).unwrap();
}
