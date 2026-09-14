mod content_entry;
mod content_loader;
mod draft_generator;
mod editorial_audit;
mod editorial_review;
mod localized_content_summary;
mod ollama_client;
mod post_bridge;
mod project_paths;

#[cfg(test)]
mod tests;

use std::env;
use std::io;
use std::process::ExitCode;

use common::invalid_data;
use draft_generator::generate_draft;
use editorial_audit::audit_editorial_content;
use editorial_review::review_editorial_content;
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
        return Err(usage_error());
    }

    let project_root = resolve_project_root(&args[1])?;
    let command = args[2].as_str();
    let options = &args[3..];

    match command {
        "audit" => audit_editorial_content(&project_root),
        "draft" => {
            let model = required_option(options, "--model")?;
            let topic = required_option(options, "--topic")?;
            let editor = generate_draft(&project_root, &model, &topic)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&editor).map_err(invalid_data)?
            );
            Ok(())
        }
        "review" => {
            let model = required_option(options, "--model")?;
            let format = optional_option(options, "--format").unwrap_or_else(|| "text".to_string());
            review_editorial_content(&project_root, &model, &format)
        }
        value => Err(invalid_input(format!(
            "unknown command \"{value}\"; expected audit, draft, or review"
        ))),
    }
}

fn required_option(options: &[String], name: &str) -> io::Result<String> {
    optional_option(options, name)
        .ok_or_else(|| invalid_input(format!("missing required option {name}")))
}

fn optional_option(options: &[String], name: &str) -> Option<String> {
    options
        .iter()
        .position(|value| value == name)
        .and_then(|index| options.get(index + 1))
        .cloned()
}

fn usage_error() -> io::Error {
    invalid_input(
        "usage: bezot_project_editorial_ai <project-root> audit \
| draft --model <name> --topic <topic> \
| review --model <name>",
    )
}

fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}
