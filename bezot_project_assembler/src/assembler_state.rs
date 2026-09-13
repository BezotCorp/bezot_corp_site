use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::file_hash;
use crate::project_scan::ProjectSnapshot;
use crate::site_generator::{GENERATED_SOURCE_PREFIX, GeneratedFile};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssemblerState {
    pub version: u32,
    pub project_root: PathBuf,
    pub inputs: Vec<InputFile>,
    pub outputs: Vec<OutputFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputFile {
    pub path: String,
    pub size: u64,
    pub hash: String,
    pub role: InputRole,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InputRole {
    Content,
    SourceCode,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutputFile {
    pub path: String,
    pub source_path: String,
    pub source_hash: String,
}

impl AssemblerState {
    pub fn from_snapshot(snapshot: &ProjectSnapshot, generated_files: &[GeneratedFile]) -> Self {
        let inputs: Vec<InputFile> = snapshot
            .files
            .iter()
            .map(|file| InputFile {
                path: file.relative_path.clone(),
                size: file.size,
                hash: file.hash.clone(),
                role: role_from_path(&file.relative_path),
            })
            .collect();

        let mut outputs: Vec<OutputFile> = inputs.iter().filter_map(output_from_input).collect();

        outputs.extend(generated_files.iter().map(|file| OutputFile {
            path: format!("prebuild/{}", file.relative_path),
            source_path: format!("{GENERATED_SOURCE_PREFIX}{}", file.relative_path),
            source_hash: file_hash::hex_u64(file_hash::fnv1a_64(&file.bytes)),
        }));
        outputs.sort_by(|left, right| left.path.cmp(&right.path));

        Self {
            version: 2,
            project_root: snapshot.project_root.clone(),
            inputs,
            outputs,
        }
    }
}

fn role_from_path(path: &str) -> InputRole {
    if path.starts_with("content/") {
        InputRole::Content
    } else if path.starts_with("scripts/")
        || path.starts_with("public/")
        || path.starts_with("src/")
    {
        InputRole::SourceCode
    } else {
        InputRole::Unknown
    }
}

fn output_from_input(input: &InputFile) -> Option<OutputFile> {
    match input.role {
        InputRole::Content | InputRole::SourceCode => Some(OutputFile {
            path: format!("prebuild/{}", input.path),
            source_path: input.path.clone(),
            source_hash: input.hash.clone(),
        }),
        InputRole::Unknown => None,
    }
}
