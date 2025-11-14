use crate::state::{AppState, Pane, refresh};

pub(super) fn next(state: &mut AppState) {
    state.container_list.next();
    refresh::save_selection(Pane::ContainerList, state);
}

pub(super) fn previous(state: &mut AppState) {
    state.container_list.previous();
    refresh::save_selection(Pane::ContainerList, state);
}
