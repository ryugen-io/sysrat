use crate::{
    api,
    state::{AppState, status_helper},
    utils,
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen_futures::spawn_local;

pub(super) fn load_details(state: &AppState, state_rc: &Rc<RefCell<AppState>>) {
    if let Some(container) = state.container_list._selected() {
        let container_id = container.id.clone();
        let state_clone = Rc::clone(state_rc);
        spawn_local(async move {
            match api::fetch_container_details(&container_id).await {
                Ok(details) => {
                    let mut st = state_clone.borrow_mut();
                    st.container_details = Some(details);
                    st.set_status("[loaded]".to_string());
                }
                Err(e) => {
                    status_helper::set_status_timed(
                        &state_clone,
                        format!(
                            "[ERROR loading details: {}]",
                            utils::error::format_error(&e)
                        ),
                    );
                }
            }
        });
    }
}
