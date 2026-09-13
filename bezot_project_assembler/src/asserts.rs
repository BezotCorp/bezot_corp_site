use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    path::Path,
};

use serde_json::Value;

use crate::{block_definition::FieldRule, content_validator::invalid_data_message};

pub(crate) fn assert_non_empty_string(value: Option<&Value>, label: &str) -> io::Result<()> {
    match value.and_then(Value::as_str) {
        Some(value) if !value.is_empty() => Ok(()),
        _ => Err(invalid_data_message(format!(
            "{label} must be a non-empty string"
        ))),
    }
}

pub(crate) fn assert_object(value: Option<&Value>, label: &str) -> io::Result<()> {
    match value.and_then(Value::as_object) {
        Some(_) => Ok(()),
        None => Err(invalid_data_message(format!("{label} must be an object"))),
    }
}

pub(crate) fn assert_status(
    value: Option<&Value>,
    allowed: &[&str],
    label: &str,
) -> io::Result<()> {
    let status = value
        .and_then(Value::as_str)
        .ok_or_else(|| invalid_data_message(format!("{label} has invalid status")))?;

    if !allowed.contains(&status) {
        return Err(invalid_data_message(format!(
            "{label} has invalid status \"{status}\". Expected: {}",
            allowed.join(", ")
        )));
    }

    Ok(())
}

pub(crate) fn assert_unique_strings(values: &[Value], label: &str) -> io::Result<()> {
    let mut seen = BTreeSet::new();

    for value in values {
        let string = value
            .as_str()
            .ok_or_else(|| invalid_data_message(format!("{label} must contain only strings")))?;

        if seen.contains(string) {
            return Err(invalid_data_message(format!(
                "{label} contains duplicate value \"{string}\""
            )));
        }

        seen.insert(string.to_string());
    }

    Ok(())
}

pub(crate) fn assert_array_contains_string(
    values: &[Value],
    expected: &str,
    label: &str,
) -> io::Result<()> {
    let contains = values.iter().any(|value| value.as_str() == Some(expected));

    if !contains {
        return Err(invalid_data_message(format!(
            "{label} \"{expected}\" is not listed in content/pages/index.json"
        )));
    }

    Ok(())
}

pub(crate) fn assert_safe_relative_path(relative_path: &str, label: &str) -> io::Result<()> {
    if relative_path.is_empty() {
        return Err(invalid_data_message(format!(
            "{label} must be a non-empty string"
        )));
    }

    let path = Path::new(relative_path);

    if path.is_absolute() {
        return Err(invalid_data_message(format!(
            "{label} must be relative, got absolute path \"{relative_path}\""
        )));
    }

    if relative_path.split('/').any(|segment| segment == "..") {
        return Err(invalid_data_message(format!(
            "{label} must not escape its index directory: \"{relative_path}\""
        )));
    }

    Ok(())
}

pub(crate) fn assert_no_locale_index_fields(
    page_id: &str,
    locale: &str,
    locale_content: &Value,
) -> io::Result<()> {
    let object = locale_content.as_object().ok_or_else(|| {
        invalid_data_message(format!(
            "Page \"{page_id}\" locale \"{locale}\" must be an object"
        ))
    })?;

    for field in ["status", "updatedAt"] {
        if object.contains_key(field) {
            return Err(invalid_data_message(format!(
                "Page \"{page_id}\" locale \"{locale}\" must not define \"{field}\". Put it in content/pages/{page_id}/index.json."
            )));
        }
    }

    Ok(())
}

pub(crate) fn assert_known_field(
    block_name: &str,
    fields: &BTreeMap<String, FieldRule>,
    field_name: &str,
) -> io::Result<()> {
    if !fields.contains_key(field_name) {
        return Err(invalid_data_message(format!(
            "Block \"{block_name}\" output references unknown field \"{field_name}\""
        )));
    }

    Ok(())
}

pub(crate) fn assert_file(path: &Path) -> io::Result<()> {
    if !path.is_file() {
        return Err(invalid_data_message(format!(
            "Missing file: {}",
            path.display()
        )));
    }

    Ok(())
}
