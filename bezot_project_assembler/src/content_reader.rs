use std::{
    fs, io,
    path::{Path, PathBuf},
};

use serde_json::{Map, Value};

use crate::{
    asserts::assert_file,
    content_validator::validate_section,
    string_operations::{string_at, string_field},
};
use crate::{
    asserts::{
        assert_array_contains_string, assert_no_locale_index_fields, assert_non_empty_string,
        assert_object, assert_safe_relative_path, assert_status, assert_unique_strings,
    },
    content_validator::invalid_data_message,
};

#[derive(Debug, Clone)]
pub struct ContentModel {
    pub site: Value,
    pub redirects: Value,
    pub gone: Value,
    pub pages: Vec<Value>,
    pub posts: Vec<Value>,
}

impl ContentModel {
    pub(crate) fn read_content_model(project_root: &Path) -> io::Result<ContentModel> {
        let content_dir = project_root.join("content");

        let content_index = read_json(&content_dir.join("index.json"))?;

        let pages_section = get_required_object(&content_index, &["sections", "pages"])?;
        let blog_section = get_required_object(&content_index, &["sections", "blog"])?;

        validate_section(pages_section, "sections.pages")?;
        validate_section(blog_section, "sections.blog")?;

        let pages = read_pages(&content_dir, pages_section, blog_section)?;
        let posts = read_blog_posts(&content_dir, blog_section)?;

        Ok(Self {
            site: read_json(&content_dir.join("website-metadata.json"))?,
            redirects: read_json(&content_dir.join("redirects.json"))?,
            gone: read_json(&content_dir.join("gone-routes.json"))?,
            pages,
            posts,
        })
    }
}

fn read_pages(
    content_dir: &Path,
    pages_section: &Map<String, Value>,
    blog_section: &Map<String, Value>,
) -> io::Result<Vec<Value>> {
    if string_field(pages_section, "status")? != "enabled" {
        return Ok(Vec::new());
    }

    let pages_index_path = safe_content_path(
        content_dir,
        string_field(pages_section, "indexPath")?,
        "sections.pages.indexPath",
    )?;

    let pages_dir = pages_index_path
        .parent()
        .ok_or_else(|| invalid_data_message("pages index has no parent directory"))?;

    let pages_index = read_json(&pages_index_path)?;
    let page_ids = array_field(
        &pages_index,
        &["pageIds"],
        "content/pages/index.json pageIds",
    )?;

    assert_unique_strings(page_ids, "content/pages/index.json pageIds")?;

    let home_page_id = string_field(pages_section, "homePageId")?;
    let blog_entry_page_id = string_field(blog_section, "entryPageId")?;

    assert_array_contains_string(page_ids, home_page_id, "homePageId")?;
    assert_array_contains_string(page_ids, blog_entry_page_id, "blog entryPageId")?;

    let mut pages = Vec::new();

    for page_id_value in page_ids {
        let page_id = page_id_value
            .as_str()
            .ok_or_else(|| invalid_data_message("pageId must be a string"))?;

        if page_id.is_empty() {
            return Err(invalid_data_message("pageId must be a non-empty string"));
        }

        let page_dir = pages_dir.join(page_id);
        let page_index_path = page_dir.join("index.json");

        assert_file(&page_index_path)?;

        let page_index = read_json(&page_index_path)?;

        let locales_index = page_index
            .get("locales")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();

        let mut locales = Map::new();

        for (locale, locale_index_value) in locales_index {
            let locale_index = locale_index_value.as_object().ok_or_else(|| {
                invalid_data_message(format!(
                    "page \"{page_id}\" locale \"{locale}\" must be an object"
                ))
            })?;

            assert_status(
                locale_index.get("status"),
                &["published", "draft"],
                &format!("page \"{page_id}\" locale \"{locale}\""),
            )?;

            assert_non_empty_string(
                locale_index.get("updatedAt"),
                &format!("page \"{page_id}\" locale \"{locale}\" updatedAt"),
            )?;

            let locale_path = page_dir.join(format!("{locale}.json"));
            assert_file(&locale_path)?;

            let locale_content = read_json(&locale_path)?;

            assert_no_locale_index_fields(page_id, &locale, &locale_content)?;

            let mut merged = Map::new();

            for (key, value) in locale_index {
                merged.insert(key.clone(), value.clone());
            }

            let locale_content_object = locale_content.as_object().ok_or_else(|| {
                invalid_data_message(format!(
                    "page \"{page_id}\" locale \"{locale}\" content must be an object"
                ))
            })?;

            for (key, value) in locale_content_object {
                merged.insert(key.clone(), value.clone());
            }

            locales.insert(locale, Value::Object(merged));
        }

        let mut page = Map::new();
        page.insert("id".to_string(), Value::String(page_id.to_string()));
        page.insert("locales".to_string(), Value::Object(locales));

        pages.push(Value::Object(page));
    }

    Ok(pages)
}

