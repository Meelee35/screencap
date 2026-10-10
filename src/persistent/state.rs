use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct State {
    pub save_directory: Option<String>,
    pub show_notification: bool,
}


fn load() -> Result<State, confy::ConfyError> {
    confy::load("io.github.meelees.screencap", Some("store"))
}

impl State {
    pub fn save(&mut self) -> Result<(), confy::ConfyError> {
        confy::store("io.github.meelees.screencap", None, Some("store"))
    }
}