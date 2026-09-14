use std::fs;
use std::io;
use std::path::Path;

use common::{invalid_data, invalid_input, read_json};
use serde_json::Value;

use crate::gone_route_writer::record_gone_route;
use crate::page_reader::load_page_editor;

pub fn delete_page(project_root: &Path, page_id: &str) -> io::Result<()> {
    let content_dir = project_root.join("content");
    reject_structural_page(&content_dir, page_id)?;

    let editor = load_page_editor(project_root, page_id)?;
    let page_dir = content_dir.join("pages").join(page_id);

    fs::remove_dir_all(&page_dir)?;
    remove_from_pages_index(&content_dir.join("pages/index.json"), page_id)?;

    if editor.fr.status == "published" {
        record_gone_route(&content_dir, "fr-fr", &editor.fr.slug)?;
    }
    if editor.en.status == "published" {
        record_gone_route(&content_dir, "en-us", &editor.en.slug)?;
    }

    Ok(())
}

/// The home page and the blog's entry page are referenced by id from
/// content/index.json (homePageId, entryPageId); deleting either would
/// break the site build with no obvious error pointing back here.
fn reject_structural_page(content_dir: &Path, page_id: &str) -> io::Result<()> {
    let index = read_json(&content_dir.join("index.json"))?;

    let home_page_id = index
        .pointer("/sections/pages/homePageId")
        .and_then(Value::as_str);
    let blog_entry_page_id = index
        .pointer("/sections/blog/entryPageId")
        .and_then(Value::as_str);

    if home_page_id == Some(page_id) || blog_entry_page_id == Some(page_id) {
        return Err(invalid_input(format!(
            "page \"{page_id}\" cannot be deleted: it is referenced as a structural page (home page or blog entry page) in content/index.json"
        )));
    }

    Ok(())
}

fn remove_from_pages_index(index_path: &Path, page_id: &str) -> io::Result<()> {
    let mut index = read_json(index_path)?;
    let page_ids = index
        .get_mut("pageIds")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| invalid_data("content/pages/index.json pageIds must be an array"))?;

    page_ids.retain(|id| id.as_str() != Some(page_id));

    let source = format!(
        "{}\n",
        serde_json::to_string_pretty(&index).map_err(invalid_data)?
    );
    fs::write(index_path, source)
}
