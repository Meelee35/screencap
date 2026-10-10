mod errors;
pub mod kwin;
pub mod pixels;

use crate::capture::errors::CaptureError;
use crate::capture::pixels::Capture;

pub fn capture() -> Result<Capture, CaptureError> {
    kwin::capture()
}