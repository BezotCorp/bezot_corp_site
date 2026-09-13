use crate::{content_reader::required_string, localized_content_summary::LocalizedContentSummary};
use common::{invalid_data, read_json};
use serde::Serialize;
use serde_json::Value;
use std::{io, path::Path};

#[derive(Debug, Serialize)]
pub struct ContentEntry {
    pub kind: &'static str,
    pub id: String,
    pub status: String,
    pub locales: Vec<LocalizedContentSummary>,
}

impl ContentEntry {
    pub fn read_content_entries(project_root: &Path) -> io::Result<Vec<Self>> {
        let content_dir = project_root.join("content");
        let mut entries = Self::read_pages(&content_dir)?;
        entries.extend(Self::read_posts(&content_dir)?);
        Ok(entries)
    }

    fn read_pages(content_dir: &Path) -> io::Result<Vec<Self>> {
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
            let locales = LocalizedContentSummary::read_page_locales(&page_dir, &page_index)?;

            entries.push(Self {
                kind: "page",
                id: page_id.to_string(),
                status: LocalizedContentSummary::entry_status(&locales),
                locales,
            });
        }

        Ok(entries)
    }

    pub(crate) fn read_posts(content_dir: &Path) -> io::Result<Vec<Self>> {
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
                locales.push(LocalizedContentSummary::localized_summary(
                    locale, &post, content,
                )?);
            }

            locales.sort_by(|left, right| left.locale.cmp(&right.locale));
            entries.push(Self {
                kind: "post",
                id: id.to_string(),
                status,
                locales,
            });
        }

        Ok(entries)
    }
}
