use std::io;
use std::path::Path;

use common::{PostEditorState, PostLocaleEditor, invalid_data, invalid_input, read_json};
use serde_json::Value;

pub fn load_post_editor(project_root: &Path, post_id: &str) -> io::Result<PostEditorState> {
    let content_dir = project_root.join("content");
    let blog_dir = content_dir.join("blog");
    let blog_index = read_json(&blog_dir.join("index.json"))?;
    let post_paths = blog_index
        .get("postPaths")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_data("content/blog/index.json postPaths must be an array"))?;

    for post_path in post_paths {
        let post_path = post_path
            .as_str()
            .ok_or_else(|| invalid_data("post path must be a string"))?;
        let post = read_json(&blog_dir.join(post_path))?;

        if string_field(&post, "id")? == post_id {
            return editor_from_post(&post);
        }
    }

    Err(invalid_input(format!("could not find post {post_id}")))
}

fn editor_from_post(post: &Value) -> io::Result<PostEditorState> {
    let locales = post
        .get("locales")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_data("post locales must be an object"))?;
    let fr = locales
        .get("fr-fr")
        .ok_or_else(|| invalid_data("post fr-fr locale is missing"))?;
    let en = locales
        .get("en-us")
        .ok_or_else(|| invalid_data("post en-us locale is missing"))?;

    Ok(PostEditorState {
        id: string_field(post, "id")?.to_string(),
        date: string_field(post, "publishedAt")?.to_string(),
        status: string_field(post, "status")?.to_string(),
        author: string_field(post, "author")?.to_string(),
        fr: locale_editor_from_value(fr)?,
        en: locale_editor_from_value(en)?,
    })
}

fn locale_editor_from_value(locale: &Value) -> io::Result<PostLocaleEditor> {
    let seo = locale
        .get("seo")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_data("locale seo must be an object"))?;

    Ok(PostLocaleEditor {
        title: seo
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        slug: string_field(locale, "slug")?.to_string(),
        description: seo
            .get("description")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        paragraph: first_paragraph(locale).unwrap_or_default(),
        affiliate_title: affiliate_field(locale, "title").unwrap_or_default(),
        affiliate_text: affiliate_field(locale, "text").unwrap_or_default(),
        affiliate_url: affiliate_field(locale, "url").unwrap_or_default(),
        affiliate_label: affiliate_field(locale, "label").unwrap_or_default(),
        affiliate_disclosure: affiliate_field(locale, "disclosure").unwrap_or_default(),
    })
}

fn first_paragraph(locale: &Value) -> Option<String> {
    locale
        .get("blocks")?
        .as_array()?
        .iter()
        .find(|block| block.get("type").and_then(Value::as_str) == Some("paragraph"))?
        .get("props")?
        .get("text")?
        .as_str()
        .map(str::to_string)
}

fn affiliate_field(locale: &Value, field: &str) -> Option<String> {
    locale
        .get("blocks")?
        .as_array()?
        .iter()
        .find(|block| block.get("type").and_then(Value::as_str) == Some("affiliate_callout"))?
        .get("props")?
        .get(field)?
        .as_str()
        .map(str::to_string)
}

fn string_field<'a>(value: &'a Value, field: &str) -> io::Result<&'a str> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| invalid_data(format!("{field} must be a string")))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    #[test]
    fn loads_existing_post_into_editor_state() {
        let project_root = temporary_project_root();
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
        "description": "Description FR"
      },
      "blocks": [
        {
          "type": "paragraph",
          "props": {
            "text": "Paragraphe FR"
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
        assert_eq!(editor.fr.paragraph, "Paragraphe FR");
        assert_eq!(editor.en.title, "Sample post");
        assert_eq!(editor.en.paragraph, "English paragraph");

        fs::remove_dir_all(project_root).unwrap();
    }

    fn temporary_project_root() -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!(
            "bezot_project_studio_core_post_reader_test_{}_{}",
            std::process::id(),
            stamp
        ))
    }
}
