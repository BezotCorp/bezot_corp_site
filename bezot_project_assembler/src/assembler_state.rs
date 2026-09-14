use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::file_hash;
use crate::input_file::InputFile;
use crate::input_role::InputRole;
use crate::output_file::OutputFile;
use crate::project_snapshot::ProjectSnapshot;
use crate::site_generator::{GENERATED_SOURCE_PREFIX, GeneratedFile};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssemblerState {
    pub version: u32,
    pub project_root: PathBuf,
    pub inputs: Vec<InputFile>,
    pub outputs: Vec<OutputFile>,
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
                role: InputRole::role_from_path(&file.relative_path),
            })
            .collect();

        let mut outputs: Vec<OutputFile> = inputs
            .iter()
            .filter_map(InputRole::output_from_input)
            .collect();

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
