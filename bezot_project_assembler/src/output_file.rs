use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutputFile {
    pub path: String,
    pub source_path: String,
    pub source_hash: String,
}

impl OutputFile {
    pub fn outputs_by_path(outputs: &[OutputFile]) -> BTreeMap<String, &Self> {
        outputs
            .iter()
            .map(|output| (output.path.clone(), output))
            .collect()
    }
}
