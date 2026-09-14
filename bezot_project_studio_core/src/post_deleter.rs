use std::fs;
use std::io;
use std::path::Path;

use common::{invalid_data, read_json};
use serde_json::Value;

use crate::gone_route_writer::record_gone_route;
use crate::post_reader::load_post_editor;

pub fn delete_post(project_root: &Path, post_id: &str) -> io::Result<()> {
    let editor = load_post_editor(project_root, post_id)?;

    let content_dir = project_root.join("content");
    let post_relative_path = format!("posts/{}/{}.json", editor.date, editor.id);
    let post_path = content_dir.join("blog").join(&post_relative_path);

    fs::remove_file(&post_path)?;
    remove_from_blog_index(&content_dir.join("blog/index.json"), &post_relative_path)?;

    if editor.status == "published" {
        record_gone_route(&content_dir, "fr-fr", &editor.fr.slug)?;
        record_gone_route(&content_dir, "en-us", &editor.en.slug)?;
    }

    Ok(())
}

fn remove_from_blog_index(index_path: &Path, post_relative_path: &str) -> io::Result<()> {
    let mut index = read_json(index_path)?;
    let post_paths = index
        .get_mut("postPaths")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| invalid_data("content/blog/index.json postPaths must be an array"))?;

    post_paths.retain(|path| path.as_str() != Some(post_relative_path));

    let source = format!(
        "{}\n",
        serde_json::to_string_pretty(&index).map_err(invalid_data)?
    );
    fs::write(index_path, source)
}
