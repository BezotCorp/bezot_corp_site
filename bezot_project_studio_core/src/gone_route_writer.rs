use std::fs;
use std::io;
use std::path::Path;

use common::{invalid_data, read_json};
use serde_json::Value;

/// Records a route as intentionally gone (HTTP 410) so a deleted piece of
/// published content leaves a clean, deliberate signal instead of turning
/// into an unexplained 404.
pub fn record_gone_route(content_dir: &Path, locale: &str, slug: &str) -> io::Result<()> {
    if slug.trim().is_empty() {
        return Ok(());
    }

    let gone_path = content_dir.join("gone-routes.json");
    let mut gone = read_json(&gone_path)?;
    let routes = gone
        .as_array_mut()
        .ok_or_else(|| invalid_data("content/gone-routes.json must be an array"))?;

    let route = format!("/{locale}/{slug}");
    let already_recorded = routes
        .iter()
        .any(|value| value.as_str() == Some(route.as_str()));

    if !already_recorded {
        routes.push(Value::String(route));
        let source = format!(
            "{}\n",
            serde_json::to_string_pretty(&gone).map_err(invalid_data)?
        );
        fs::write(gone_path, source)?;
    }

    Ok(())
}
