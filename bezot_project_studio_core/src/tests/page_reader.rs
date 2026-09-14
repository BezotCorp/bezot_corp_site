use std::fs;

use common::PageBlock;

use crate::page_reader::load_page_editor;
use crate::tests::support::temporary_project_root;

#[test]
fn loads_existing_page_into_editor_state() {
    let project_root = temporary_project_root("page_reader_test");
    let page_dir = project_root.join("content/pages/contact");
    fs::create_dir_all(&page_dir).unwrap();
    fs::write(
        project_root.join("content/pages/index.json"),
        r#"{"pageIds": ["contact"]}
"#,
    )
    .unwrap();
    fs::write(
        page_dir.join("index.json"),
        r#"{
  "locales": {
    "fr-fr": {"status": "published", "updatedAt": "2026-05-17"},
    "en-us": {"status": "published", "updatedAt": "2026-05-17"}
  }
}
"#,
    )
    .unwrap();
    fs::write(
        page_dir.join("fr-fr.json"),
        r#"{
  "slug": "contact",
  "seo": {
    "title": "Contact - Bezot Corp",
    "description": "Contactez Bezot Corp.",
    "robots": "index, follow",
    "ogTitle": "Contact - Bezot Corp",
    "ogDescription": "Une question ? Contactez-nous.",
    "ogImage": "/og/bezot-corp-default.png"
  },
  "blocks": [
    {"type": "hero", "props": {"title": "Contact", "subtitle": "Une question ?"}},
    {"type": "mail_link", "props": {"email": "contact@bezotcorp.com", "label": "Envoyer un email"}}
  ]
}
"#,
    )
    .unwrap();
    fs::write(
        page_dir.join("en-us.json"),
        r#"{
  "slug": "contact",
  "seo": {
    "title": "Contact - Bezot Corp",
    "description": "Contact Bezot Corp.",
    "robots": "index, follow",
    "ogTitle": "Contact - Bezot Corp",
    "ogDescription": "A question? Reach out.",
    "ogImage": "/og/bezot-corp-default.png"
  },
  "blocks": [
    {"type": "hero", "props": {"title": "Contact", "subtitle": "A question?"}},
    {"type": "mail_link", "props": {"email": "contact@bezotcorp.com", "label": "Send an email"}}
  ]
}
"#,
    )
    .unwrap();

    let editor = load_page_editor(&project_root, "contact").unwrap();

    assert_eq!(editor.id, "contact");
    assert_eq!(editor.fr.status, "published");
    assert_eq!(editor.fr.slug, "contact");
    assert_eq!(editor.fr.title, "Contact - Bezot Corp");
    assert_eq!(editor.fr.blocks.len(), 2);
    assert_eq!(
        editor.fr.blocks[0],
        PageBlock::Hero {
            title: "Contact".to_string(),
            subtitle: "Une question ?".to_string(),
        }
    );
    assert_eq!(editor.en.blocks.len(), 2);

    fs::remove_dir_all(project_root).unwrap();
}

#[test]
fn rejects_an_unknown_page_id() {
    let project_root = temporary_project_root("page_reader_test");
    fs::create_dir_all(project_root.join("content/pages")).unwrap();
    fs::write(
        project_root.join("content/pages/index.json"),
        r#"{"pageIds": []}
"#,
    )
    .unwrap();

    assert!(load_page_editor(&project_root, "missing-page").is_err());

    fs::remove_dir_all(project_root).unwrap();
}
