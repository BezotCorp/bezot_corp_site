mod assembler_runner;
mod command_mode;
mod content_entry;
mod content_list_command;
mod content_page_command;
mod content_post_command;
mod content_reader;
mod gone_route_writer;
mod launcher;
mod localized_content_summary;
mod output_format;
mod page_deleter;
mod page_document;
mod page_reader;
mod page_writer;
mod post_deleter;
mod post_document;
mod post_reader;
mod post_writer;
mod process_runner;
mod project_paths;
mod redirect_writer;

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
