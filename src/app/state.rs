use std::path::PathBuf;
use crate::capture::pixels::Capture;
use crate::config::Config;
use crate::output;
use crate::persistent::state::{Mode, State};
use crate::ui::main_window::ViewHandle;

#[derive(Default)]
pub struct AppState {
    pub capture: Capture,
    pub mode: CaptureMode,
    pub view: Option<ViewHandle>,
    pub requested_action: Option<Vec<SaveType>>
}

#[derive(Debug)]
pub enum CaptureMode {
    Fullscreen,
    Rectangle(Rectangle),
}

#[derive(Default, Debug)]
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

pub enum SaveType {
    Clipboard,
    File(PathBuf),
}

pub struct Saved(pub Vec<SaveType>);

impl From<&SaveType> for String {
    fn from(save_type: &SaveType) -> Self {
        match save_type {
            SaveType::Clipboard => String::from("Copied to clipboard"),
            SaveType::File(path) => format!("Saved to {}", path.to_string_lossy()),
        }
    }
}

impl Saved {
    pub fn build_summary(&self) -> String {
        let types = &self.0;
        if types.is_empty() {
            return "".to_owned();
        }
        
        types
            .iter()
            .map(|savetype|savetype.into())
            .collect::<Vec<String>>()
            .join(" and ")
    }
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
            mode: previous_state.last_mode.into(),
            ..Self::default()
        }
    }

    pub fn close(&self) {
        if let Some(view) = &self.view {
            view.close();
        }
    }

    pub fn capture_fullscreen(&self, config: &Config) -> anyhow::Result<Saved> {
        let mut save_types = vec![];
        if let Some(save_dir) = config.save_directory.clone() {
            output::save::save_capture(&save_dir, &self.capture)?;
            save_types.push(SaveType::File(save_dir));
        }
        if config.clipboard {
            output::clipboard::copy_capture(&self.capture)?;
            save_types.push(SaveType::Clipboard);
        }

        Ok(Saved(save_types))
    }
}