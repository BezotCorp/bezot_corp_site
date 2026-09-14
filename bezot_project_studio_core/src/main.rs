mod assembler_runner;
mod command_mode;
mod content_entry;
mod content_list_command;
mod content_post_command;
mod content_reader;
mod launcher;
mod localized_content_summary;
mod output_format;
mod post_document;
mod post_reader;
mod post_writer;
mod process_runner;
mod project_paths;

#[cfg(test)]
mod tests;

use launcher::run;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
