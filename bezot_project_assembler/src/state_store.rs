use std::{
    fs, io,
    path::{Path, PathBuf},
};

use bincode_next::config;
use ron::ser::PrettyConfig;

use crate::assembler_state::AssemblerState;

pub fn load_state(project_root: &Path) -> io::Result<Option<AssemblerState>> {
    let bin_path = state_bin_path(project_root);
    let ron_path = state_ron_path(project_root);

    if bin_path.exists() {
        let bytes = fs::read(&bin_path)?;
        let (state, _bytes_read): (AssemblerState, usize) =
            bincode_next::serde::decode_from_slice(&bytes, config::standard())
                .map_err(invalid_data)?;
        return Ok(Some(state));
    }

    if ron_path.exists() {
        let text = fs::read_to_string(&ron_path)?;
        let state = ron::from_str::<AssemblerState>(&text).map_err(invalid_data)?;
        return Ok(Some(state));
    }

    Ok(None)
}

pub fn store_state(state: &AssemblerState) -> io::Result<()> {
    let dir = state_dir(&state.project_root);
    fs::create_dir_all(&dir)?;

    let ron_path = state_ron_path(&state.project_root);
    let bin_path = state_bin_path(&state.project_root);

    let pretty = PrettyConfig::new()
        .depth_limit(8)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let ron_text = ron::ser::to_string_pretty(state, pretty).map_err(invalid_data)?;

    let bin_bytes =
        bincode_next::serde::encode_to_vec(state, config::standard()).map_err(invalid_data)?;

    write_if_changed(&ron_path, ron_text.as_bytes())?;
    write_if_changed(&bin_path, &bin_bytes)?;

    Ok(())
}

fn state_dir(project_root: &Path) -> PathBuf {
    project_root.join("prebuild").join(".assembler")
}

fn state_ron_path(project_root: &Path) -> PathBuf {
    state_dir(project_root).join("state.ron")
}

fn state_bin_path(project_root: &Path) -> PathBuf {
    state_dir(project_root).join("state.bin")
}

fn write_if_changed(path: &Path, bytes: &[u8]) -> io::Result<()> {
    match fs::read(path) {
        Ok(existing) if existing == bytes => Ok(()),
        _ => fs::write(path, bytes),
    }
}

fn invalid_data<E>(error: E) -> io::Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    io::Error::new(io::ErrorKind::InvalidData, error)
}
