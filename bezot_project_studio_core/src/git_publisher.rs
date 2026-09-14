use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use common::invalid_input;

/// Publishes a content change: fetches the real `origin/main`, creates an
/// isolated worktree branched from it, copies in only the content files git
/// reports as changed under `content/` (refusing if anything else in the
/// repository is dirty), commits, pushes, opens a Pull Request into `main`,
/// and merges it. Never checks out, stashes, or otherwise touches the
/// working tree or branch this process is running from — so code in
/// progress there can never be swept into `main` through this path.
pub fn publish_content_change(
    project_root: &Path,
    branch_slug: &str,
    summary: &str,
) -> io::Result<()> {
    let project_root = project_root.canonicalize()?;
    let repo_root = discover_repo_root(&project_root)?;
    let content_root_relative = project_root
        .strip_prefix(&repo_root)
        .map_err(|_| invalid_input("le dossier du site est en dehors du dépôt git"))?
        .join("content");

    let changed_files = discover_changed_content_files(&repo_root, &content_root_relative)?;

    run_git(&repo_root, &["fetch", "origin", "main"])?;

    let branch_name = format!(
        "content/{}-{}",
        sanitize_branch_slug(branch_slug),
        unique_suffix()
    );
    let worktree_path = repo_root.join(".git-publish-worktrees").join(&branch_name);
    let _ = fs::remove_dir_all(&worktree_path);

    run_git(
        &repo_root,
        &[
            "worktree",
            "add",
            "-b",
            &branch_name,
            path_str(&worktree_path)?,
            "origin/main",
        ],
    )?;

    let result = publish_from_worktree(
        &repo_root,
        &worktree_path,
        &branch_name,
        &changed_files,
        summary,
    );

    let _ = Command::new("git")
        .args(["worktree", "remove", "--force"])
        .arg(&worktree_path)
        .current_dir(&repo_root)
        .status();

    result
}

fn publish_from_worktree(
    repo_root: &Path,
    worktree_path: &Path,
    branch_name: &str,
    changed_files: &[PathBuf],
    summary: &str,
) -> io::Result<()> {
    for relative_path in changed_files {
        let source = repo_root.join(relative_path);
        let destination = worktree_path.join(relative_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&source, &destination)?;
    }

    run_git(worktree_path, &["add", "--", "."])?;
    run_git(worktree_path, &["commit", "-m", summary])?;
    run_git(worktree_path, &["push", "-u", "origin", branch_name])?;
    run_gh(
        worktree_path,
        &[
            "pr",
            "create",
            "--base",
            "main",
            "--head",
            branch_name,
            "--title",
            summary,
            "--body",
            "",
        ],
    )?;
    run_gh(
        worktree_path,
        &["pr", "merge", branch_name, "--merge", "--delete-branch"],
    )
}

/// Reads `git status --porcelain` for the whole repository and returns the
/// changed paths under `content/`, relative to the repo root — or an error
/// if anything outside `content/` is dirty, so an in-progress code change
/// can never be committed through the content-publish path.
fn discover_changed_content_files(
    repo_root: &Path,
    content_root_relative: &Path,
) -> io::Result<Vec<PathBuf>> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(repo_root)
        .output()?;

    if !output.status.success() {
        return Err(command_failure("git status", &output.stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut files = Vec::new();

    for line in stdout.lines() {
        let path_part = line.get(3..).unwrap_or("").trim();
        let relative_path = PathBuf::from(path_part.rsplit(" -> ").next().unwrap_or(path_part));

        if !relative_path.starts_with(content_root_relative) {
            return Err(invalid_input(format!(
                "publication refusée : une modification hors de content/ a été détectée ({})",
                relative_path.display()
            )));
        }

        files.push(relative_path);
    }

    if files.is_empty() {
        return Err(invalid_input("aucune modification de contenu à publier"));
    }

    Ok(files)
}

fn discover_repo_root(project_root: &Path) -> io::Result<PathBuf> {
    let output = Command::new("git")
        .args(["-C"])
        .arg(project_root)
        .args(["rev-parse", "--show-toplevel"])
        .output()?;

    if !output.status.success() {
        return Err(command_failure(
            "git rev-parse --show-toplevel",
            &output.stderr,
        ));
    }

    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim().to_string(),
    ))
}

pub(crate) fn sanitize_branch_slug(slug: &str) -> String {
    let mut sanitized = String::with_capacity(slug.len());
    let mut previous_was_dash = false;

    for character in slug.to_lowercase().chars() {
        if character.is_ascii_alphanumeric() {
            sanitized.push(character);
            previous_was_dash = false;
        } else if !previous_was_dash {
            sanitized.push('-');
            previous_was_dash = true;
        }
    }

    sanitized.trim_matches('-').to_string()
}

fn unique_suffix() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

fn path_str(path: &Path) -> io::Result<&str> {
    path.to_str()
        .ok_or_else(|| invalid_input("chemin non-UTF-8"))
}

fn run_git(working_directory: &Path, args: &[&str]) -> io::Result<()> {
    run_external(working_directory, "git", args)
}

fn run_gh(working_directory: &Path, args: &[&str]) -> io::Result<()> {
    run_external(working_directory, "gh", args)
}

fn run_external(working_directory: &Path, program: &str, args: &[&str]) -> io::Result<()> {
    let output = Command::new(program)
        .args(args)
        .current_dir(working_directory)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(command_failure(
            &format!("{program} {}", args.join(" ")),
            &output.stderr,
        ))
    }
}

fn command_failure(command: &str, stderr: &[u8]) -> io::Error {
    let message = String::from_utf8_lossy(stderr).trim().to_string();
    io::Error::other(if message.is_empty() {
        format!("{command} a échoué")
    } else {
        format!("{command} a échoué : {message}")
    })
}
