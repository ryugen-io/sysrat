#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Menu,
    FileList,
    Editor,
    ContainerList,
}

impl Pane {
    pub fn as_str(&self) -> &'static str {
        match self {
            Pane::Menu => "Menu",
            Pane::FileList => "FileList",
            Pane::Editor => "Editor",
            Pane::ContainerList => "ContainerList",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Menu" => Some(Pane::Menu),
            "FileList" => Some(Pane::FileList),
            "Editor" => Some(Pane::Editor),
            "ContainerList" => Some(Pane::ContainerList),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VimMode {
    Normal,
    Insert,
}
