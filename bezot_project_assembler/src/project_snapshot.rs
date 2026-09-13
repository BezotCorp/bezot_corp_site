use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::project_config::ProjectConfig;
use crate::project_file::ProjectFile;

#[derive(Debug)]
pub struct ProjectSnapshot {
    pub project_root: PathBuf,
    pub files: Vec<ProjectFile>,
}

impl ProjectSnapshot {
    pub fn scan_project(project_root: &Path, config: &ProjectConfig) -> io::Result<Self> {
        let canonical_project_root = fs::canonicalize(project_root)?;
        let mut files = Vec::new();

        for input_root in &config.input_roots {
            let root = canonical_project_root.join(input_root);

            if !root.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("input root does not exist: {}", root.display()),
                ));
            }

            ProjectFile::collect_files(&canonical_project_root, &root, config, &mut files)?;
        }

        files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));

        Ok(Self {
            project_root: canonical_project_root,
            files,
        })
    }
}
