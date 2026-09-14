use std::{
    io::Write,
    net::TcpStream,
    os::unix::process::CommandExt,
    path::Path,
    process::Stdio,
    time::{Duration, Instant},
};

use common::{PageEditorState, PostEditorState};

use crate::studio_core_runner::studio_core_process;

/// Must match `bezot_project_studio_core::preview_runner::PREVIEW_PORT`.
/// Duplicated rather than shared: it only crosses the subprocess boundary
/// as a number baked into the preview URL, the same way other wire shapes
/// are duplicated per crate instead of centralized in `common`.
pub(crate) const PREVIEW_PORT: u16 = 4174;

#[derive(Debug, Clone)]
pub(crate) struct PreviewSession {
    pub(crate) pid: u32,
    pub(crate) url_fr: String,
    pub(crate) url_en: String,
}

/// Mirrors `bezot_project_studio_core::preview_runner::preview_route`.
fn preview_route(locale: &str, slug: &str) -> String {
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

pub(crate) fn start_post_preview(
    project_root: &Path,
    editor: &PostEditorState,
) -> std::io::Result<PreviewSession> {
    let source = serde_json::to_vec(editor)?;
    let pid = spawn_preview(
        project_root,
        &["content".into(), "post".into(), "preview".into()],
        &source,
    )?;

    wait_until_ready(pid)?;

    Ok(PreviewSession {
        pid,
        url_fr: format!(
            "http://localhost:{PREVIEW_PORT}{}",
            preview_route("fr-fr", &editor.fr.slug)
        ),
        url_en: format!(
            "http://localhost:{PREVIEW_PORT}{}",
            preview_route("en-us", &editor.en.slug)
        ),
    })
}

pub(crate) fn start_page_preview(
    project_root: &Path,
    editor: &PageEditorState,
) -> std::io::Result<PreviewSession> {
    let source = serde_json::to_vec(editor)?;
    let pid = spawn_preview(
        project_root,
        &["content".into(), "page".into(), "preview".into()],
        &source,
    )?;

    wait_until_ready(pid)?;

    Ok(PreviewSession {
        pid,
        url_fr: format!(
            "http://localhost:{PREVIEW_PORT}{}",
            preview_route("fr-fr", &editor.fr.slug)
        ),
        url_en: format!(
            "http://localhost:{PREVIEW_PORT}{}",
            preview_route("en-us", &editor.en.slug)
        ),
    })
}

/// Kills the whole process tree the preview spawned (studio_core, the
/// assembler, pnpm, Vite): the child is started in its own process group
/// (`process_group(0)`, so its pgid equals its own pid), and a negative pid
/// targets that whole group rather than just the immediate child. Uses the
/// `kill(2)` syscall directly rather than shelling out, so this can't be
/// derailed by `PATH` resolution or subprocess spawn overhead.
pub(crate) fn stop_preview(pid: u32) {
    unsafe {
        libc::kill(-(pid as libc::pid_t), libc::SIGTERM);
    }
}

fn spawn_preview(
    project_root: &Path,
    args: &[std::ffi::OsString],
    stdin_source: &[u8],
) -> std::io::Result<u32> {
    let mut command = studio_core_process(project_root, args);
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .process_group(0);

    let mut child = command.spawn()?;
    let pid = child.id();
    child
        .stdin
        .take()
        .ok_or_else(|| std::io::Error::other("could not open preview process stdin"))?
        .write_all(stdin_source)?;

    Ok(pid)
}

/// The preview reruns the full production pipeline (install, eslint, tsc,
/// two Vite builds, prerender, four check scripts) before Vite Preview binds
/// the port — measured at ~15-30s with a warm cache, but slower under CPU
/// contention, so the ceiling here is generous rather than tight.
const READY_TIMEOUT: Duration = Duration::from_secs(300);

fn wait_until_ready(pid: u32) -> std::io::Result<()> {
    let deadline = Instant::now() + READY_TIMEOUT;

    while Instant::now() < deadline {
        if TcpStream::connect(("127.0.0.1", PREVIEW_PORT)).is_ok() {
            return Ok(());
        }
        if !process_is_alive(pid) {
            return Err(std::io::Error::other(
                "le processus d’aperçu s’est arrêté avant d’être prêt (voir les vérifications du site)",
            ));
        }
        std::thread::sleep(Duration::from_millis(300));
    }

    stop_preview(pid);
    Err(std::io::Error::other(
        "l’aperçu n’a pas démarré à temps (délai de 300 s dépassé)",
    ))
}

fn process_is_alive(pid: u32) -> bool {
    unsafe { libc::kill(pid as libc::pid_t, 0) == 0 }
}
