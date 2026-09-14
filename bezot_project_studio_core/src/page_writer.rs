use std::fs;
use std::io;
use std::path::Path;

use common::{
    PageBlock, PageEditorState, PageLocaleEditor, invalid_data, invalid_input, read_json,
};
use serde::Serialize;
use serde_json::Value;

use crate::page_document::{PageIndexDocument, PageLocaleContentDocument};
use crate::page_reader::load_page_editor;
use crate::redirect_writer::record_slug_redirect;

pub fn save_page(project_root: &Path, editor: &PageEditorState) -> io::Result<()> {
    validate_editor(editor)?;
    record_slug_redirects_if_changed(project_root, editor)?;

    let content_dir = project_root.join("content");
    let page_dir = content_dir.join("pages").join(&editor.id);
    fs::create_dir_all(&page_dir)?;

    write_json(
        &page_dir.join("index.json"),
        &PageIndexDocument::from(editor),
    )?;
    write_json(
        &page_dir.join("fr-fr.json"),
        &PageLocaleContentDocument::from(&editor.fr),
    )?;
    write_json(
        &page_dir.join("en-us.json"),
        &PageLocaleContentDocument::from(&editor.en),
    )?;

    register_page_id(&content_dir.join("pages/index.json"), &editor.id)
}

fn record_slug_redirects_if_changed(
    project_root: &Path,
    editor: &PageEditorState,
) -> io::Result<()> {
    let Ok(previous) = load_page_editor(project_root, &editor.id) else {
        return Ok(());
    };

    record_slug_redirect(project_root, "fr-fr", &previous.fr.slug, &editor.fr.slug)?;
    record_slug_redirect(project_root, "en-us", &previous.en.slug, &editor.en.slug)
}

fn validate_editor(editor: &PageEditorState) -> io::Result<()> {
    validate_id(&editor.id)?;
    validate_locale("français", &editor.fr)?;
    validate_locale("anglais", &editor.en)
}

fn validate_id(value: &str) -> io::Result<()> {
    let valid = !value.is_empty()
        && value.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        });

    if valid {
        Ok(())
    } else {
        Err(invalid_input(
            "L’identifiant de page doit contenir uniquement des lettres minuscules, des chiffres et des tirets",
        ))
    }
}

fn validate_locale(locale_label: &str, editor: &PageLocaleEditor) -> io::Result<()> {
    validate_required(&format!("Le statut ({locale_label})"), &editor.status)?;
    validate_required(
        &format!("La date de mise à jour ({locale_label})"),
        &editor.updated_at,
    )?;
    validate_required(&format!("Le titre SEO ({locale_label})"), &editor.title)?;

    if editor.blocks.is_empty() {
        return Err(invalid_input(format!(
            "La page doit contenir au moins un bloc ({locale_label})"
        )));
    }

    for (index, block) in editor.blocks.iter().enumerate() {
        validate_block(locale_label, index, block)?;
    }

    Ok(())
}

fn validate_block(locale_label: &str, index: usize, block: &PageBlock) -> io::Result<()> {
    match block {
        PageBlock::Hero { title, .. } => validate_required(
            &format!("Le titre du bloc hero {index} ({locale_label})"),
            title,
        ),
        PageBlock::Paragraph { text } => validate_required(
            &format!("Le texte du bloc paragraphe {index} ({locale_label})"),
            text,
        ),
        PageBlock::MailLink { email, .. } => validate_required(
            &format!("L’email du bloc lien {index} ({locale_label})"),
            email,
        ),
        PageBlock::CardGrid { items } => {
            if items.is_empty() {
                return Err(invalid_input(format!(
                    "La grille de cartes {index} doit contenir au moins une carte ({locale_label})"
                )));
            }

            for item in items {
                validate_required(
                    &format!("Le titre d’une carte du bloc {index} ({locale_label})"),
                    &item.title,
                )?;
                validate_required(
                    &format!("Le texte d’une carte du bloc {index} ({locale_label})"),
                    &item.text,
                )?;
            }

            Ok(())
        }
        PageBlock::AffiliateCallout {
            title,
            url,
            label,
            disclosure,
            ..
        } => {
            validate_required(
                &format!("Le titre de l’encart {index} ({locale_label})"),
                title,
            )?;
            validate_https_url(&format!("L’URL de l’encart {index} ({locale_label})"), url)?;
            validate_required(
                &format!("Le libellé de l’encart {index} ({locale_label})"),
                label,
            )?;
            validate_required(
                &format!("La mention de l’encart {index} ({locale_label})"),
                disclosure,
            )
        }
        PageBlock::PostList { .. } => Ok(()),
    }
}

fn validate_required(field_label: &str, value: &str) -> io::Result<()> {
    if value.trim().is_empty() {
        Err(invalid_input(format!("{field_label} est obligatoire")))
    } else {
        Ok(())
    }
}

fn validate_https_url(field_label: &str, value: &str) -> io::Result<()> {
    if value.starts_with("https://") {
        Ok(())
    } else {
        Err(invalid_input(format!(
            "{field_label} doit commencer par https://"
        )))
    }
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    let source = format!(
        "{}\n",
        serde_json::to_string_pretty(value).map_err(invalid_data)?
    );
    fs::write(path, source)
}

fn register_page_id(index_path: &Path, page_id: &str) -> io::Result<()> {
    let mut index = read_json(index_path)?;
    let page_ids = index
        .get_mut("pageIds")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| invalid_data("content/pages/index.json pageIds must be an array"))?;

    let already_indexed = page_ids.iter().any(|id| id.as_str() == Some(page_id));

    if !already_indexed {
        page_ids.push(Value::String(page_id.to_string()));
        write_json(index_path, &index)?;
    }

    Ok(())
}
