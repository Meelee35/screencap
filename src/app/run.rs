use crate::{app, capture, config, persistent, ui};
use crate::ui::main_window::MainWindowView;

pub fn run() -> anyhow::Result<()> {
    log!("Loading config");
    let config = config::loader::load();
    log!("Loading state");
    let persistent_state = persistent::state::load().unwrap_or_default();

    let mut app_state = app::state::AppState::new(&persistent_state);

    log!("Capturing screen");
    app_state.capture = capture::capture()?;
    log!("Setting up window");
    let window = MainWindowView::new(persistent_state.last_mode)?;
    window.set_capture(app_state.capture.image);
    log!("Showing window");
    window.run()?;

    Ok(())
}