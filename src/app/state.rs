use crate::capture::pixels::Capture;

pub struct AppState {
    pub capture: Capture,
    pub mode: CaptureMode,
}

pub enum CaptureMode {
    Fullscreen,
    Rectangle(Rectangle),
}

pub struct Rectangle {
    pub selection: Option<Rect>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
