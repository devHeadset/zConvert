use std::path::Path;
use std::process::Command;
use std::fs;
use std::io;

pub fn extract_archive(path: &str, out_dir: &str) -> io::Result<()> {
    fs::create_dir_all(out_dir)?;

    let lower = path.to_lowercase();
    let status = if lower.ends_with(".zip") {
        Command::new("unzip")
            .args([path, "-d", out_dir])
            .status()?
    } else if lower.ends_with(".rar") {
        Command::new("unrar")
            .args(["x", path, out_dir])
            .status()?
    } else if lower.ends_with(".tar.gz") || lower.ends_with(".tgz") {
        Command::new("tar")
            .args(["-xzf", path, "-C", out_dir])
            .status()?
    } else if lower.ends_with(".tar") {
        Command::new("tar")
            .args(["-xf", path, "-C", out_dir])
            .status()?
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "unsupported archive"));
    };

    if status.success() {
        println!("extracted {} → {}", path, out_dir);
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "extraction failed"))
    }
}
