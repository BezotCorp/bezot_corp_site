use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::assembler_runner::run_assembler_with_port;
use crate::command_mode::CommandMode;

/// Fixed rather than left to Vite's default so a caller can build the
/// preview URL deterministically instead of parsing Vite's log output.
/// Distinct from Vite's own default preview port (4173) so an unrelated
/// `vite preview`/`pnpm dev` session on this machine does not collide with
/// it — `--strictPort` (set in bezot_project_assembler) makes a real
/// collision a loud failure instead of a silently wrong URL.
pub const PREVIEW_PORT: u16 = 4174;

const PREVIEW_TEMP_PREFIX: &str = "bezot_project_studio_core_preview_";
const EXCLUDED_DIR_NAMES: &[&str] = &["node_modules", "prebuild", "dist", "dist-ssr"];

/// Creates an isolated copy of `project_root` under the OS temp directory so
/// a draft can be temporarily marked published there without ever touching
/// the real, tracked `content/` — the copy excludes install/build output
/// directories, which the pipeline regenerates on its own.
///
/// Any leftover copy from a previous preview is removed first. A preview
/// server that is killed rather than stopped through the studio therefore
/// leaves at most one stale copy behind, cleaned up the next time a preview
/// starts — not accumulated indefinitely.
pub fn build_preview_root(project_root: &Path) -> io::Result<PathBuf> {
    remove_previous_preview_roots()?;

    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let temp_root = std::env::temp_dir().join(format!(
        "{PREVIEW_TEMP_PREFIX}{}_{stamp}",
        std::process::id()
    ));

    copy_tree(project_root, &temp_root)?;
    Ok(temp_root)
}

/// Starts the same "prepare final site then serve it" pipeline the `dev`
/// command uses. Blocks until the preview server (Vite Preview) is killed.
pub fn run_preview(preview_root: &Path) -> io::Result<()> {
    run_assembler_with_port(preview_root, CommandMode::Dev, Some(PREVIEW_PORT))
}

/// Mirrors `getPathForLocaleAndSlug` in `site/src/application/create-site-runtime.ts`.
pub fn preview_route(locale: &str, slug: &str) -> String {
    let trimmed_slug = slug.trim_matches('/');
    let path = if trimmed_slug.is_empty() {
        format!("/{locale}")
    } else {
        format!("/{locale}/{trimmed_slug}")
    };

    if path.ends_with('/') {
        path
    } else {
        format!("{path}/")
    }
}

fn remove_previous_preview_roots() -> io::Result<()> {
    let temp_dir = std::env::temp_dir();
    let Ok(entries) = fs::read_dir(&temp_dir) else {
        return Ok(());
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        if name.to_string_lossy().starts_with(PREVIEW_TEMP_PREFIX) {
            let _ = fs::remove_dir_all(entry.path());
        }
    }

    Ok(())
}

fn copy_tree(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let file_name = entry.file_name();
        if EXCLUDED_DIR_NAMES
            .iter()
            .any(|excluded| file_name.to_string_lossy() == *excluded)
        {
            continue;
        }

        let source_path = entry.path();
        let destination_path = destination.join(&file_name);
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            copy_tree(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            fs::copy(&source_path, &destination_path)?;
        }
    }

    Ok(())
}
