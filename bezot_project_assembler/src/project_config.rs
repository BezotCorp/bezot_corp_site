use std::path::PathBuf;

#[derive(Debug)]
pub struct ProjectConfig {
    pub input_roots: Vec<PathBuf>,
    pub ignored_paths: Vec<String>,
}

impl ProjectConfig {
    pub fn default_site_project() -> Self {
        Self {
            input_roots: vec![
                PathBuf::from("content"),
                PathBuf::from("public"),
                PathBuf::from("scripts"),
                PathBuf::from("src"),
            ],
            ignored_paths: vec![
                ".git/".to_string(),
                "node_modules/".to_string(),
                "dist/".to_string(),
                "prebuild/".to_string(),
                "target/".to_string(),
            ],
        }
    }
}
