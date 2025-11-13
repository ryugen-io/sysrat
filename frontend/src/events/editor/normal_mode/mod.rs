mod editing;
mod insert_commands;
mod navigation;

use crate::state::AppState;
use editing::handle_editing;
use insert_commands::handle_insert_commands;
use navigation::handle_navigation;
use ratzilla::event::KeyEvent;

pub(super) fn handle_normal_mode(state: &mut AppState, key_event: KeyEvent) {
    if handle_insert_commands(state, &key_event) {
        return;
    }
    if handle_navigation(state, &key_event) {
        return;
    }
    handle_editing(state, &key_event);
}
