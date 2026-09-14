use std::{io, path::Path};

use crate::command_mode::CommandMode;
use crate::process_runner::run_command;
use crate::project_paths::assembler_manifest_path;

pub fn run_assembler(project_root: &Path, mode: CommandMode) -> io::Result<()> {
    run_assembler_with_port(project_root, mode, None)
}

/// `port` fixes the port Vite Preview binds to in `dev` mode (ignored for
/// `production`), so a caller that needs a predictable preview URL — the
/// studio's isolated draft preview — doesn't have to parse Vite's log output.
pub fn run_assembler_with_port(
    project_root: &Path,
    mode: CommandMode,
    port: Option<u16>,
) -> io::Result<()> {
    let mut args = vec!["run".to_string()];

    if mode.use_release_assembler() {
        args.push("--release".to_string());
    }

    args.extend([
        "--manifest-path".to_string(),
        assembler_manifest_path().to_string_lossy().into_owned(),
        "--".to_string(),
        project_root.to_string_lossy().into_owned(),
        mode.assembler_command().to_string(),
    ]);

    if let Some(port) = port {
        args.extend(["--port".to_string(), port.to_string()]);
    }

    run_command("cargo", args)
}
