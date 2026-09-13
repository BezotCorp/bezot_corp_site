use std::io::{self, Read};
use std::path::Path;

use common::{invalid_data, invalid_input};

use crate::post_editor_state::PostEditorState;
use crate::post_reader::load_post_editor;
use crate::post_writer::save_post;

pub fn run_content_post_command(project_root: &Path, args: &[String]) -> io::Result<()> {
    match args {
        [command, post_id] if command == "get" => print_post(project_root, post_id),
        [command] if command == "save" => save_post_from_stdin(project_root),
        _ => Err(invalid_input(
            "usage: content post get <post-id> | content post save",
        )),
    }
}

fn print_post(project_root: &Path, post_id: &str) -> io::Result<()> {
    let editor = load_post_editor(project_root, post_id)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&editor).map_err(invalid_data)?
    );
    Ok(())
}

fn save_post_from_stdin(project_root: &Path) -> io::Result<()> {
    let mut source = String::new();
    io::stdin().read_to_string(&mut source)?;
    let editor = serde_json::from_str::<PostEditorState>(&source).map_err(invalid_data)?;
    save_post(project_root, &editor)
}
