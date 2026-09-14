use std::path::PathBuf;

pub fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("studio UI crate must live directly under repository root")
        .to_path_buf()
}
