use crate::api::ContainerInfo;

pub struct ContainerListState {
    pub containers: Vec<ContainerInfo>,
    pub selected_index: usize,
}

impl ContainerListState {
    pub fn new() -> Self {
        Self {
            containers: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn next(&mut self) {
        if !self.containers.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.containers.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.containers.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.containers.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn _selected(&self) -> Option<&ContainerInfo> {
        self.containers.get(self.selected_index)
    }

    pub fn set_containers(&mut self, containers: Vec<ContainerInfo>) {
        // Preserve selection by container ID
        let selected_id = self._selected().map(|c| c.id.clone());

        self.containers = containers;

        // Try to restore previous selection
        if let Some(id) = selected_id
            && let Some(pos) = self.containers.iter().position(|c| c.id == id)
        {
            self.selected_index = pos;
            return;
        }

        // Fallback: Keep index within bounds
        if self.selected_index >= self.containers.len() && !self.containers.is_empty() {
            self.selected_index = self.containers.len() - 1;
        }
    }
}
