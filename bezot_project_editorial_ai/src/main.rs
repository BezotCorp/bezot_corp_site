mod content_entry;
mod content_loader;
mod editorial_audit;
mod localized_content_summary;
mod project_paths;

use std::env;
use std::io;
use std::process::ExitCode;

use editorial_audit::audit_editorial_content;
use project_paths::resolve_project_root;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> io::Result<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 3 {
        return Err(invalid_input(
            "usage: bezot_project_editorial_ai <project-root> audit",
        ));
    }

    let project_root = resolve_project_root(&args[1])?;

    match args[2].as_str() {
        "audit" => audit_editorial_content(&project_root),
        value => Err(invalid_input(format!(
            "unknown command \"{value}\"; expected audit"
        ))),
    }
}

fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}
