use super::AppState;
use gloo_timers::callback::Timeout;
use std::{cell::RefCell, rc::Rc};

/// Set status message with automatic clearing after 3 seconds
pub fn set_status_timed(state_rc: &Rc<RefCell<AppState>>, message: impl Into<String>) {
    state_rc.borrow_mut().set_status(message);

    let state_clone = Rc::clone(state_rc);
    Timeout::new(3_000, move || {
        state_clone.borrow_mut().clear_status();
    })
    .forget();
}
