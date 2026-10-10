use crate::persistent::state::Mode;
use slint::{ComponentHandle, Image, Rgba8Pixel, SharedPixelBuffer};
use super::{CaptureMode, MainWindow};

pub struct MainWindowView {
    window: MainWindow,
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

    pub fn set_capture(&self, image: image::RgbaImage) {
        let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
            image.as_raw(),
            image.width(),
            image.height(),
        );
        self.window.set_capture(Image::from_rgba8(buffer));
    }

    pub fn on_mode_changed(&self, handler: impl Fn(Mode) + 'static) {
        // TODO: window.on_mode_changed(...), mapping CaptureMode -> Mode inside the closure
        todo!()

    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.window.run()
    }

    pub fn close(&self) {
        let _ = self.window.hide();
    }
}