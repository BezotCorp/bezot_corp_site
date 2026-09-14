use std::io;
use std::path::Path;

use common::{
    PageBlock, PageEditorState, PageLocaleEditor, invalid_data, invalid_input, read_json,
};
use serde_json::Value;

pub fn load_page_editor(project_root: &Path, page_id: &str) -> io::Result<PageEditorState> {
    let content_dir = project_root.join("content");
    let pages_index = read_json(&content_dir.join("pages/index.json"))?;
    let page_ids = pages_index
        .get("pageIds")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_data("content/pages/index.json pageIds must be an array"))?;

    let known = page_ids.iter().any(|id| id.as_str() == Some(page_id));
    if !known {
        return Err(invalid_input(format!("could not find page {page_id}")));
    }

    let page_dir = content_dir.join("pages").join(page_id);
    let page_index = read_json(&page_dir.join("index.json"))?;
    let locales = page_index
        .get("locales")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_data("page locales must be an object"))?;
    let fr_index = locales
        .get("fr-fr")
        .ok_or_else(|| invalid_data("page fr-fr locale is missing"))?;
    let en_index = locales
        .get("en-us")
        .ok_or_else(|| invalid_data("page en-us locale is missing"))?;

    Ok(PageEditorState {
        id: page_id.to_string(),
        fr: locale_editor_from_files(fr_index, &read_json(&page_dir.join("fr-fr.json"))?)?,
        en: locale_editor_from_files(en_index, &read_json(&page_dir.join("en-us.json"))?)?,
    })
}

fn locale_editor_from_files(index_value: &Value, content: &Value) -> io::Result<PageLocaleEditor> {
    let seo = content
        .get("seo")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_data("page locale seo must be an object"))?;
    let blocks_value = content
        .get("blocks")
        .cloned()
        .ok_or_else(|| invalid_data("page locale blocks must be an array"))?;
    let blocks: Vec<PageBlock> = serde_json::from_value(blocks_value).map_err(invalid_data)?;

    Ok(PageLocaleEditor {
        status: string_field(index_value, "status")?.to_string(),
        updated_at: string_field(index_value, "updatedAt")?.to_string(),
        slug: string_field(content, "slug")?.to_string(),
        title: optional_string(seo, "title"),
        description: optional_string(seo, "description"),
        robots: optional_string(seo, "robots"),
        og_title: optional_string(seo, "ogTitle"),
        og_description: optional_string(seo, "ogDescription"),
        og_image: optional_string(seo, "ogImage"),
        blocks,
    })
}

fn optional_string(object: &serde_json::Map<String, Value>, field: &str) -> String {
    object
        .get(field)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

fn string_field<'a>(value: &'a Value, field: &str) -> io::Result<&'a str> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| invalid_data(format!("{field} must be a string")))
}
