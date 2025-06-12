use std::process::{Command, Stdio};
use std::io;
use std::time::Instant;
use crate::utils::get_ext;

pub fn convert_image(input: &str, output: &str) -> io::Result<()> {
    let valid = ["jpg", "jpeg", "png", "bmp", "tiff", "gif", "webp", "avif"];
    let input_ext = get_ext(input).unwrap_or_default();
    let output_ext = get_ext(output).unwrap_or_default();

    if !valid.contains(&input_ext.as_str()) || !valid.contains(&output_ext.as_str()) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "unsupported image format"));
    }

    println!("Converting...");
    let start = Instant::now();

    let status = Command::new("ffmpeg")
        .args(["-y", "-i", input, output])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    let ms = start.elapsed().as_millis();
    if status.success() {
        println!("Done in {} ms", ms);
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "ffmpeg failed"))
    }
}

pub fn convert_media(input: &str, output: &str) -> io::Result<()> {
    let input_ext = get_ext(input).unwrap_or_default();
    let output_ext = get_ext(output).unwrap_or_default();

    let is_audio = |ext: &str| ["mp3", "wav", "flac", "aac", "ogg", "m4a"].contains(&ext);
    let is_video = |ext: &str| ["mp4", "mov", "mkv", "webm", "avi"].contains(&ext);

    println!("Converting...");
    let start = Instant::now();

    let mut cmd = Command::new("ffmpeg");

    if is_video(&input_ext) && is_audio(&output_ext) {
        // video → audio
        cmd.args(["-y", "-i", input, "-vn", output]);
    } else if is_audio(&input_ext) && is_video(&output_ext) {
        // audio → video w/ visualizer or fallback
        cmd.args([
            "-y",
            "-f", "lavfi",
            "-i", "color=black:s=1280x720:r=30",
            "-i", input,
            "-shortest",
            "-c:v", "libx264",
            "-c:a", "aac",
            "-b:a", "192k",
            output,
        ]);
    } else {
        // direct convert
        cmd.args(["-y", "-i", input, output]);
    }

    let status = cmd.stdout(Stdio::null()).stderr(Stdio::null()).status()?;

    let ms = start.elapsed().as_millis();
    if status.success() {
        println!("Done in {} ms", ms);
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "ffmpeg failed"))
    }
}
