use std::io;
use std::path::Path;

use crate::content_entry::ContentEntry;
use crate::content_reader::read_content_entries;
use crate::errors::{invalid_data, invalid_input};
use crate::output_format::OutputFormat;

pub fn run_content_command(project_root: &Path, args: &[String]) -> io::Result<()> {
    match args.first().map(String::as_str) {
        Some("list") => {
            let format = parse_format_argument(&args[1..])?;
            let entries = read_content_entries(project_root)?;
            print_content_entries(&entries, format)
        }
        Some(value) => Err(invalid_input(format!(
            "unknown content command \"{value}\"; expected list"
        ))),
        None => Err(invalid_input("missing content command; expected list")),
    }
}

fn parse_format_argument(args: &[String]) -> io::Result<OutputFormat> {
    match args {
        [] => OutputFormat::parse(None).map_err(invalid_input),
        [flag, value] if flag == "--format" => {
            OutputFormat::parse(Some(value.as_str())).map_err(invalid_input)
        }
        _ => Err(invalid_input("usage: content list [--format text|json]")),
    }
}

fn print_content_entries(entries: &[ContentEntry], format: OutputFormat) -> io::Result<()> {
    match format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(entries).map_err(invalid_data)?
            );
        }
        OutputFormat::Text => {
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
