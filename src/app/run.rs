use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use crate::{capture, config, output, persistent};
use crate::app::state::{AppState, SaveType};
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
            state.borrow().close();
            state.borrow_mut().requested_action.get_or_insert_with(Vec::new).push(SaveType::File(PathBuf::new()));
        }
    });

    log!("Showing window");
    window.run()?;

    if let Some(actions) = &app_state.borrow().requested_action {
        for action in actions {
            match action {
                SaveType::Clipboard => {
                    todo!()
                }
                SaveType::File(_) => {
                    match app_state.borrow().capture_fullscreen(&config) {
                        Ok(saved) => {
                            let save_types = &saved.0;
                            if save_types.is_empty() {
                                output::notify::warning("No save method was specified, so nothing was saved.")
                            } else {
                                let message = saved.build_summary();
                                output::notify::success(&message, &app_state.borrow().capture);
                            }
                            output::notify::wait()
                        }
                        Err(e) => {
                            output::dialogue::error(&format!("Failed to save screen: {:#}", e));
                        }

                    };
                }
            }
        }
    }

    Ok(())
}
