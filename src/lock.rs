use anyhow::Result;
use std::path::Path;
use std::fs;

pub fn acquire(lock_dir: &Path) -> Result<()> {
    fs::create_dir_all(lock_dir)?;
    Ok(())
}

pub fn release(lock_dir: &Path) {
    let _ = fs::remove_dir_all(lock_dir);
}
