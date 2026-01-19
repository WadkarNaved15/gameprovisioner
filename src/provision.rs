use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;

use crate::{download, extract, lock};

const GAME_ROOT: &str = "C:\\games";

pub async fn ensure_game_ready(
    game_id: &str,
    build_id: &str,
    s3_url: &str,
    format: &str, // "7z" | "zip" | "exe"
) -> Result<PathBuf> {

    let build_dir = Path::new(GAME_ROOT).join(game_id).join(build_id);
    let ready_file = build_dir.join("READY");
    let lock_dir = build_dir.join(".lock");

    if ready_file.exists() {
        return Ok(build_dir);
    }

    lock::acquire(&lock_dir)?;

    let result = provision_internal(game_id, build_id, s3_url, format, &build_dir).await;

    lock::release(&lock_dir);
    result?;

    Ok(build_dir)
}

async fn provision_internal(
    game_id: &str,
    build_id: &str,
    s3_url: &str,
    format: &str,
    build_dir: &Path,
) -> Result<()> {

    fs::create_dir_all(build_dir)?;

    let tmp_file = std::env::temp_dir()
        .join(format!("{}_{}.{}", game_id, build_id, format));

    download::from_s3(s3_url, &tmp_file).await?;

    let game_dir = build_dir.join("game");
    fs::create_dir_all(&game_dir)?;

    match format {
        "exe" => {
            let exe_path = game_dir.join(tmp_file.file_name().unwrap());
            fs::rename(&tmp_file, &exe_path)?;
        }
        "zip" => {
            extract::zip(&tmp_file, &game_dir)?;
            fs::remove_file(&tmp_file).ok();
        }
        "7z" => {
            extract::seven_zip(&tmp_file, &game_dir)?;
            fs::remove_file(&tmp_file).ok();
        }
        _ => anyhow::bail!("Unsupported format"),
    }

    fs::write(build_dir.join("READY"), b"")?;

    Ok(())
}
