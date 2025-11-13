use super::input::convert_key_event_to_input;
use crate::state::{AppState, VimMode};
use ratzilla::event::{KeyCode, KeyEvent};

pub(super) fn handle_insert_mode(state: &mut AppState, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => {
            state.vim_mode = VimMode::Normal;
        }
        _ => {
            let input = convert_key_event_to_input(key_event);
            state.editor.textarea.input(input);
        }
    }
}
