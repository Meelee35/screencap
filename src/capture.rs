mod errors;
pub mod kwin;
pub mod pixels;
mod portal;

pub use crate::capture::errors::CaptureError;
use crate::capture::pixels::Capture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureMethod {
    Direct,
    Portal,
}

pub struct Captured {
    pub capture: Capture,
    pub method: CaptureMethod,
}

pub fn capture() -> Result<Captured, CaptureError> {
    match capture_kwin() {
        Err(CaptureError::NotAuthorised) => {
            warn!("Screen capture via KWin was not authorised, using portal.");
            log!("Make sure 'X-KDE-DBUS-Restricted-Interfaces=org.kde.KWin.ScreenShot2' is in the .desktop file");
            capture_portal()
        }
        result => result,
    }
}

fn capture_kwin() -> Result<Captured, CaptureError> {
    let raw = kwin::capture_workspace()?;
    Ok(Captured {
        capture: Capture::try_from(raw)?,
        method: CaptureMethod::Direct,
    })
}

fn capture_portal() -> Result<Captured, CaptureError> {
    let path = portal::capture_file()?;
    log!("Portal saved capture at {}", path.display());

    let image = image::open(&path)?.into_rgba8();
    let _ = std::fs::remove_file(&path);

    Ok(Captured {
        capture: Capture {
            image,
            scale: 1.0, // i've been told to comment on these magic numbers. this one is here because the portal does not return 
            timestamp: std::time::SystemTime::now(),
        },
        method: CaptureMethod::Portal,
    })
}