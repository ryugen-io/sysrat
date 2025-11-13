#[derive(Debug, Clone)]
pub struct SavedState {
    pub pane: String,
    pub filename: Option<String>,
    pub content: Option<String>,
}
