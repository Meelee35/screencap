use std::io::Write;
use std::process::{Command, Stdio};
use anyhow::Context;
use crate::capture::pixels::Capture;

pub fn copy_capture(capture: &Capture) -> anyhow::Result<()> {
    let mut png = Vec::new();
    capture.image.write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)?;

    let mut child = Command::new("wl-copy")
        .args(["--type", "image/png"])
        .stdin(Stdio::piped())
        .spawn()
        .context("wl-copy is not installed")?;

    child.stdin.take().unwrap().write_all(&png)?;
    log!("Copied to clipboard");
    Ok(())
}
