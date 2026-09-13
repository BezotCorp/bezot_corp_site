use anyhow::Result;
use std::fs;

pub(crate) fn run(path: String) -> Result<()> {
    let repo = gix::discover(".")?;

    let data = fs::read(&path)?;

    let id = repo.write_blob(data)?;

    println!("object written: {}", id);

    Ok(())
}
