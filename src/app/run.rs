use crate::app::state::{AppState, CaptureMode, Rect, Rectangle};
use crate::ui::main_window::MainWindowView;
use crate::{capture, config, output, persistent};
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() -> anyhow::Result<()> {
    log!("Loading config");
    let config = config::loader::load();
    log!("Loading state");
    let persistent_state = persistent::state::load().unwrap_or_default();

    let app_state = Rc::new(RefCell::new(AppState::new(&persistent_state)));

    log!("Capturing screen");
    app_state.borrow_mut().capture = capture::capture()?;

    log!("Setting up window");
    let window = MainWindowView::new(persistent_state.last_mode)?;
    window.set_capture(app_state.borrow().capture.image.clone());

    let state = Rc::downgrade(&app_state);
    app_state.borrow_mut().view = Some(window.handle());

    let mode_state = state.clone();
    window.on_mode_changed(move |mode| {
        if let Some(state) = mode_state.upgrade() {
            state.borrow_mut().mode = mode.into();
        }
    });

    let cancel_state = state.clone();
    window.on_cancel_requested(move || {
        log!("Cancel requested");
        if let Some(state) = cancel_state.upgrade() {
            state.borrow().close();
        }
    });

    let capture_state = state.clone();
    window.on_capture_requested(move || {
        log!("Capture requested");
        if let Some(state) = capture_state.upgrade() {
            let mode = state.borrow().mode.clone();
            state.borrow().close();
            state.borrow_mut().action_requested = Some(mode);
        }
    });

    let region_state = state.clone();
    window.on_region_requested(move |x, y, w, h| {
        log!("Region requested");
        if let Some(state) = region_state.upgrade() {
            let mode = CaptureMode::Rectangle(Rectangle {
                selection: Some(Rect {
                    x,
                    y,
                    width: w,
                    height: h,
                }),
            });
            state.borrow_mut().mode = mode.clone();
            state.borrow().close();
            state.borrow_mut().action_requested = Some(mode);
        }
    });

    log!("Showing window");
    window.run()?;

    let capture_state = state.clone();
    let action = capture_state
        .upgrade()
        .and_then(|final_state| final_state.borrow().action_requested.clone());

    if let Some(action) = action {
        let result = match action {
            CaptureMode::Fullscreen => app_state.borrow().capture_fullscreen(&config),
            CaptureMode::Rectangle(rectangle) => {
                app_state.borrow().capture_rectangle(&config, &rectangle)
            }
        };

        match result {
            Ok(saved) => {
                if saved.0.is_empty() {
                    output::notify::warning("No save method was specified, so nothing was saved.")
                } else {
                    let message = saved.build_summary();
                    output::notify::success(&message);
                }
                output::notify::wait()
            }
            Err(e) => {
                output::dialogue::error(&format!("Failed to save screen: {:#}", e));
            }
        }
    }

    Ok(())
}
