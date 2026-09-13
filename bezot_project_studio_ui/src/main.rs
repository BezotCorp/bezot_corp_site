mod content_entry;
mod content_loader;
mod localized_content_summary;
mod message;
mod project_paths;
mod studio_state;
mod studio_view;

#[cfg(test)]
mod tests;

use std::{env, io, path::PathBuf};

use iced::{application, Result, Theme};

use content_loader::load_content_entries;
use message::Message;
use project_paths::resolve_project_root;
use studio_state::StudioState;
use studio_view::view;

fn main() -> Result {
    let state = match boot_state() {
        Ok(state) => state,
        Err(error) => StudioState {
            project_root: PathBuf::from("."),
            entries: Vec::new(),
            error: Some(error.to_string()),
        },
    };

    application(move || state.clone(), update, view)
        .title("Bezot Project Studio")
        .theme(theme)
        .run()
}

fn boot_state() -> io::Result<StudioState> {
    let args = env::args().collect::<Vec<_>>();
    let project_root_argument = project_root_argument(&args)?;
    let project_root = resolve_project_root(project_root_argument)?;
    let entries = load_content_entries(&project_root)?;

    Ok(StudioState {
        project_root,
        entries,
        error: None,
    })
}

fn project_root_argument(args: &[String]) -> io::Result<&str> {
    match args {
        [_program] => Ok("site"),
        [_program, project_root] => Ok(project_root),
        _ => Err(invalid_input(
            "usage: bezot_project_studio_ui [project-root]",
        )),
    }
}

fn update(_state: &mut StudioState, message: Message) {
    match message {}
}

fn theme(_state: &StudioState) -> Theme {
    Theme::Dark
}

fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}
