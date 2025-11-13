use crate::state::AppState;
use ratzilla::event::{KeyCode, KeyEvent};

pub(super) fn handle_editing(state: &mut AppState, key_event: &KeyEvent) -> bool {
    match key_event.code {
        KeyCode::Char('d') => {
            state.editor.textarea.delete_line_by_head();
            true
        }
        KeyCode::Char('u') => {
            state.editor.textarea.undo();
            true
        }
        KeyCode::Char('r') if key_event.ctrl => {
            state.editor.textarea.redo();
            true
        }
        _ => false,
    }
}
