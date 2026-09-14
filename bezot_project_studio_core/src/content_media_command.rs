use std::io;
use std::path::Path;

use common::{invalid_data, invalid_input};

use crate::media_asset::{list_media_assets, upload_media_asset};
use crate::output_format::OutputFormat;

pub fn run_content_media_command(project_root: &Path, args: &[String]) -> io::Result<()> {
    match args {
        [command, rest @ ..] if command == "list" => {
            let format = OutputFormat::parse_format_argument(rest)?;
            print_media_assets(project_root, format)
        }
        [command, source_path] if command == "upload" => {
            upload_and_print(project_root, Path::new(source_path))
        }
        _ => Err(invalid_input(
            "usage: content media list [--format text|json] | content media upload <local-path>",
        )),
    }
}

fn print_media_assets(project_root: &Path, format: OutputFormat) -> io::Result<()> {
    let assets = list_media_assets(project_root)?;

    match format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&assets).map_err(invalid_data)?
            );
        }
        OutputFormat::Text => {
            for asset in &assets {
                println!(
                    "{} {} ({} bytes)",
                    asset.public_path, asset.name, asset.size_bytes
                );
            }
        }
    }

    Ok(())
}

fn upload_and_print(project_root: &Path, source_path: &Path) -> io::Result<()> {
    let asset = upload_media_asset(project_root, source_path)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&asset).map_err(invalid_data)?
    );
    Ok(())
}