fn read_blog_posts(
    content_dir: &Path,
    blog_section: &Map<String, Value>,
) -> io::Result<Vec<Value>> {
    if string_field(blog_section, "status")? != "enabled" {
        return Ok(Vec::new());
    }

    let blog_index_path = safe_content_path(
        content_dir,
        string_field(blog_section, "indexPath")?,
        "sections.blog.indexPath",
    )?;

    let blog_dir = blog_index_path
        .parent()
        .ok_or_else(|| invalid_data_message("blog index has no parent directory"))?;

    let blog_index = read_json(&blog_index_path)?;

    assert_status(
        blog_index.get("status"),
        &["enabled", "disabled"],
        "blog index",
    )?;

    if blog_index.get("status").and_then(Value::as_str) != Some("enabled") {
        return Ok(Vec::new());
    }

    let entry_page_id = string_at(
        &blog_index,
        &["entryPageId"],
        "content/blog/index.json entryPageId",
    )?;
    let section_entry_page_id = string_field(blog_section, "entryPageId")?;

    if entry_page_id != section_entry_page_id {
        return Err(invalid_data_message(
            "Blog entryPageId mismatch between content/index.json and content/blog/index.json",
        ));
    }

    let post_paths = array_field(
        &blog_index,
        &["postPaths"],
        "content/blog/index.json postPaths",
    )?;

    assert_unique_strings(post_paths, "content/blog/index.json postPaths")?;

    let mut posts = Vec::new();

    for post_path_value in post_paths {
        let post_path = post_path_value
            .as_str()
            .ok_or_else(|| invalid_data_message("blog post path must be a string"))?;

        assert_safe_relative_path(post_path, &format!("blog post path \"{post_path}\""))?;

        let post_file_path = blog_dir.join(post_path);
        assert_file(&post_file_path)?;

        let post = read_json(&post_file_path)?;

        assert_non_empty_string(post.get("id"), &format!("blog post \"{post_path}\" id"))?;

        let post_id = post.get("id").and_then(Value::as_str).unwrap_or(post_path);

        assert_status(
            post.get("status"),
            &["published", "draft", "archived"],
            &format!("blog post \"{post_id}\""),
        )?;

        assert_non_empty_string(
            post.get("publishedAt"),
            &format!("blog post \"{post_id}\" publishedAt"),
        )?;

        assert_object(
            post.get("locales"),
            &format!("blog post \"{post_id}\" locales"),
        )?;

        posts.push(post);
    }

    Ok(posts)
}

fn read_json(path: &Path) -> io::Result<Value> {
    assert_file(path)?;

    let text = fs::read_to_string(path)?;
    serde_json::from_str(&text).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

fn get_required_object<'a>(value: &'a Value, path: &[&str]) -> io::Result<&'a Map<String, Value>> {
    let mut current = value;

    for key in path {
        current = current
            .get(*key)
            .ok_or_else(|| invalid_data_message(format!("Missing {}", path.join("."))))?;
    }

    current
        .as_object()
        .ok_or_else(|| invalid_data_message(format!("{} must be an object", path.join("."))))
}

fn array_field<'a>(value: &'a Value, path: &[&str], label: &str) -> io::Result<&'a Vec<Value>> {
    let mut current = value;

    for key in path {
        current = current
            .get(*key)
            .ok_or_else(|| invalid_data_message(format!("Missing {label}")))?;
    }

    current
        .as_array()
        .ok_or_else(|| invalid_data_message(format!("{label} must be an array")))
}

fn safe_content_path(content_dir: &Path, relative_path: &str, label: &str) -> io::Result<PathBuf> {
    assert_safe_relative_path(relative_path, label)?;
    Ok(content_dir.join(relative_path))
}
