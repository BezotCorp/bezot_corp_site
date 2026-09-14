use std::fs;
use std::io;
use std::path::Path;

use common::{PostEditorState, invalid_data, invalid_input, read_json};
use serde_json::Value;

use crate::post_document::PostDocument;
use crate::post_reader::load_post_editor;
use crate::redirect_writer::record_slug_redirect;

pub fn save_post(project_root: &Path, editor: &PostEditorState) -> io::Result<()> {
    validate_editor(editor)?;
    record_slug_redirects_if_changed(project_root, editor)?;

    let content_dir = project_root.join("content");
    let post_relative_path = format!("posts/{}/{}.json", editor.date, editor.id);
    let post_path = content_dir.join("blog").join(&post_relative_path);
    write_post_document(&post_path, editor)?;
    update_blog_index(&content_dir.join("blog/index.json"), &post_relative_path)
}

fn record_slug_redirects_if_changed(
    project_root: &Path,
    editor: &PostEditorState,
) -> io::Result<()> {
    let Ok(previous) = load_post_editor(project_root, &editor.id) else {
        return Ok(());
    };

    record_slug_redirect(project_root, "fr-fr", &previous.fr.slug, &editor.fr.slug)?;
    record_slug_redirect(project_root, "en-us", &previous.en.slug, &editor.en.slug)
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
    validate_required("Le paragraphe anglais", &editor.en.paragraph)?;
    validate_affiliate_fields("français", &editor.fr)?;
    validate_affiliate_fields("anglais", &editor.en)
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

fn validate_affiliate_fields(
    locale_label: &str,
    editor: &common::PostLocaleEditor,
) -> io::Result<()> {
    if editor.affiliate_url.trim().is_empty() {
        return Ok(());
    }

    validate_https_url(
        &format!("L’URL d’affiliation {locale_label}"),
        &editor.affiliate_url,
    )?;
    validate_required(
        &format!("Le titre d’affiliation {locale_label}"),
        &editor.affiliate_title,
    )?;
    validate_required(
        &format!("Le libellé d’affiliation {locale_label}"),
        &editor.affiliate_label,
    )?;
    validate_required(
        &format!("La mention d’affiliation {locale_label}"),
        &editor.affiliate_disclosure,
    )
}

fn validate_https_url(label: &str, value: &str) -> io::Result<()> {
    if value.starts_with("https://") {
        Ok(())
    } else {
        Err(invalid_input(format!(
            "{label} doit commencer par https://"
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
