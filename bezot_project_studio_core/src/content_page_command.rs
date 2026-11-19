use std::io::{self, Read};
use std::path::Path;

use common::{PageEditorState, invalid_data, invalid_input};

use crate::page_deleter::delete_page;
use crate::page_reader::load_page_editor;
use crate::page_writer::save_page;
use crate::preview_runner::{build_preview_root, preview_route, run_preview};

pub fn run_content_page_command(project_root: &Path, args: &[String]) -> io::Result<()> {
    match args {
        [command, page_id] if command == "get" => print_page(project_root, page_id),
        [command] if command == "save" => save_page_from_stdin(project_root),
        [command, page_id] if command == "delete" => delete_page(project_root, page_id),
        [command] if command == "preview" => preview_page_from_stdin(project_root),
        _ => Err(invalid_input(
            "usage: content page get <page-id> | content page save | content page delete <page-id> | content page preview",
        )),
    }
}

fn print_page(project_root: &Path, page_id: &str) -> io::Result<()> {
    let editor = load_page_editor(project_root, page_id)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&editor).map_err(invalid_data)?
    );
    Ok(())
}

fn save_page_from_stdin(project_root: &Path) -> io::Result<()> {
    let mut source = String::new();
    io::stdin().read_to_string(&mut source)?;
    let editor = serde_json::from_str::<PageEditorState>(&source).map_err(invalid_data)?;
    save_page(project_root, &editor)?;

    if editor.fr.status == "published" || editor.en.status == "published" {
        let summary = format!("chore: publish \"{}\"", editor.fr.title);
        crate::git_publisher::publish_content_change(project_root, &editor.id, &summary)?;
    }

    Ok(())
}

/// Same isolation as `content post preview`: an isolated copy, both locales
/// forced to "published", then the real production pipeline and preview
/// server — never touching the real, tracked content.
fn preview_page_from_stdin(project_root: &Path) -> io::Result<()> {
    let mut source = String::new();
    io::stdin().read_to_string(&mut source)?;
    let mut editor = serde_json::from_str::<PageEditorState>(&source).map_err(invalid_data)?;
    editor.fr.status = "published".to_string();
    editor.en.status = "published".to_string();

    let preview_root = build_preview_root(project_root)?;
    save_page(&preview_root, &editor)?;

    println!(
        "PREVIEW_ROUTE_FR: {}",
        preview_route("fr-fr", &editor.fr.slug)
    );
    println!(
        "PREVIEW_ROUTE_EN: {}",
        preview_route("en-us", &editor.en.slug)
    );

    run_preview(&preview_root)
}
