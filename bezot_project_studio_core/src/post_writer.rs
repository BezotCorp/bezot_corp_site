use std::fs;
use std::io;
use std::path::Path;

use common::{invalid_data, invalid_input, read_json};
use serde_json::Value;

use crate::post_document::PostDocument;
use crate::post_editor_state::PostEditorState;

pub fn save_post(project_root: &Path, editor: &PostEditorState) -> io::Result<()> {
    validate_editor(editor)?;

    let content_dir = project_root.join("content");
    let post_relative_path = format!("posts/{}/{}.json", editor.date, editor.id);
    let post_path = content_dir.join("blog").join(&post_relative_path);
    write_post_document(&post_path, editor)?;
    update_blog_index(&content_dir.join("blog/index.json"), &post_relative_path)
}

fn validate_editor(editor: &PostEditorState) -> io::Result<()> {
    validate_slug_part("post id", &editor.id)?;
    validate_date(&editor.date)?;
    validate_required("Le statut", &editor.status)?;
    validate_required("L’auteur", &editor.author)?;
    validate_required("Le titre français", &editor.fr.title)?;
    validate_required("Le slug français", &editor.fr.slug)?;
    validate_required("Le paragraphe français", &editor.fr.paragraph)?;
    validate_required("Le titre anglais", &editor.en.title)?;
    validate_required("Le slug anglais", &editor.en.slug)?;
    validate_required("Le paragraphe anglais", &editor.en.paragraph)
}

fn validate_required(label: &str, value: &str) -> io::Result<()> {
    if value.trim().is_empty() {
        return Err(invalid_input(format!("{label} est obligatoire")));
    }

    Ok(())
}

fn validate_slug_part(label: &str, value: &str) -> io::Result<()> {
    validate_required(label, value)?;

    if value.chars().all(|character| {
        character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
    }) {
        Ok(())
    } else {
        Err(invalid_input(format!(
            "{label} doit contenir uniquement des lettres minuscules, des chiffres et des tirets"
        )))
    }
}

fn validate_date(value: &str) -> io::Result<()> {
    let valid = value.len() == 10
        && value.chars().enumerate().all(|(index, character)| {
            matches!(index, 4 | 7) == (character == '-')
                && (character.is_ascii_digit() || character == '-')
        });

    if valid {
        Ok(())
    } else {
        Err(invalid_input("La date doit utiliser le format YYYY-MM-DD"))
    }
}

fn write_post_document(post_path: &Path, editor: &PostEditorState) -> io::Result<()> {
    let parent = post_path
        .parent()
        .ok_or_else(|| invalid_input("post path has no parent directory"))?;
    fs::create_dir_all(parent)?;

    let document = PostDocument::from(editor);
    let source = format!(
        "{}\n",
        serde_json::to_string_pretty(&document).map_err(invalid_data)?
    );
    fs::write(post_path, source)
}

fn update_blog_index(index_path: &Path, post_relative_path: &str) -> io::Result<()> {
    let mut index = read_json(index_path)?;
    let post_paths = index
        .get_mut("postPaths")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| invalid_data("content/blog/index.json postPaths must be an array"))?;

    let already_indexed = post_paths
        .iter()
        .any(|path| path.as_str() == Some(post_relative_path));

    if !already_indexed {
        post_paths.insert(0, Value::String(post_relative_path.to_string()));
        write_json(index_path, &index)?;
    }

    Ok(())
}

fn write_json(path: &Path, value: &Value) -> io::Result<()> {
    let source = format!(
        "{}\n",
        serde_json::to_string_pretty(value).map_err(invalid_data)?
    );
    fs::write(path, source)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    #[test]
    fn saves_post_and_registers_it_in_blog_index() {
        let project_root = temporary_project_root();
        let blog_dir = project_root.join("content/blog");
        fs::create_dir_all(&blog_dir).unwrap();
        fs::write(
            blog_dir.join("index.json"),
            r#"{
  "status": "enabled",
  "entryPageId": "blog",
  "postPaths": []
}
"#,
        )
        .unwrap();

        let editor = PostEditorState::default();

        save_post(&project_root, &editor).unwrap();

        let post_path = project_root
            .join("content/blog/posts")
            .join(&editor.date)
            .join(format!("{}.json", editor.id));
        assert!(post_path.exists());

        let index = read_json(&blog_dir.join("index.json")).unwrap();
        let post_paths = index.get("postPaths").and_then(Value::as_array).unwrap();
        assert_eq!(
            post_paths.first().and_then(Value::as_str),
            Some("posts/2026-09-14/new-blog-post.json")
        );

        fs::remove_dir_all(project_root).unwrap();
    }

    fn temporary_project_root() -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!(
            "bezot_project_studio_core_post_writer_test_{}_{}",
            std::process::id(),
            stamp
        ))
    }
}
