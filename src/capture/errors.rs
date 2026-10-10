use thiserror::Error;

#[derive(Debug, Error)]
pub enum CaptureError {
    #[error("DBus error: {0}")]
    DBus(#[from] zbus::Error),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Bad Metadata: {0}")]
    BadMetadata(String),
}

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(u32),
    #[error("Buffer error: Width:{width} Height:{height}")]
    BadBuffer { width: u32, height: u32 },
}