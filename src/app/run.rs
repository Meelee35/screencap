use std::cell::RefCell;
use std::rc::Rc;
use crate::{app, capture, config, persistent, ui};
use crate::app::state::AppState;
use crate::ui::main_window::MainWindowView;

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
    let view = window.handle();

    let mode_state = state.clone();
    window.on_mode_changed(move |mode| {
        if let Some(state) = mode_state.upgrade() {
            state.borrow_mut().mode = mode.into();
        }
    });

    let cancel_state = state.clone();
    window.on_cancel_requested(move || {
        log!("Cancel requested");
        view.close();
        if let Some(state) = cancel_state.upgrade() {
            state.borrow_mut().cancelled = true;
        }
    });

    log!("Showing window");
    window.run()?;
    if app_state.borrow().cancelled {
        return Ok(())
    }

    Ok(())
}