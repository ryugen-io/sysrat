use crate::api;
use crate::state::{AppState, Pane};
use ratzilla::event::{KeyCode, KeyEvent};
use wasm_bindgen_futures::spawn_local;

pub fn handle_keys(state: &mut AppState, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('j') | KeyCode::Down => {
            state.container_list.next();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.container_list.previous();
        }
        KeyCode::Char('s') => {
            if let Some(container) = state.container_list._selected() {
                let container_id = container.id.clone();
                spawn_local(async move {
                    if let Err(e) = api::start_container(&container_id).await {
                        web_sys::console::error_1(
                            &format!("Failed to start container: {:?}", e).into(),
                        );
                    }
                });
            }
        }
        KeyCode::Char('x') => {
            if let Some(container) = state.container_list._selected() {
                let container_id = container.id.clone();
                spawn_local(async move {
                    if let Err(e) = api::stop_container(&container_id).await {
                        web_sys::console::error_1(
                            &format!("Failed to stop container: {:?}", e).into(),
                        );
                    }
                });
            }
        }
        KeyCode::Char('r') => {
            if let Some(container) = state.container_list._selected() {
                let container_id = container.id.clone();
                spawn_local(async move {
                    if let Err(e) = api::restart_container(&container_id).await {
                        web_sys::console::error_1(
                            &format!("Failed to restart container: {:?}", e).into(),
                        );
                    }
                });
            }
        }
        KeyCode::Esc => {
            state.focus = Pane::Menu;
        }
        KeyCode::Left if key_event.ctrl => {
            state.focus = Pane::Menu;
        }
        _ => {}
    }
}
