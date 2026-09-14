use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use common::CommandMode;

pub fn execute(mode: CommandMode, project_root: PathBuf, port: Option<u16>) -> io::Result<()> {
    match mode {
        CommandMode::Dev => run_dev(&project_root, port),
        CommandMode::Production => prepare_final_site(&project_root),
    }
}

fn prepare_final_site(project_root: &Path) -> io::Result<()> {
    let prebuild_root = project_root.join("prebuild");
    run_pnpm(&prebuild_root, &["install", "--frozen-lockfile"])?;
    run_pnpm(&prebuild_root, &["exec", "eslint", "."])?;
    run_node(
        &prebuild_root,
        &["scripts/checks/source-boundary-checks.mjs"],
    )?;
    run_pnpm(&prebuild_root, &["exec", "tsc", "-b"])?;
    run_pnpm(&prebuild_root, &["exec", "vite", "build"])?;
    run_pnpm(
        &prebuild_root,
        &[
            "exec",
            "vite",
            "build",
            "--ssr",
            "assembled/entry-server.tsx",
            "--outDir",
            "dist/server",
            "--emptyOutDir",
            "false",
        ],
    )?;
    run_node(&prebuild_root, &["scripts/prerender.mjs"])?;
    fs::remove_dir_all(prebuild_root.join("dist/server")).or_else(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            Ok(())
        } else {
            Err(error)
        }
    })?;
    run_node(
        &prebuild_root,
        &["scripts/checks/production-dist-checks.mjs"],
    )?;
    run_node(
        &prebuild_root,
        &["scripts/checks/production-seo-checks.mjs"],
    )?;
    run_node(
        &prebuild_root,
        &["scripts/checks/html/html-structure-checks.mjs"],
    )?;
    run_node(
        &prebuild_root,
        &["scripts/checks/html/html-accessibility-checks.mjs"],
    )?;
    run_node(
        &prebuild_root,
        &["scripts/checks/html/html-reference-checks.mjs"],
    )
}

fn run_dev(project_root: &Path, port: Option<u16>) -> io::Result<()> {
    prepare_final_site(project_root)?;

    let prebuild_root = project_root.join("prebuild");
    let port_argument = port.map(|port| port.to_string());
    let mut arguments = vec!["exec", "vite", "preview"];
    if let Some(port_argument) = &port_argument {
        // --strictPort makes a taken port a hard failure instead of Vite
        // silently picking another one, which callers that need a
        // predictable URL (the studio's preview feature) rely on.
        arguments.extend(["--port", port_argument, "--strictPort"]);
    }

    run_pnpm(&prebuild_root, &arguments)
}

fn run_pnpm(working_directory: &Path, arguments: &[&str]) -> io::Result<()> {
    run_command(working_directory, "pnpm", arguments)
}

fn run_node(working_directory: &Path, arguments: &[&str]) -> io::Result<()> {
    run_command(working_directory, "node", arguments)
}

fn run_command(working_directory: &Path, program: &str, arguments: &[&str]) -> io::Result<()> {
    let status = Command::new(program)
        .args(arguments)
        .current_dir(working_directory)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        let command = format!("{program} {}", arguments.join(" "));
        let message = match status.code() {
            Some(status) => format!("{command} exited with status {status}"),
            None => format!("{command} was terminated by a signal"),
        };
        Err(io::Error::other(message))
    }
}
