use crate::persistent::state::Mode;
use slint::{ComponentHandle, Image, Rgba8Pixel, SharedPixelBuffer};
use super::{CaptureMode, MainWindow};

pub struct MainWindowView {
    window: MainWindow,
}

#[derive(Clone)]
pub(crate) struct ViewHandle {
    window: slint::Weak::<MainWindow>
}

impl ViewHandle {
    pub(crate) fn close(&self) {
        if let Some(window) = self.window.upgrade() {
            let _ = window.hide();
        }
    }
}

impl MainWindowView {
    pub fn new(initial_mode: Mode) -> Result<Self, slint::PlatformError> {
        let window = MainWindow::new()?;
        window.set_mode(match initial_mode {
            Mode::Rectangle => CaptureMode::Rectangle,
            Mode::Fullscreen => CaptureMode::Fullscreen,
        });
        Ok(Self { window })
    }

    pub(crate) fn handle(&self) -> ViewHandle {
        ViewHandle {
            window: self.window.as_weak()
        }
    }

    pub fn set_capture(&self, image: image::RgbaImage) {
        let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
            image.as_raw(),
            image.width(),
            image.height(),
        );
        self.window.set_capture(Image::from_rgba8(buffer));
    }

    pub fn on_mode_changed(&self, handler: impl Fn(Mode) + 'static) {
        self.window.on_mode_changed(move |capture_mode| {
            log!("Changing mode to {:?}", capture_mode);
            let mode = match capture_mode {
                CaptureMode::Rectangle => Mode::Rectangle,
                CaptureMode::Fullscreen => Mode::Fullscreen,
            };

            handler(mode);
        })
    }

    pub fn on_cancel_requested(&self, handler: impl Fn() + 'static) {
        self.window.on_cancel_requested(move || handler());
    }

    pub fn on_capture_requested(&self, handler: impl Fn() + 'static) {
        self.window.on_capture_requested(move || handler());
    }

    pub fn on_region_requested(&self, handler: impl Fn(f64, f64, f64, f64) + 'static) {
        self.window.on_region_requested(move |x, y, w, h| {
            handler(x as f64, y as f64, w as f64, h as f64);
        });
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.window.run()
    }
}