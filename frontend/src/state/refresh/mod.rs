mod cache;
mod container_list;
mod file_list;

use crate::state::{AppState, Pane};
use std::{cell::RefCell, rc::Rc};

// Re-export cache functions
pub use cache::{load_pane_cache, save_selection};

// Re-export background refresh
pub use container_list::start_background_refresh;

/// Refresh data for a specific pane
pub fn refresh_pane(pane: Pane, state_rc: &Rc<RefCell<AppState>>) {
    match pane {
        Pane::FileList => file_list::refresh_file_list(state_rc),
        Pane::ContainerList => container_list::refresh_container_list(state_rc),
        _ => {}
    }
}
