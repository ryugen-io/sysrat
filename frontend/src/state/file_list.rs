use crate::api::FileInfo;

pub struct FileListState {
    pub files: Vec<FileInfo>,
    pub selected_index: usize,
}

impl FileListState {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn next(&mut self) {
        if !self.files.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.files.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.files.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.files.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn selected(&self) -> Option<&FileInfo> {
        self.files.get(self.selected_index)
    }

    pub fn set_files(&mut self, files: Vec<FileInfo>) {
        // Preserve selection by filename
        let selected_name = self.selected().map(|f| f.name.clone());

        self.files = files;

        // Try to restore previous selection
        if let Some(name) = selected_name
            && let Some(pos) = self.files.iter().position(|f| f.name == name)
        {
            self.selected_index = pos;
            return;
        }

        // Fallback: Keep index within bounds
        if self.selected_index >= self.files.len() && !self.files.is_empty() {
            self.selected_index = self.files.len() - 1;
        }
    }
}
