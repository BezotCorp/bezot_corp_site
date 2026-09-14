use std::fs;

use crate::post_reader::load_post_editor;
use crate::tests::support::temporary_project_root;

#[test]
fn loads_existing_post_into_editor_state() {
    let project_root = temporary_project_root("post_reader_test");
    let blog_dir = project_root.join("content/blog");
    let post_dir = blog_dir.join("posts/2026-09-14");
    fs::create_dir_all(&post_dir).unwrap();
    fs::write(
        blog_dir.join("index.json"),
        r#"{
  "status": "enabled",
  "entryPageId": "blog",
  "postPaths": ["posts/2026-09-14/sample-post.json"]
}
"#,
    )
    .unwrap();
    fs::write(
        post_dir.join("sample-post.json"),
        r#"{
  "id": "sample-post",
  "type": "editorial",
  "status": "draft",
  "author": "Bezot Corp",
  "publishedAt": "2026-09-14",
  "updatedAt": "2026-09-14",
  "locales": {
    "fr-fr": {
      "slug": "blog/article-exemple",
      "seo": {
        "title": "Article exemple",
        "description": "Description FR",
        "ogImage": "/og/article-exemple.png"
      },
      "blocks": [
        {
          "type": "paragraph",
          "props": {
            "text": "Paragraphe FR un"
          }
        },
        {
          "type": "paragraph",
          "props": {
            "text": "Paragraphe FR deux"
          }
        }
      ]
    },
    "en-us": {
      "slug": "blog/sample-post",
      "seo": {
        "title": "Sample post",
        "description": "Description EN"
      },
      "blocks": [
        {
          "type": "paragraph",
          "props": {
            "text": "English paragraph"
          }
        }
      ]
    }
  }
}
"#,
    )
    .unwrap();

    let editor = load_post_editor(&project_root, "sample-post").unwrap();

    assert_eq!(editor.id, "sample-post");
    assert_eq!(editor.fr.title, "Article exemple");
    assert_eq!(editor.fr.og_image, "/og/article-exemple.png");
    assert_eq!(
        editor.fr.paragraphs,
        vec![
            "Paragraphe FR un".to_string(),
            "Paragraphe FR deux".to_string()
        ]
    );
    assert_eq!(editor.en.title, "Sample post");
    assert_eq!(editor.en.paragraphs, vec!["English paragraph".to_string()]);

    fs::remove_dir_all(project_root).unwrap();
}
