use std::fs;
use std::io;
use std::path::Path;

use common::{invalid_data, read_json};
use serde_json::Value;

/// Records a 301 redirect from a post's previous slug to its new one, so a
/// slug edited in the studio never turns a published URL into a silent 404.
///
/// Every slug a post has ever had must keep redirecting straight to its
/// *current* slug, even across several edits. So this also rewrites any
/// existing entry that pointed at the old slug to point at the new one
/// instead, collapsing what would otherwise become a multi-hop redirect
/// chain (old -> older -> current) into single hops (old -> current).
pub fn record_slug_redirect(
    project_root: &Path,
    locale: &str,
    old_slug: &str,
    new_slug: &str,
) -> io::Result<()> {
    if old_slug.trim().is_empty() || old_slug == new_slug {
        return Ok(());
    }

    let redirects_path = project_root.join("content/redirects.json");
    let mut redirects = read_json(&redirects_path)?;
    let entries = redirects
        .as_array_mut()
        .ok_or_else(|| invalid_data("content/redirects.json must be an array"))?;

    let from = format!("/{locale}/{old_slug}");
    let to = format!("/{locale}/{new_slug}");
    let mut already_recorded = false;

    for entry in entries.iter_mut() {
        let Some(object) = entry.as_object_mut() else {
            continue;
        };

        if object.get("to").and_then(Value::as_str) == Some(from.as_str()) {
            object.insert("to".to_string(), Value::String(to.clone()));
        }

        if object.get("from").and_then(Value::as_str) == Some(from.as_str()) {
            object.insert("to".to_string(), Value::String(to.clone()));
            already_recorded = true;
        }
    }

    if !already_recorded {
        entries.push(serde_json::json!({
            "from": from,
            "to": to,
            "status": 301,
        }));
    }

    // A slug reverted to a previous value can leave a self-redirect behind
    // (e.g. A -> B then B -> A collapses the A -> B entry into A -> A).
    entries.retain(|entry| entry.get("from") != entry.get("to"));

    let source = format!(
        "{}\n",
        serde_json::to_string_pretty(&redirects).map_err(invalid_data)?
    );
    fs::write(redirects_path, source)
}
