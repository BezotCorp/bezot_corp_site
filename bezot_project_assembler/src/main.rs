mod assembler_state;
mod assembly;
mod asserts;
mod block_definition;
mod content_reader;
mod content_validator;
mod file_hash;
mod input_file;
mod input_role;
mod output_file;
mod prebuild_writer;
mod project_config;
mod project_execution;
mod project_file;
mod project_snapshot;
mod site_generator;
mod state_diff;
mod state_store;
mod string_operations;

use common::CommandMode;
use std::{env, fmt, io, path::PathBuf, process};

use project_execution::execute;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("usage: bezot_project_assembler <project-root> <dev|production> [--port <port>]");
        process::exit(1);
    }
    let mode = match CommandMode::parse(args.get(2).map(String::as_str)) {
        Ok(mode) => mode,
        Err(error) => exit_with_error(error),
    };
    let port = match parse_port_argument(&args[3..]) {
        Ok(port) => port,
        Err(error) => exit_with_error(error),
    };

    let project_root = match resolve_project_root(&args[1]) {
        Ok(path) => path,
        Err(error) => exit_with_error(error),
    };

    let report = match assembly::AssemblyReport::assemble_project(&project_root) {
        Ok(report) => report,
        Err(error) => exit_with_error(error),
    };
    report.print_summary();

    if let Err(error) = execute(mode, report.project_root().to_path_buf(), port) {
        exit_with_error(error);
    }
}

/// Parses an optional `--port <port>` trailing argument, only meaningful for
/// `dev` (it fixes the port Vite Preview binds to, so a caller that needs a
/// predictable preview URL doesn't have to parse Vite's own log output).
fn parse_port_argument(args: &[String]) -> Result<Option<u16>, String> {
    match args {
        [] => Ok(None),
        [flag, value] if flag == "--port" => value
            .parse::<u16>()
            .map(Some)
            .map_err(|_| format!("invalid --port value \"{value}\"")),
        _ => Err("usage: <project-root> <dev|production> [--port <port>]".to_string()),
    }
}

/// Resolves `input` against the current directory, then, if that doesn't
/// exist, against the assembler's own workspace root (the parent of
/// `CARGO_MANIFEST_DIR`, fixed at build time). This lets the tool be invoked
/// from anywhere in the workspace without requiring the caller to know the
/// exact relative path, without relying on git or any other marker file.
fn resolve_project_root(input: &str) -> io::Result<PathBuf> {
    let given = PathBuf::from(input);
    if given.exists() {
        return Ok(given);
    }

    if let Some(workspace_root) = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent() {
        let candidate = workspace_root.join(&given);
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!(
            "could not find project root '{input}' relative to the current directory or the workspace root"
        ),
    ))
}

fn exit_with_error(error: impl fmt::Display) -> ! {
    eprintln!("error: {error}");
    process::exit(1);
}
