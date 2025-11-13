use crate::state::{AppState, VimMode};
use ratzilla::event::{KeyCode, KeyEvent};

pub(super) fn handle_normal_mode(state: &mut AppState, key_event: KeyEvent) {
    match key_event.code {
        // Enter insert mode
        KeyCode::Char('i') => {
            state.vim_mode = VimMode::Insert;
        }
        KeyCode::Char('a') => {
            state.vim_mode = VimMode::Insert;
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Forward);
        }
        KeyCode::Char('A') => {
            state.vim_mode = VimMode::Insert;
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::End);
        }
        KeyCode::Char('I') => {
            state.vim_mode = VimMode::Insert;
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Head);
        }
        KeyCode::Char('o') => {
            state.vim_mode = VimMode::Insert;
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::End);
            state.editor.textarea.insert_newline();
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
        }
        // Navigation
        KeyCode::Char('h') | KeyCode::Left => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Back);
        }
        KeyCode::Char('j') | KeyCode::Down => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Down);
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Up);
        }
        KeyCode::Char('l') | KeyCode::Right => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Forward);
        }
        KeyCode::Char('0') => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Head);
        }
        KeyCode::Char('$') => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::End);
        }
        KeyCode::Char('g') => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Top);
        }
        KeyCode::Char('G') => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Bottom);
        }
        // Delete line
        KeyCode::Char('d') => {
            state.editor.textarea.delete_line_by_head();
        }
        KeyCode::Char('u') => {
            state.editor.textarea.undo();
        }
        KeyCode::Char('r') if key_event.ctrl => {
            state.editor.textarea.redo();
        }
        _ => {}
    }
}
