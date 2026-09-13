use serde::Serialize;
use serde_json::Value;
use std::{io, path::Path};

use common::{invalid_data, read_json};

#[derive(Debug, Serialize)]
pub struct LocalizedContentSummary {
    pub locale: String,
    pub status: String,
    pub title: String,
    pub slug: String,
    pub updated_at: Option<String>,
    pub block_count: usize,
}

impl LocalizedContentSummary {
    pub(crate) fn read_page_locales(page_dir: &Path, page_index: &Value) -> io::Result<Vec<Self>> {
        let locale_index = page_index
            .get("locales")
            .and_then(Value::as_object)
            .ok_or_else(|| invalid_data("page locales must be an object"))?;
        let mut locales = Vec::new();

        for (locale, metadata) in locale_index {
            let content = read_json(&page_dir.join(format!("{locale}.json")))?;
            locales.push(Self::localized_summary(locale, metadata, &content)?);
        }

        locales.sort_by(|left, right| left.locale.cmp(&right.locale));
        Ok(locales)
    }

    pub(crate) fn localized_summary(
        locale: &str,
        metadata: &Value,
        content: &Value,
    ) -> io::Result<Self> {
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

        Ok(Self {
            locale: locale.to_string(),
            status,
            title,
            slug,
            updated_at,
            block_count,
        })
    }

    pub(crate) fn entry_status(locales: &[LocalizedContentSummary]) -> String {
        if locales.iter().any(|locale| locale.status == "published") {
            "published".to_string()
        } else if let Some(locale) = locales.first() {
            locale.status.clone()
        } else {
            "unknown".to_string()
        }
    }
}
