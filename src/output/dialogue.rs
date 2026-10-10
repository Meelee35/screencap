use std::process::Command;
use crate::constants;

pub fn error(message: &str) {
    let title = constants::APP_NAME;
    let tools: &[&[&str]] = &[
        &["kdialog", "--title", title, "--error", message][..],
        &["zenity", "--title", title, "--error", "--text", message][..],
    ];
    for tool in tools {
        if Command::new(tool[0]).args(&tool[1..]).spawn().is_ok() {
            return;
        }
    }
    eprintln!("[{title}] {message}");
}
