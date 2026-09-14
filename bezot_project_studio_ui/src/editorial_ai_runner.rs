use std::env;
use std::ffi::OsString;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Output};

use crate::project_paths::editorial_ai_manifest_path;

const EDITORIAL_AI_ENV: &str = "BEZOT_PROJECT_EDITORIAL_AI";
const EDITORIAL_AI_BINARY: &str = "bezot_project_editorial_ai";

pub(crate) fn run_editorial_ai(args: &[OsString]) -> io::Result<Output> {
    editorial_ai_command(args).output()
}

fn editorial_ai_command(args: &[OsString]) -> Command {
    match editorial_ai_executable() {
        Some(executable) => {
            let mut command = Command::new(executable);
            command.args(args);
            command
        }
        None => {
            let mut command = Command::new("cargo");
            command
                .args(["run", "--manifest-path"])
                .arg(editorial_ai_manifest_path())
                .arg("--")
                .args(args);
            command
        }
    }
}

fn editorial_ai_executable() -> Option<PathBuf> {
    explicit_editorial_ai_executable().or_else(sibling_editorial_ai_executable)
}

fn explicit_editorial_ai_executable() -> Option<PathBuf> {
    env::var_os(EDITORIAL_AI_ENV)
        .map(PathBuf::from)
        .filter(|path| path.is_file())
}

fn sibling_editorial_ai_executable() -> Option<PathBuf> {
    env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(PathBuf::from))
        .map(|directory| directory.join(EDITORIAL_AI_BINARY))
        .filter(|path| path.is_file())
}

pub(crate) fn editorial_ai_failure(output: &Output) -> io::Error {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let details = if stderr.is_empty() { stdout } else { stderr };

    io::Error::other(if details.is_empty() {
        "bezot_project_editorial_ai failed without output".to_string()
    } else {
        details
    })
}
