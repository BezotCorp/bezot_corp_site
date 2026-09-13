mod assembler_runner;
mod command_mode;
mod content_entry;
mod content_list_command;
mod content_reader;
mod errors;
mod localized_content_summary;
mod output_format;
mod process_runner;
mod project_paths;

use std::env;
use std::io;
use std::process::ExitCode;

use assembler_runner::run_assembler;
use command_mode::CommandMode;
use content_list_command::run_content_command;
use errors::invalid_input;
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

    if args.len() < 3 {
        return Err(invalid_input(
            "usage: bezot_project_studio_core <project-root> <dev|production|content list [--format text|json]>",
        ));
    }

    let project_root = resolve_project_root(&args[1])?;

    match args[2].as_str() {
        "dev" | "production" => {
            if args.len() != 3 {
                return Err(invalid_input(
                    "dev and production do not accept extra arguments",
                ));
            }

            let mode =
                CommandMode::parse(args.get(2).map(String::as_str)).map_err(invalid_input)?;
            run_assembler(&project_root, mode)
        }
        "content" => run_content_command(&project_root, &args[3..]),
        value => Err(invalid_input(format!(
            "unknown command \"{value}\"; expected dev, production, or content"
        ))),
    }
}
