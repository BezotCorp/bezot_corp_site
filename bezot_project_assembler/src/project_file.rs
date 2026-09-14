use std::fs;
use std::io;
use std::path::Path;

use crate::file_hash::{fnv1a_64, hex_u64};
use crate::project_config::ProjectConfig;

#[derive(Debug)]
pub struct ProjectFile {
    pub relative_path: String,
    pub size: u64,
    pub hash: String,
}

impl ProjectFile {
    pub(crate) fn collect_files(
        project_root: &Path,
        current_path: &Path,
        config: &ProjectConfig,
        files: &mut Vec<ProjectFile>,
    ) -> io::Result<()> {
        let mut entries = fs::read_dir(current_path)?.collect::<Result<Vec<_>, io::Error>>()?;

        entries.sort_by_key(|entry| entry.path());

        for entry in entries {
            let path = entry.path();
            let relative_path = relative_path(project_root, &path)?;

            if should_ignore(&relative_path, config) {
                continue;
            }

            let metadata = entry.metadata()?;

            if metadata.is_dir() {
                Self::collect_files(project_root, &path, config, files)?;
                continue;
            }

            if !metadata.is_file() {
                continue;
            }

            let bytes = fs::read(&path)?;

            files.push(Self {
                relative_path,
                size: metadata.len(),
                hash: hex_u64(fnv1a_64(&bytes)),
            });
        }

        Ok(())
    }
}

fn relative_path(project_root: &Path, path: &Path) -> io::Result<String> {
    Ok(path
        .strip_prefix(project_root)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?
        .to_string_lossy()
        .replace('\\', "/"))
}

fn should_ignore(relative_path: &str, config: &ProjectConfig) -> bool {
    let normalized = if relative_path.ends_with('/') {
        relative_path.to_string()
    } else {
        format!("{relative_path}/")
    };

    config.ignored_paths.iter().any(|ignored| {
        relative_path == ignored.trim_end_matches('/')
            || normalized.starts_with(ignored)
            || relative_path.starts_with(ignored)
    })
}
