use std::{io, path::Path};

use crate::{content_entry::ContentEntry, output_format::OutputFormat};
use common::invalid_input;

pub fn run_content_command(project_root: &Path, args: &[String]) -> io::Result<()> {
    match args.first().map(String::as_str) {
        Some("list") => {
            let format = OutputFormat::parse_format_argument(&args[1..])?;
            let entries = ContentEntry::read_content_entries(project_root)?;
            OutputFormat::print_content_entries(&entries, format)
        }
        Some(value) => Err(invalid_input(format!(
            "unknown content command \"{value}\"; expected list"
        ))),
        None => Err(invalid_input("missing content command; expected list")),
    }
}
