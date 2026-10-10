use crate::capture::kwin::capture_workspace;
use crate::capture::pixels::Capture;
use std::path::PathBuf;

slint::include_modules!();

pub mod capture;
pub mod ui;
pub mod app;

fn main() -> anyhow::Result<()> {
    let raw_capture = capture_workspace()?;
    let capture = Capture::try_from(raw_capture)?;
    capture.image.save(PathBuf::from("capture.png"))?;
    
    run().expect("TODO: panic message");
    
    Ok(())
}

fn run() -> anyhow::Result<()> {
    let window = MainWindow::new()?;
    window.run()?;
    Ok(())
}