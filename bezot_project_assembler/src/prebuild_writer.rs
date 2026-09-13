use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::Path;

use crate::assembler_state::AssemblerState;
use crate::site_generator::{GENERATED_SOURCE_PREFIX, GeneratedFile};

#[derive(Debug)]
pub struct PrebuildMaterialization {
    pub written: usize,
    pub unchanged: usize,
    pub deleted: usize,
}

impl PrebuildMaterialization {
    pub fn has_changes(&self) -> bool {
        self.written > 0 || self.deleted > 0
    }
}

pub fn materialize_prebuild_sources(
    old_state: Option<&AssemblerState>,
    new_state: &AssemblerState,
    generated_files: &[GeneratedFile],
) -> io::Result<PrebuildMaterialization> {
    let mut result = PrebuildMaterialization {
        written: 0,
        unchanged: 0,
        deleted: 0,
    };

    delete_removed_outputs(old_state, new_state, &mut result)?;
    materialize_root_files(new_state, &mut result)?;
    materialize_source_files(new_state, &mut result)?;
    materialize_generated_files(new_state, generated_files, &mut result)?;

    Ok(result)
}

fn materialize_root_files(
    state: &AssemblerState,
    result: &mut PrebuildMaterialization,
) -> io::Result<()> {
    let root_files = [
        "package.json",
        "index.html",
        "eslint.config.js",
        "tsconfig.json",
        "tsconfig.app.json",
        "tsconfig.node.json",
        "vite.config.ts",
        "vite.config.js",
        "pnpm-lock.yaml",
    ];

    for relative_path in root_files {
        let source_path = state.project_root.join(relative_path);

        if !source_path.is_file() {
            continue;
        }

        let target_path = state.project_root.join("prebuild").join(relative_path);
        let bytes = fs::read(&source_path)?;

        count_write(write_if_changed(&target_path, &bytes)?, result);
    }

    Ok(())
}

fn materialize_source_files(
    state: &AssemblerState,
    result: &mut PrebuildMaterialization,
) -> io::Result<()> {
    for output in &state.outputs {
        if output.source_path.starts_with(GENERATED_SOURCE_PREFIX) {
            continue;
        }

        let source_path = state.project_root.join(&output.source_path);
        let target_path = state.project_root.join(&output.path);
        let bytes = fs::read(&source_path)?;

        count_write(write_if_changed(&target_path, &bytes)?, result);
    }

    Ok(())
}

fn materialize_generated_files(
    state: &AssemblerState,
    files: &[GeneratedFile],
    result: &mut PrebuildMaterialization,
) -> io::Result<()> {
    for file in files {
        let target_path = state
            .project_root
            .join("prebuild")
            .join(&file.relative_path);
        count_write(write_if_changed(&target_path, &file.bytes)?, result);
    }

    Ok(())
}

fn delete_removed_outputs(
    old_state: Option<&AssemblerState>,
    new_state: &AssemblerState,
    result: &mut PrebuildMaterialization,
) -> io::Result<()> {
    let Some(old_state) = old_state else {
        return Ok(());
    };

    let new_output_paths: BTreeSet<&str> = new_state
        .outputs
        .iter()
        .map(|output| output.path.as_str())
        .collect();

    for old_output in &old_state.outputs {
        if !new_output_paths.contains(old_output.path.as_str()) {
            let path = new_state.project_root.join(&old_output.path);

            if path.exists() {
                fs::remove_file(&path)?;
                result.deleted += 1;
            }
        }
    }

    Ok(())
}

fn write_if_changed(path: &Path, bytes: &[u8]) -> io::Result<bool> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    match fs::read(path) {
        Ok(existing) if existing == bytes => Ok(false),
        _ => {
            fs::write(path, bytes)?;
            Ok(true)
        }
    }
}

fn count_write(written: bool, result: &mut PrebuildMaterialization) {
    if written {
        result.written += 1;
    } else {
        result.unchanged += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use crate::assembler_state::{AssemblerState, InputFile, InputRole, OutputFile};

    #[test]
    fn writes_generated_files_into_prebuild() {
        let project_root = temp_project_root();
        fs::create_dir_all(project_root.join("src")).unwrap();
        fs::write(
            project_root.join("src/App.tsx"),
            "export const App = () => null;\n",
        )
        .unwrap();
        fs::write(project_root.join("package.json"), "{}\n").unwrap();

        let state = AssemblerState {
            version: 2,
            project_root: project_root.clone(),
            inputs: vec![InputFile {
                path: "src/App.tsx".to_string(),
                size: 30,
                hash: "source-hash".to_string(),
                role: InputRole::SourceCode,
            }],
            outputs: vec![
                OutputFile {
                    path: "prebuild/src/App.tsx".to_string(),
                    source_path: "src/App.tsx".to_string(),
                    source_hash: "source-hash".to_string(),
                },
                OutputFile {
                    path: "prebuild/assembled/main.tsx".to_string(),
                    source_path: format!("{GENERATED_SOURCE_PREFIX}assembled/main.tsx"),
                    source_hash: "generated-hash".to_string(),
                },
            ],
        };
        let generated_files = vec![GeneratedFile {
            relative_path: "assembled/main.tsx".to_string(),
            bytes: b"generated entry".to_vec(),
        }];

        let result = materialize_prebuild_sources(None, &state, &generated_files).unwrap();

        assert!(result.written >= 2);
        assert_eq!(
            fs::read_to_string(project_root.join("prebuild/assembled/main.tsx")).unwrap(),
            "generated entry"
        );
        assert_eq!(
            fs::read_to_string(project_root.join("prebuild/src/App.tsx")).unwrap(),
            "export const App = () => null;\n"
        );

        fs::remove_dir_all(project_root).unwrap();
    }

    fn temp_project_root() -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("bezot-project-assembler-test-{suffix}"))
    }
}
