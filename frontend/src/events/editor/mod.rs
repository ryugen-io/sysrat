mod input;
mod insert_mode;
mod normal_mode;

use crate::state::{AppState, VimMode};
use insert_mode::handle_insert_mode;
use normal_mode::handle_normal_mode;
use ratzilla::event::KeyEvent;

pub fn handle_keys(state: &mut AppState, key_event: KeyEvent) {
    match state.vim_mode {
        VimMode::Normal => handle_normal_mode(state, key_event),
        VimMode::Insert => handle_insert_mode(state, key_event),
    }

    state.check_dirty();
}
