use crate::state::AppState;
use ratzilla::event::{KeyCode, KeyEvent};

pub(super) fn handle_navigation(state: &mut AppState, key_event: &KeyEvent) -> bool {
    match key_event.code {
        KeyCode::Char('h') | KeyCode::Left => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Back);
            true
        }
        KeyCode::Char('j') | KeyCode::Down => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Down);
            true
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Up);
            true
        }
        KeyCode::Char('l') | KeyCode::Right => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Forward);
            true
        }
        KeyCode::Char('0') => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Head);
            true
        }
        KeyCode::Char('$') => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::End);
            true
        }
        KeyCode::Char('g') => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Top);
            true
        }
        KeyCode::Char('G') => {
            state
                .editor
                .textarea
                .move_cursor(tui_textarea::CursorMove::Bottom);
            true
        }
        _ => false,
    }
}
