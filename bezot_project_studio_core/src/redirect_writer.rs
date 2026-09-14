use std::fs;
use std::io;
use std::path::Path;

use common::{invalid_data, read_json};
use serde_json::Value;

/// Records a 301 redirect from a post's previous slug to its new one, so a
/// slug edited in the studio never turns a published URL into a silent 404.
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

    let already_recorded = entries
        .iter()
        .any(|entry| entry.get("from").and_then(Value::as_str) == Some(from.as_str()));

    if already_recorded {
        return Ok(());
    }

    entries.push(serde_json::json!({
        "from": from,
        "to": to,
        "status": 301,
    }));

    let source = format!(
        "{}\n",
        serde_json::to_string_pretty(&redirects).map_err(invalid_data)?
    );
    fs::write(redirects_path, source)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    #[test]
    fn records_a_redirect_when_the_slug_changes() {
        let project_root = temporary_project_root();
        let content_dir = project_root.join("content");
        fs::create_dir_all(&content_dir).unwrap();
        fs::write(content_dir.join("redirects.json"), "[]\n").unwrap();

        record_slug_redirect(
            &project_root,
            "fr-fr",
            "blog/ancien-slug",
            "blog/nouveau-slug",
        )
        .unwrap();

        let redirects = read_json(&content_dir.join("redirects.json")).unwrap();
        let entries = redirects.as_array().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(
            entries[0].get("from").and_then(Value::as_str),
            Some("/fr-fr/blog/ancien-slug")
        );
        assert_eq!(
            entries[0].get("to").and_then(Value::as_str),
            Some("/fr-fr/blog/nouveau-slug")
        );
        assert_eq!(entries[0].get("status").and_then(Value::as_u64), Some(301));

        fs::remove_dir_all(project_root).unwrap();
    }

    #[test]
    fn does_not_duplicate_an_existing_redirect() {
        let project_root = temporary_project_root();
        let content_dir = project_root.join("content");
        fs::create_dir_all(&content_dir).unwrap();
        fs::write(
            content_dir.join("redirects.json"),
            r#"[{"from":"/fr-fr/blog/ancien-slug","to":"/fr-fr/blog/nouveau-slug","status":301}]
"#,
        )
        .unwrap();

        record_slug_redirect(
            &project_root,
            "fr-fr",
            "blog/ancien-slug",
            "blog/autre-slug",
        )
        .unwrap();

        let redirects = read_json(&content_dir.join("redirects.json")).unwrap();
        assert_eq!(redirects.as_array().unwrap().len(), 1);

        fs::remove_dir_all(project_root).unwrap();
    }

    #[test]
    fn does_nothing_when_the_slug_is_unchanged() {
        let project_root = temporary_project_root();
        let content_dir = project_root.join("content");
        fs::create_dir_all(&content_dir).unwrap();
        fs::write(content_dir.join("redirects.json"), "[]\n").unwrap();

        record_slug_redirect(&project_root, "fr-fr", "blog/meme-slug", "blog/meme-slug").unwrap();

        let redirects = read_json(&content_dir.join("redirects.json")).unwrap();
        assert!(redirects.as_array().unwrap().is_empty());

        fs::remove_dir_all(project_root).unwrap();
    }

    fn temporary_project_root() -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!(
            "bezot_project_studio_core_redirect_writer_test_{}_{}",
            std::process::id(),
            stamp
        ))
    }
}
