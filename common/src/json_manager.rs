use serde_json::Value;
use std::{fs, io, path::Path};

use crate::invalid_data;

pub fn read_json(path: &Path) -> io::Result<Value> {
    let source = fs::read_to_string(path)?;
    serde_json::from_str(&source).map_err(invalid_data)
}
