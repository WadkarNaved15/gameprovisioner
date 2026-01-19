use anyhow::{Result, Context};
use std::path::Path;
use std::fs::File;
use std::io;
use zip::ZipArchive;
use std::process::Command;

pub fn zip(archive: &Path, target: &Path) -> Result<()> {
    let file = File::open(archive)?;
    let mut zip = ZipArchive::new(file)?;

    for i in 0..zip.len() {
        let mut f = zip.by_index(i)?;
        let out = target.join(f.name());

        if f.is_dir() {
            std::fs::create_dir_all(&out)?;
        } else {
            if let Some(p) = out.parent() {
                std::fs::create_dir_all(p)?;
            }
            let mut out_file = File::create(out)?;
            io::copy(&mut f, &mut out_file)?;
        }
    }
    Ok(())
}

pub fn seven_zip(archive: &Path, target: &Path) -> Result<()> {
    let status = Command::new(r"C:\Program Files\7-Zip\7z.exe")
        .args([
            "x",
            archive.to_str().unwrap(),
            "-y",
            &format!("-o{}", target.display()),
        ])
        .status()
        .context("Failed to run 7z")?;

    if !status.success() {
        anyhow::bail!("7z extraction failed");
    }

    Ok(())
}
