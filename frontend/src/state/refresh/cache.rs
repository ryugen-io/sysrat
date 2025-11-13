use crate::state::{AppState, Pane};

/// Save selection index for a pane
pub fn save_selection(pane: Pane, state: &AppState) {
    match pane {
        Pane::FileList => {
            crate::storage::generic::save("file-list-selection", &state.file_list.selected_index);
        }
        Pane::ContainerList => {
            crate::storage::generic::save(
                "container-list-selection",
                &state.container_list.selected_index,
            );
        }
        _ => {}
    }
}

/// Load cached data for a pane from storage
pub fn load_pane_cache(pane: Pane, state: &mut AppState) {
    match pane {
        Pane::FileList => {
            if let Some(files) = crate::storage::generic::load("file-list") {
                state.file_list.set_files(files);
            }
            // Restore selection index
            if let Some(index) = crate::storage::generic::load::<usize>("file-list-selection")
                && index < state.file_list.files.len()
            {
                state.file_list.selected_index = index;
            }
        }
        Pane::ContainerList => {
            if let Some(containers) = crate::storage::generic::load("container-list") {
                state.container_list.set_containers(containers);
            }
            // Restore selection index
            if let Some(index) = crate::storage::generic::load::<usize>("container-list-selection")
                && index < state.container_list.containers.len()
            {
                state.container_list.selected_index = index;
            }
        }
        _ => {}
    }
}
