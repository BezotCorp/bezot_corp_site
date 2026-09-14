use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::input_role::InputRole;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputFile {
    pub path: String,
    pub size: u64,
    pub hash: String,
    pub role: InputRole,
}

impl InputFile {
    pub fn inputs_by_path(inputs: &[InputFile]) -> BTreeMap<String, &Self> {
        inputs
            .iter()
            .map(|input| (input.path.clone(), input))
            .collect()
    }
}
