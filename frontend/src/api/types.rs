use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(super) struct FileListResponse {
    pub files: Vec<String>,
}

#[derive(Deserialize)]
pub(super) struct FileContentResponse {
    pub content: String,
}

#[derive(Serialize)]
pub(super) struct WriteConfigRequest {
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub state: String,
    pub status: String,
}

#[derive(Deserialize)]
pub(super) struct ContainerListResponse {
    pub containers: Vec<ContainerInfo>,
}
