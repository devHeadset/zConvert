use std::process::Command;
use std::io;

pub fn convert_media(input: &str, output: &str) -> io::Result<()> {
    let status = Command::new("ffmpeg")
        .args(["-i", input, output])
        .status()?;

    if status.success() {
        println!("converted {} → {}", input, output);
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "ffmpeg failed"))
    }
}
