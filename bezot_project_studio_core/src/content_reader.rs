use std::fs;
use std::io;
use std::path::Path;

use serde_json::Value;

use crate::content_entry::ContentEntry;
use crate::errors::invalid_data;
use crate::localized_content_summary::LocalizedContentSummary;

pub fn read_content_entries(project_root: &Path) -> io::Result<Vec<ContentEntry>> {
    let content_dir = project_root.join("content");
    let mut entries = read_pages(&content_dir)?;
    entries.extend(read_posts(&content_dir)?);
    Ok(entries)
}

fn read_pages(content_dir: &Path) -> io::Result<Vec<ContentEntry>> {
    let pages_index = read_json(&content_dir.join("pages/index.json"))?;
    let page_ids = pages_index
        .get("pageIds")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_data("content/pages/index.json pageIds must be an array"))?;
    let mut entries = Vec::new();

    for page_id in page_ids {
        let page_id = page_id
            .as_str()
            .ok_or_else(|| invalid_data("page id must be a string"))?;
        let page_dir = content_dir.join("pages").join(page_id);
        let page_index = read_json(&page_dir.join("index.json"))?;
        let locales = read_page_locales(&page_dir, &page_index)?;

        entries.push(ContentEntry {
            kind: "page",
            id: page_id.to_string(),
            status: entry_status(&locales),
            locales,
        });
    }

    Ok(entries)
}

fn read_page_locales(
    page_dir: &Path,
    page_index: &Value,
) -> io::Result<Vec<LocalizedContentSummary>> {
    let locale_index = page_index
        .get("locales")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_data("page locales must be an object"))?;
    let mut locales = Vec::new();

    for (locale, metadata) in locale_index {
        let content = read_json(&page_dir.join(format!("{locale}.json")))?;
        locales.push(localized_summary(locale, metadata, &content)?);
    }

    locales.sort_by(|left, right| left.locale.cmp(&right.locale));
    Ok(locales)
}

fn read_posts(content_dir: &Path) -> io::Result<Vec<ContentEntry>> {
    let blog_index = read_json(&content_dir.join("blog/index.json"))?;
    let post_paths = blog_index
        .get("postPaths")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_data("content/blog/index.json postPaths must be an array"))?;
    let mut entries = Vec::new();

    for post_path in post_paths {
        let post_path = post_path
            .as_str()
            .ok_or_else(|| invalid_data("post path must be a string"))?;
        let post = read_json(&content_dir.join("blog").join(post_path))?;
        let id = required_string(&post, "id")?;
        let status = required_string(&post, "status")?.to_string();
        let locales_object = post
            .get("locales")
            .and_then(Value::as_object)
            .ok_or_else(|| invalid_data("post locales must be an object"))?;
        let mut locales = Vec::new();

        for (locale, content) in locales_object {
            locales.push(localized_summary(locale, &post, content)?);
        }

        locales.sort_by(|left, right| left.locale.cmp(&right.locale));
        entries.push(ContentEntry {
            kind: "post",
            id: id.to_string(),
            status,
            locales,
        });
    }

    Ok(entries)
}

fn localized_summary(
    locale: &str,
    metadata: &Value,
    content: &Value,
) -> io::Result<LocalizedContentSummary> {
    let seo = content
        .get("seo")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_data(format!("{locale} seo must be an object")))?;
    let title = seo
        .get("title")
        .and_then(Value::as_str)
        .unwrap_or("(untitled)")
        .to_string();
    let slug = content
        .get("slug")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let status = metadata
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("published")
        .to_string();
    let updated_at = metadata
        .get("updatedAt")
        .and_then(Value::as_str)
        .map(str::to_string);
    let block_count = content
        .get("blocks")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);

    Ok(LocalizedContentSummary {
        locale: locale.to_string(),
        status,
        title,
        slug,
        updated_at,
        block_count,
    })
}

fn entry_status(locales: &[LocalizedContentSummary]) -> String {
    if locales.iter().any(|locale| locale.status == "published") {
        "published".to_string()
    } else if let Some(locale) = locales.first() {
        locale.status.clone()
    } else {
        "unknown".to_string()
    }
}

fn read_json(path: &Path) -> io::Result<Value> {
    let source = fs::read_to_string(path)?;
    serde_json::from_str(&source).map_err(invalid_data)
}

fn required_string<'a>(value: &'a Value, field: &str) -> io::Result<&'a str> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| invalid_data(format!("{field} must be a string")))
}
