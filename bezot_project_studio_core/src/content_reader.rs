use serde_json::Value;
use std::io;

use common::invalid_data;

pub(crate) fn required_string<'a>(value: &'a Value, field: &str) -> io::Result<&'a str> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| invalid_data(format!("{field} must be a string")))
}
