use crate::constants;
use notify_rust::{Hint, Notification};
use std::sync::{Mutex, OnceLock};
use std::thread::JoinHandle;

static PENDING: OnceLock<Mutex<Vec<JoinHandle<()>>>> = OnceLock::new();

pub fn warning(message: &str) {
    let mut notification = Notification::new();
    notification
        .appname(constants::APP_NAME)
        .summary(constants::APP_NAME)
        .icon("dialog-warning")
        .body(message);
    show(notification);
}

pub fn success(message: &str) {
    let mut notification = Notification::new();
    notification
        .appname(constants::APP_NAME)
        .summary(constants::APP_NAME)
        .body(message);
    show(notification);
}

pub fn wait() {
    let Some(pending) = PENDING.get() else {
        return;
    };
    for handle in pending.lock().unwrap().drain(..) {
        let _ = handle.join();
    }
}

fn show(mut notification: Notification) {
    notification.hint(Hint::Custom(
        "x-kde-show-when-inhibited".into(),
        "true".into(),
    ));
    let handle = std::thread::spawn(move || {
        if let Err(err) = notification.show() {
            err!("Failed to show notification: {err}");
        }
    });
    PENDING
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap()
        .push(handle);
}
