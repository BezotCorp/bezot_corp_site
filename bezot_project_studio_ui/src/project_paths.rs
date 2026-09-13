use std::{env, io, path::PathBuf};

pub(crate) fn resolve_project_root(input: &str) -> io::Result<PathBuf> {
    let given = PathBuf::from(input);
    if given.exists() {
        return Ok(given);
    }

    let candidate = repository_root().join(&given);
    if candidate.exists() {
        return Ok(candidate);
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!(
            "could not find project root '{input}' relative to the current directory or repository root"
        ),
    ))
}

pub(crate) fn studio_core_manifest_path() -> PathBuf {
    repository_root().join("bezot_project_studio_core/Cargo.toml")
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("studio UI crate must live directly under repository root")
        .to_path_buf()
}
