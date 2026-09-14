use std::io;

use common::{invalid_data, invalid_input};

use crate::content_entry::ContentEntry;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

impl OutputFormat {
    pub fn parse(value: Option<&str>) -> Result<Self, String> {
        match value {
            None => Ok(Self::Text),
            Some("text") => Ok(Self::Text),
            Some("json") => Ok(Self::Json),
            Some(value) => Err(format!(
                "unknown output format \"{value}\"; expected text or json"
            )),
        }
    }

    pub(crate) fn print_content_entries(
        entries: &[ContentEntry],
        format: OutputFormat,
    ) -> io::Result<()> {
        match format {
            Self::Json => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(entries).map_err(invalid_data)?
                );
            }
            Self::Text => {
                for entry in entries {
                    println!("{} {}", entry.kind, entry.id);
                    for locale in &entry.locales {
                        println!(
                            "  {} [{}] {} /{} blocks:{}",
                            locale.locale,
                            locale.status,
                            locale.title,
                            locale.slug.trim_start_matches('/'),
                            locale.block_count
                        );
                    }
                }
            }
        }

        Ok(())
    }

    pub(crate) fn parse_format_argument(args: &[String]) -> io::Result<Self> {
        match args {
            [] => Self::parse(None).map_err(invalid_input),
            [flag, value] if flag == "--format" => {
                Self::parse(Some(value.as_str())).map_err(invalid_input)
            }
            _ => Err(invalid_input("usage: content list [--format text|json]")),
        }
    }
}
