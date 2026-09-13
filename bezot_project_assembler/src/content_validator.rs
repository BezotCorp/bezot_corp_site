use std::collections::{BTreeMap, BTreeSet};
use std::io;

use serde_json::{Map, Value};

use crate::block_definition::{
    BlockDataset, BlockSource, CardinalityRule, FieldValueType, PlacementRule,
};
use crate::content_reader::ContentModel;

pub fn validate_content_model(
    content: &ContentModel,
    block_dataset: &BlockDataset,
) -> io::Result<()> {
    let site = required_object(&content.site, "website metadata")?;
    required_string(site, "name", "website metadata")?;
    required_string(site, "baseUrl", "website metadata")?;
    required_string(site, "defaultOgImage", "website metadata")?;
    let default_locale = required_string(site, "defaultLocale", "website metadata")?;
    let locales = required_string_array(site.get("locales"), "website metadata locales")?;

    if locales.is_empty() {
        return Err(invalid_data("website metadata locales must not be empty"));
    }

    let locale_set: BTreeSet<&str> = locales.iter().copied().collect();
    if locale_set.len() != locales.len() {
        return Err(invalid_data("website metadata locales must be unique"));
    }

    if !locale_set.contains(default_locale) {
        return Err(invalid_data(format!(
            "default locale \"{default_locale}\" is not listed in website metadata locales"
        )));
    }

    let mut entry_ids = BTreeSet::new();
    let mut routes = BTreeSet::new();

    for page in &content.pages {
        validate_entry(
            page,
            "page",
            &locale_set,
            block_dataset,
            &mut entry_ids,
            &mut routes,
        )?;
    }

    for post in &content.posts {
        validate_entry(
            post,
            "post",
            &locale_set,
            block_dataset,
            &mut entry_ids,
            &mut routes,
        )?;
    }

    validate_redirects(&content.redirects)?;
    validate_gone_routes(&content.gone)?;

    Ok(())
}

fn validate_entry(
    entry: &Value,
    kind: &str,
    known_locales: &BTreeSet<&str>,
    block_dataset: &BlockDataset,
    entry_ids: &mut BTreeSet<String>,
    routes: &mut BTreeSet<String>,
) -> io::Result<()> {
    let object = required_object(entry, kind)?;
    let id = required_string(object, "id", kind)?;

    if !entry_ids.insert(id.to_string()) {
        return Err(invalid_data(format!("duplicate content entry id \"{id}\"")));
    }

    let locales = object
        .get("locales")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_data(format!("{kind} \"{id}\" locales must be an object")))?;

    if locales.is_empty() {
        return Err(invalid_data(format!(
            "{kind} \"{id}\" must define at least one locale"
        )));
    }

    for (locale, localized_content) in locales {
        if !known_locales.contains(locale.as_str()) {
            return Err(invalid_data(format!(
                "{kind} \"{id}\" uses unknown locale \"{locale}\""
            )));
        }

        let label = format!("{kind} \"{id}\" locale \"{locale}\"");
        let localized = required_object(localized_content, &label)?;
        let slug = required_string(localized, "slug", &label)?;
        validate_seo(localized.get("seo"), &label)?;
        validate_blocks(localized.get("blocks"), &label, block_dataset)?;

        let route = if slug.is_empty() {
            format!("/{locale}/")
        } else {
            format!("/{locale}/{}/", slug.trim_matches('/'))
        };

        if !routes.insert(route.clone()) {
            return Err(invalid_data(format!(
                "duplicate published route \"{route}\""
            )));
        }
    }

    Ok(())
}

fn validate_seo(value: Option<&Value>, entry_label: &str) -> io::Result<()> {
    let label = format!("{entry_label} seo");
    let seo = value
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_data(format!("{label} must be an object")))?;

    for field in ["title", "ogTitle"] {
        required_string(seo, field, &label)?;
    }

    for field in ["description", "robots", "ogDescription", "ogImage"] {
        if let Some(value) = seo.get(field)
            && !value.is_string()
        {
            return Err(invalid_data(format!("{label}.{field} must be a string")));
        }
    }

    Ok(())
}

fn validate_blocks(
    value: Option<&Value>,
    entry_label: &str,
    block_dataset: &BlockDataset,
) -> io::Result<()> {
    let blocks = value
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_data(format!("{entry_label} blocks must be an array")))?;
    let mut counts = BTreeMap::<&str, usize>::new();

    for (index, block) in blocks.iter().enumerate() {
        let label = format!("{entry_label} block {index}");
        let object = required_object(block, &label)?;
        let block_type = required_string(object, "type", &label)?;
        let definition = block_dataset
            .blocks
            .get(block_type)
            .ok_or_else(|| invalid_data(format!("{label} has unknown type \"{block_type}\"")))?;

        if definition.source != BlockSource::Content {
            return Err(invalid_data(format!(
                "{label} type \"{block_type}\" is reserved for runtime assembly"
            )));
        }

        if definition.rules.placement == PlacementRule::PageStart && index != 0 {
            return Err(invalid_data(format!(
                "{label} type \"{block_type}\" must be the first block"
            )));
        }

        let count = counts.entry(block_type).or_default();
        *count += 1;
        if definition.rules.cardinality == CardinalityRule::AtMostOne && *count > 1 {
            return Err(invalid_data(format!(
                "{entry_label} contains more than one \"{block_type}\" block"
            )));
        }

        let empty_props = Map::new();
        let props = match object.get("props") {
            Some(Value::Object(props)) => props,
            Some(_) => return Err(invalid_data(format!("{label} props must be an object"))),
            None => &empty_props,
        };

        for field_name in props.keys() {
            if !definition.rules.fields.contains_key(field_name) {
                return Err(invalid_data(format!(
                    "{label} type \"{block_type}\" has unknown field \"{field_name}\""
                )));
            }
        }

        for (field_name, rule) in &definition.rules.fields {
            let field_value = props.get(field_name);

            if rule.required && field_value.is_none() {
                return Err(invalid_data(format!(
                    "{label} type \"{block_type}\" is missing required field \"{field_name}\""
                )));
            }

            if let Some(field_value) = field_value {
                validate_field_value(field_value, &rule.value_type, &label, field_name)?;
            }
        }
    }

    Ok(())
}

