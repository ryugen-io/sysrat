use ratzilla::event::{KeyCode, KeyEvent};
use tui_textarea::Input;

pub(super) fn convert_key_event_to_input(key_event: KeyEvent) -> Input {
    Input {
        key: convert_keycode(key_event.code),
        ctrl: key_event.ctrl,
        alt: key_event.alt,
        shift: key_event.shift,
    }
}

fn convert_keycode(code: KeyCode) -> tui_textarea::Key {
    match code {
        KeyCode::Char(c) => tui_textarea::Key::Char(c),
        KeyCode::Backspace => tui_textarea::Key::Backspace,
        KeyCode::Enter => tui_textarea::Key::Enter,
        KeyCode::Left => tui_textarea::Key::Left,
        KeyCode::Right => tui_textarea::Key::Right,
        KeyCode::Up => tui_textarea::Key::Up,
        KeyCode::Down => tui_textarea::Key::Down,
        KeyCode::Tab => tui_textarea::Key::Tab,
        KeyCode::Delete => tui_textarea::Key::Delete,
        KeyCode::Home => tui_textarea::Key::Home,
        KeyCode::End => tui_textarea::Key::End,
        KeyCode::PageUp => tui_textarea::Key::PageUp,
        KeyCode::PageDown => tui_textarea::Key::PageDown,
        KeyCode::Esc => tui_textarea::Key::Esc,
        _ => tui_textarea::Key::Null,
    }
}
