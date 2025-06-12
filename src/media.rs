use std::process::{Command, Stdio};
use std::io;
use std::time::Instant;

pub fn convert_media(input: &str, output: &str) -> io::Result<()> {
    println!("Converting...");

    let start = Instant::now();

    let status = Command::new("ffmpeg")
        .args(["-i", input, output])
        // hide all stdout and stderr from ffmpeg
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    let duration = start.elapsed().as_millis();

    if status.success() {
        println!("Done in {} ms", duration);
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "ffmpeg failed"))
    }
}
