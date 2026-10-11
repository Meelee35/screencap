use std::path::Path;
use chrono::{DateTime, Local};
use image::ImageError;
use crate::capture::pixels::Capture;

pub fn save_capture(dir: &Path, capture: &Capture) -> Result<(), ImageError> {
    let datetime: DateTime<Local> = capture.timestamp.into();
    let filename = datetime.format("Screenshot_%Y%m%d_%H%M%S.png").to_string();
    let save_path = dir.join(filename);
    let result = capture.image.save(save_path.clone());
    if result.is_ok() {
        log!("Saved to '{}'", save_path.display());
    }
    result
}