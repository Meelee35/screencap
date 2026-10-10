use crate::capture::pixels::Capture;
use crate::persistent::state::{Mode, State};

#[derive(Default)]
pub struct AppState {
    pub capture: Capture,
    pub mode: CaptureMode,
}

pub enum CaptureMode {
    Fullscreen,
    Rectangle(Rectangle),
}

#[derive(Default)]
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

impl Default for CaptureMode {
    fn default() -> Self {
        Self::Rectangle(Rectangle::default())
    }
}

impl From<Mode> for CaptureMode {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Fullscreen => Self::Fullscreen,
            Mode::Rectangle => Self::Rectangle(Rectangle::default()),
        }
    }
}

impl AppState {
    pub fn new(previous_state: &State) -> Self {
        Self {
            mode: previous_state.last_mode.clone().into(),
            ..Self::default()
        }
    }
}