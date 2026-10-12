use crate::capture::pixels::Capture;
use anyhow::Context;
use chrono::{DateTime, Local};
use std::path::Path;

pub fn save_capture(dir: &Path, capture: &Capture) -> anyhow::Result<()> {
    if !dir.exists() {
        anyhow::bail!("Save directory '{}' does not exist", dir.display());
    }
    if !dir.is_dir() {
        anyhow::bail!("Save path '{}' is not a directory", dir.display());
    }

    let datetime: DateTime<Local> = capture.timestamp.into();
    let filename = datetime.format("Screenshot_%Y%m%d_%H%M%S.png").to_string();
    let save_path = dir.join(filename);
    capture
        .image
        .save(&save_path)
        .with_context(|| format!("Failed to save to '{}'", save_path.display()))?;
    log!("Saved to '{}'", save_path.display());
    Ok(())
}
