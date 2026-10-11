use crate::constants;
use std::process::Command;

pub fn error(message: &str) {
    err!("{message}");
    let title = constants::APP_NAME;
    let tools: &[&[&str]] = &[
        &["kdialog", "--title", title, "--error", message][..],
        &["zenity", "--title", title, "--error", "--text", message][..],
    ];
    for tool in tools {
        if Command::new(tool[0]).args(&tool[1..]).status().is_ok() {
            return;
        }
    }
}
