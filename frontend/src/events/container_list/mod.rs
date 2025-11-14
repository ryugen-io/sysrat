mod actions;
mod details;
mod navigation;

use crate::state::{AppState, Pane};
use ratzilla::event::{KeyCode, KeyEvent};
use std::{cell::RefCell, rc::Rc};

pub fn handle_keys(state: &mut AppState, state_rc: &Rc<RefCell<AppState>>, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('j') | KeyCode::Down => navigation::next(state),
        KeyCode::Char('k') | KeyCode::Up => navigation::previous(state),
        KeyCode::Enter => details::load_details(state, state_rc),
        KeyCode::Char('s') => actions::start_container(state, state_rc),
        KeyCode::Char('x') => actions::stop_container(state, state_rc),
        KeyCode::Char('r') => actions::restart_container(state, state_rc),
        KeyCode::Esc => state.focus = Pane::Menu,
        KeyCode::Left if key_event.ctrl => state.focus = Pane::Menu,
        _ => {}
    }
}
