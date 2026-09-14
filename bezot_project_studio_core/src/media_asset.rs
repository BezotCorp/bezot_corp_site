use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use common::invalid_input;
use serde::Serialize;

const ALLOWED_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "webp", "svg"];

#[derive(Debug, Serialize)]
pub struct MediaAsset {
    pub name: String,
    #[serde(rename = "publicPath")]
    pub public_path: String,
    #[serde(rename = "sizeBytes")]
    pub size_bytes: u64,
}

pub fn media_dir(project_root: &Path) -> PathBuf {
    project_root.join("public/media")
}

pub fn list_media_assets(project_root: &Path) -> io::Result<Vec<MediaAsset>> {
    let dir = media_dir(project_root);
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut assets = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        let size_bytes = entry.metadata()?.len();
        assets.push(MediaAsset {
            public_path: format!("/media/{name}"),
            name,
            size_bytes,
        });
    }

    assets.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(assets)
}

/// Copies `source_path` into `site/public/media`, sanitizing the file name so
/// it is safe to serve as a public URL segment and never overwriting an
/// existing asset — a colliding name gets a numeric suffix instead.
pub fn upload_media_asset(project_root: &Path, source_path: &Path) -> io::Result<MediaAsset> {
    let extension = source_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_lowercase)
        .ok_or_else(|| invalid_input("file has no extension; expected an image file"))?;

    if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
        return Err(invalid_input(format!(
            "unsupported extension \".{extension}\"; expected one of {}",
            ALLOWED_EXTENSIONS.join(", ")
        )));
    }

    let stem = source_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| invalid_input("file has no usable name"))?;
    let sanitized_stem = sanitize_stem(stem);

    let dir = media_dir(project_root);
    fs::create_dir_all(&dir)?;

    let (name, destination) = unique_destination(&dir, &sanitized_stem, &extension);
    fs::copy(source_path, &destination)?;

    Ok(MediaAsset {
        public_path: format!("/media/{name}"),
        size_bytes: fs::metadata(&destination)?.len(),
        name,
    })
}

fn sanitize_stem(stem: &str) -> String {
    let mut sanitized = String::with_capacity(stem.len());
    let mut previous_was_dash = false;

    for ch in stem.to_lowercase().chars() {
        let normalized = if ch.is_ascii_alphanumeric() {
            Some(ch)
        } else {
            None
        };

        match normalized {
            Some(ch) => {
                sanitized.push(ch);
                previous_was_dash = false;
            }
            None if !previous_was_dash && !sanitized.is_empty() => {
                sanitized.push('-');
                previous_was_dash = true;
            }
            None => {}
        }
    }

    let trimmed = sanitized.trim_end_matches('-').to_string();
    if trimmed.is_empty() {
        "media".to_string()
    } else {
        trimmed
    }
}

fn unique_destination(dir: &Path, stem: &str, extension: &str) -> (String, PathBuf) {
    let name = format!("{stem}.{extension}");
    let destination = dir.join(&name);
    if !destination.exists() {
        return (name, destination);
    }

    let mut suffix = 1u32;
    loop {
        let name = format!("{stem}-{suffix}.{extension}");
        let destination = dir.join(&name);
        if !destination.exists() {
            return (name, destination);
        }
        suffix += 1;
    }
}
