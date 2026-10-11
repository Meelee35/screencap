use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread::JoinHandle;
use notify_rust::{Hint, Notification};
use crate::capture::pixels::Capture;
use crate::constants;

static PENDING: OnceLock<Mutex<Vec<JoinHandle<()>>>> = OnceLock::new();
static COUNTER: AtomicU64 = AtomicU64::new(0);

const MAX_IMAGE_WIDTH: u32 = 400;
const MAX_IMAGE_HEIGHT: u32 = 300;

pub fn warning(message: &str) {
    let mut notification = Notification::new();
    notification
        .appname(constants::APP_NAME)
        .summary(constants::APP_NAME)
        .icon("dialog-warning")
        .body(message);
    show(notification);
}

pub fn success(message: &str, capture: &Capture) {
    let mut notification = Notification::new();
    notification
        .appname(constants::APP_NAME)
        .summary(constants::APP_NAME);
    if let Some(path) = preview_path(capture) {
        notification.body(&format!(
            "{message}<img src=\"file://{}\" alt=\"Screenshot\"/>",
            path.display()
        ));
    } else {
        notification.body(message);
    }
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

fn preview_path(capture: &Capture) -> Option<PathBuf> {
    let image = downscale(&capture.image);
    let path = std::env::temp_dir().join(format!(
        "screencap-{}-{}.png",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    match image.save(&path) {
        Ok(()) => Some(path),
        Err(err) => {
            err!("Failed to write notification preview: {err}");
            None
        }
    }
}

fn downscale(image: &image::RgbaImage) -> image::RgbaImage {
    let scale = (MAX_IMAGE_WIDTH as f32 / image.width() as f32)
        .min(MAX_IMAGE_HEIGHT as f32 / image.height() as f32)
        .min(1.0);
    if scale >= 1.0 {
        return image.clone();
    }
    let width = ((image.width() as f32 * scale).round() as u32).max(1);
    let height = ((image.height() as f32 * scale).round() as u32).max(1);
    image::imageops::resize(image, width, height, image::imageops::FilterType::Triangle)
}

fn show(mut notification: Notification) {
    notification.hint(Hint::Custom("x-kde-show-when-inhibited".into(), "true".into()));
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
