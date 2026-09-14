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
        subtitle: hero_field(locale, "subtitle").unwrap_or_default(),
        slug: string_field(locale, "slug")?.to_string(),
        description: seo
            .get("description")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        paragraphs: all_paragraphs(locale),
        affiliate_title: affiliate_field(locale, "title").unwrap_or_default(),
        affiliate_text: affiliate_field(locale, "text").unwrap_or_default(),
        affiliate_url: affiliate_field(locale, "url").unwrap_or_default(),
        affiliate_label: affiliate_field(locale, "label").unwrap_or_default(),
        affiliate_disclosure: affiliate_field(locale, "disclosure").unwrap_or_default(),
    })
}

fn all_paragraphs(locale: &Value) -> Vec<String> {
    let Some(blocks) = locale.get("blocks").and_then(Value::as_array) else {
        return Vec::new();
    };

    blocks
        .iter()
        .filter(|block| block.get("type").and_then(Value::as_str) == Some("paragraph"))
        .filter_map(|block| block.get("props")?.get("text")?.as_str())
        .map(str::to_string)
        .collect()
}

fn hero_field(locale: &Value, field: &str) -> Option<String> {
    locale
        .get("blocks")?
        .as_array()?
        .iter()
        .find(|block| block.get("type").and_then(Value::as_str) == Some("hero"))?
        .get("props")?
        .get(field)?
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
