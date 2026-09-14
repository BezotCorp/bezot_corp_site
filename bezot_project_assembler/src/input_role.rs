use serde::{Deserialize, Serialize};

use crate::input_file::InputFile;
use crate::output_file::OutputFile;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InputRole {
    Content,
    SourceCode,
    Unknown,
}

impl InputRole {
    pub(crate) fn role_from_path(path: &str) -> Self {
        if path.starts_with("content/") {
            Self::Content
        } else if path.starts_with("scripts/")
            || path.starts_with("public/")
            || path.starts_with("src/")
        {
            Self::SourceCode
        } else {
            Self::Unknown
        }
    }

    pub(crate) fn output_from_input(input: &InputFile) -> Option<OutputFile> {
        match input.role {
            Self::Content | Self::SourceCode => Some(OutputFile {
                path: format!("prebuild/{}", input.path),
                source_path: input.path.clone(),
                source_hash: input.hash.clone(),
            }),
            Self::Unknown => None,
        }
    }
}
