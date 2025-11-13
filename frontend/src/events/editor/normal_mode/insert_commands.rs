use crate::state::{AppState, VimMode};
use ratzilla::event::{KeyCode, KeyEvent};

pub(super) fn handle_insert_commands(state: &mut AppState, key_event: &KeyEvent) -> bool {
    match key_event.code {
        KeyCode::Char('i') => {
            state.vim_mode = VimMode::Insert;
            true
        }
        KeyCode::Char('a') => {
            state.vim_mode = VimMode::Insert;
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Forward);
            true
        }
        KeyCode::Char('A') => {
            state.vim_mode = VimMode::Insert;
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::End);
            true
        }
        KeyCode::Char('I') => {
            state.vim_mode = VimMode::Insert;
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Head);
            true
        }
        KeyCode::Char('o') => {
            state.vim_mode = VimMode::Insert;
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::End);
            state.editor.textarea.insert_newline();
            true
        }
        KeyCode::Char('O') => {
            state.vim_mode = VimMode::Insert;
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Head);
            state.editor.textarea.insert_newline();
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Up);
            true
        }
        _ => false,
    }
}
