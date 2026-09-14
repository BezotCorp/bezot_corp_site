use std::io;

use serde_json::{Map, Value};

use crate::content_validator::invalid_data_message;

pub(crate) fn string_field<'a>(object: &'a Map<String, Value>, key: &str) -> io::Result<&'a str> {
    object
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| invalid_data_message(format!("{key} must be a non-empty string")))
}

pub(crate) fn string_at<'a>(value: &'a Value, path: &[&str], label: &str) -> io::Result<&'a str> {
    let mut current = value;

    for key in path {
        current = current
            .get(*key)
            .ok_or_else(|| invalid_data_message(format!("Missing {label}")))?;
    }

    current
        .as_str()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| invalid_data_message(format!("{label} must be a non-empty string")))
}
