use crate::constants;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct State {
    pub last_mode: Mode,
}

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub enum Mode {
    Fullscreen,
    #[default]
    Rectangle,
}

pub fn load() -> Result<State, confy::ConfyError> {
    confy::load_path(path())
}

impl State {
    pub fn save(&self) -> Result<(), confy::ConfyError> {
        confy::store_path(path(), self)
    }
}

fn path() -> PathBuf {
    let base = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/state")));
    base.unwrap_or_default()
        .join(constants::APP_ID)
        .join("store.toml")
}
