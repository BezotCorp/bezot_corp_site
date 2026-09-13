use std::{io, path::Path};

use crate::command_mode::CommandMode;
use crate::process_runner::run_command;
use crate::project_paths::assembler_manifest_path;

pub fn run_assembler(project_root: &Path, mode: CommandMode) -> io::Result<()> {
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

    run_command("cargo", args)
}