fn validate_field_value(
    value: &Value,
    value_type: &FieldValueType,
    block_label: &str,
    field_name: &str,
) -> io::Result<()> {
    let valid = match value_type {
        FieldValueType::String => value.as_str().is_some_and(|value| !value.is_empty()),
        FieldValueType::PositiveInteger => value.as_u64().is_some_and(|value| value > 0),
        FieldValueType::Boolean => value.is_boolean(),
        FieldValueType::CardList => value.as_array().is_some_and(|items| {
            !items.is_empty()
                && items.iter().all(|item| {
                    item.as_object().is_some_and(|item| {
                        ["title", "text"].iter().all(|field| {
                            item.get(*field)
                                .and_then(Value::as_str)
                                .is_some_and(|value| !value.is_empty())
                        }) && item.keys().all(|field| field == "title" || field == "text")
                    })
                })
        }),
    };

    if valid {
        Ok(())
    } else {
        Err(invalid_data(format!(
            "{block_label} field \"{field_name}\" does not satisfy {value_type:?}"
        )))
    }
}

fn validate_redirects(value: &Value) -> io::Result<()> {
    let redirects = value
        .as_array()
        .ok_or_else(|| invalid_data("redirects must be an array"))?;

    for (index, redirect) in redirects.iter().enumerate() {
        let label = format!("redirect {index}");
        let redirect = required_object(redirect, &label)?;
        required_string(redirect, "from", &label)?;
        required_string(redirect, "to", &label)?;
        let status = redirect.get("status").and_then(Value::as_u64);
        if !matches!(status, Some(301 | 302 | 307 | 308)) {
            return Err(invalid_data(format!("{label} has an invalid HTTP status")));
        }
    }

    Ok(())
}

fn validate_gone_routes(value: &Value) -> io::Result<()> {
    let routes = value
        .as_array()
        .ok_or_else(|| invalid_data("gone routes must be an array"))?;

    if routes.iter().all(Value::is_string) {
        Ok(())
    } else {
        Err(invalid_data("gone routes must contain only strings"))
    }
}

fn required_object<'a>(value: &'a Value, label: &str) -> io::Result<&'a Map<String, Value>> {
    value
        .as_object()
        .ok_or_else(|| invalid_data(format!("{label} must be an object")))
}

fn required_string<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    label: &str,
) -> io::Result<&'a str> {
    object
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty() || field == "slug")
        .ok_or_else(|| invalid_data(format!("{label}.{field} must be a string")))
}

fn required_string_array<'a>(value: Option<&'a Value>, label: &str) -> io::Result<Vec<&'a str>> {
    value
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_data(format!("{label} must be an array")))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .filter(|value| !value.is_empty())
                .ok_or_else(|| invalid_data(format!("{label} must contain non-empty strings")))
        })
        .collect()
}

fn invalid_data(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_definition::load_block_dataset;

    fn content_with_blocks(blocks: Value) -> ContentModel {
        ContentModel {
            site: serde_json::json!({
                "name": "Test site",
                "baseUrl": "https://example.com",
                "defaultLocale": "en-us",
                "locales": ["en-us"],
                "defaultOgImage": "/og.png"
            }),
            redirects: serde_json::json!([]),
            gone: serde_json::json!([]),
            pages: vec![serde_json::json!({
                "id": "home",
                "locales": {
                    "en-us": {
                        "status": "published",
                        "updatedAt": "2026-09-13",
                        "slug": "",
                        "seo": {
                            "title": "Home",
                            "description": "Home page",
                            "robots": "index, follow",
                            "ogTitle": "Home",
                            "ogDescription": "Home page",
                            "ogImage": "/og.png"
                        },
                        "blocks": blocks
                    }
                }
            })],
            posts: Vec::new(),
        }
    }

    #[test]
    fn accepts_content_that_satisfies_the_block_dataset() {
        let dataset = load_block_dataset().unwrap();
        let content = content_with_blocks(serde_json::json!([
            {"type": "hero", "props": {"title": "Home"}},
            {"type": "paragraph", "props": {"text": "Welcome"}}
        ]));

        validate_content_model(&content, &dataset).unwrap();
    }

    #[test]
    fn rejects_unknown_block_types() {
        let dataset = load_block_dataset().unwrap();
        let content = content_with_blocks(serde_json::json!([
            {"type": "unknown", "props": {}}
        ]));

        let error = validate_content_model(&content, &dataset).unwrap_err();
        assert!(error.to_string().contains("unknown type \"unknown\""));
    }

    #[test]
    fn rejects_runtime_blocks_declared_in_json() {
        let dataset = load_block_dataset().unwrap();
        let content = content_with_blocks(serde_json::json!([
            {"type": "post_meta", "props": {}}
        ]));

        let error = validate_content_model(&content, &dataset).unwrap_err();
        assert!(error.to_string().contains("reserved for runtime assembly"));
    }

    #[test]
    fn rejects_invalid_block_placement() {
        let dataset = load_block_dataset().unwrap();
        let content = content_with_blocks(serde_json::json!([
            {"type": "paragraph", "props": {"text": "Before"}},
            {"type": "hero", "props": {"title": "Too late"}}
        ]));

        let error = validate_content_model(&content, &dataset).unwrap_err();
        assert!(error.to_string().contains("must be the first block"));
    }
}
