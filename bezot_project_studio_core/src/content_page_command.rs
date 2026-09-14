use std::io::{self, Read};
use std::path::Path;

use common::{PageEditorState, invalid_data, invalid_input};

use crate::page_deleter::delete_page;
use crate::page_reader::load_page_editor;
use crate::page_writer::save_page;

pub fn run_content_page_command(project_root: &Path, args: &[String]) -> io::Result<()> {
    match args {
        [command, page_id] if command == "get" => print_page(project_root, page_id),
        [command] if command == "save" => save_page_from_stdin(project_root),
        [command, page_id] if command == "delete" => delete_page(project_root, page_id),
        _ => Err(invalid_input(
            "usage: content page get <page-id> | content page save | content page delete <page-id>",
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
    save_page(project_root, &editor)
}
