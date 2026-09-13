use std::env;
use std::ffi::OsString;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

use crate::project_paths::studio_core_manifest_path;

const STUDIO_CORE_ENV: &str = "BEZOT_PROJECT_STUDIO_CORE";
const STUDIO_CORE_BINARY: &str = "bezot_project_studio_core";

pub(crate) fn run_studio_core(args: &[OsString]) -> io::Result<Output> {
    studio_core_command(args).output()
}

pub(crate) fn run_studio_core_with_stdin(
    args: &[OsString],
    stdin_source: &[u8],
) -> io::Result<Output> {
    let mut child = studio_core_command(args)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("could not open studio_core stdin"))?;
    stdin.write_all(stdin_source)?;
    drop(stdin);

    child.wait_with_output()
}

fn studio_core_command(args: &[OsString]) -> Command {
    match studio_core_executable() {
        Some(executable) => {
            let mut command = Command::new(executable);
            command.args(args);
            command
        }
        None => {
            let mut command = Command::new("cargo");
            command
                .args(["run", "--manifest-path"])
                .arg(studio_core_manifest_path())
                .arg("--")
                .args(args);
            command
        }
    }
}

fn studio_core_executable() -> Option<PathBuf> {
    explicit_studio_core_executable().or_else(sibling_studio_core_executable)
}

fn explicit_studio_core_executable() -> Option<PathBuf> {
    env::var_os(STUDIO_CORE_ENV)
        .map(PathBuf::from)
        .filter(|path| path.is_file())
}

fn sibling_studio_core_executable() -> Option<PathBuf> {
    env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(PathBuf::from))
        .map(|directory| directory.join(STUDIO_CORE_BINARY))
        .filter(|path| path.is_file())
}

pub(crate) fn studio_core_failure(output: &Output) -> io::Error {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let details = if stderr.is_empty() { stdout } else { stderr };

    io::Error::other(if details.is_empty() {
        "studio_core failed without output".to_string()
    } else {
        details
    })
}
