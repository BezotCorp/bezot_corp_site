use std::fs;

use common::{PageEditorState, read_json};
use serde_json::Value;

use crate::page_deleter::delete_page;
use crate::page_writer::save_page;
use crate::tests::support::temporary_project_root;

fn write_content_index(project_root: &std::path::Path) {
    fs::create_dir_all(project_root.join("content")).unwrap();
    fs::write(
        project_root.join("content/index.json"),
        r#"{
  "sections": {
    "pages": {"status": "enabled", "indexPath": "pages/index.json", "homePageId": "home"},
    "blog": {"status": "enabled", "indexPath": "blog/index.json", "entryPageId": "blog"}
  }
}
"#,
    )
    .unwrap();
    fs::create_dir_all(project_root.join("content/pages")).unwrap();
    fs::write(
        project_root.join("content/pages/index.json"),
        r#"{"pageIds": []}
"#,
    )
    .unwrap();
    fs::write(project_root.join("content/gone-routes.json"), "[]\n").unwrap();
}

#[test]
fn deletes_a_draft_page() {
    let project_root = temporary_project_root("page_deleter_test");
    write_content_index(&project_root);

    let mut editor = PageEditorState::default();
    editor.id = "about".to_string();
    save_page(&project_root, &editor).unwrap();

    let page_dir = project_root.join("content/pages/about");
    assert!(page_dir.exists());

    delete_page(&project_root, "about").unwrap();

    assert!(!page_dir.exists());
    let index = read_json(&project_root.join("content/pages/index.json")).unwrap();
    assert!(
        index
            .get("pageIds")
            .and_then(Value::as_array)
            .unwrap()
            .is_empty()
    );

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn refuses_to_delete_the_home_page() {
    let project_root = temporary_project_root("page_deleter_test");
    write_content_index(&project_root);

    let mut editor = PageEditorState::default();
    editor.id = "home".to_string();
    save_page(&project_root, &editor).unwrap();

    assert!(delete_page(&project_root, "home").is_err());
    assert!(project_root.join("content/pages/home").exists());

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn refuses_to_delete_the_blog_entry_page() {
    let project_root = temporary_project_root("page_deleter_test");
    write_content_index(&project_root);

    let mut editor = PageEditorState::default();
    editor.id = "blog".to_string();
    save_page(&project_root, &editor).unwrap();

    assert!(delete_page(&project_root, "blog").is_err());

    fs::remove_dir_all(project_root).unwrap();
}
