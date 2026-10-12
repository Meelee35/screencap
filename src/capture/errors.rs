use thiserror::Error;

#[derive(Debug, Error)]
pub enum CaptureError {
    #[error("DBus error: {0}")]
    DBus(#[from] zbus::Error),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Bad Metadata: {0}")]
    BadMetadata(String),
    #[error(transparent)]
    Conversion(#[from] ConversionError),
    #[error("Not authorised")]
    NotAuthorised,
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Screenshot cancelled")]
    Cancelled,
    #[error("Portal denied the screenshot request")]
    PortalDenied,
}

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(u32),
    #[error("Buffer error: Width:{width} Height:{height}")]
    BadBuffer { width: u32, height: u32 },
}
