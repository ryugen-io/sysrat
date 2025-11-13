use super::validation::{build_config_path, config_dir, validate_filename};
use crate::routes::types::{
    FileContentResponse, FileListResponse, WriteConfigRequest, WriteConfigResponse,
};
use axum::{Json, extract::Path, http::StatusCode};

/// GET /api/configs - List all config files
pub async fn list_configs() -> Result<Json<FileListResponse>, (StatusCode, String)> {
    // Ensure config directory exists
    tokio::fs::create_dir_all(config_dir()).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create config dir: {}", e),
        )
    })?;

    let mut files = Vec::new();
    let mut dir = tokio::fs::read_dir(config_dir()).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read directory: {}", e),
        )
    })?;

    while let Some(entry) = dir.next_entry().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read entry: {}", e),
        )
    })? {
        if let Some(filename) = entry.file_name().to_str() {
            // Only include .conf and .toml files
            if filename.ends_with(".conf") || filename.ends_with(".toml") {
                files.push(filename.to_string());
            }
        }
    }

    files.sort();
    Ok(Json(FileListResponse { files }))
}

/// GET /api/configs/:filename - Read a config file
pub async fn read_config(
    Path(filename): Path<String>,
) -> Result<Json<FileContentResponse>, (StatusCode, String)> {
    validate_filename(&filename)?;

    let path = build_config_path(&filename);

    match tokio::fs::read_to_string(&path).await {
        Ok(content) => Ok(Json(FileContentResponse { content })),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err((
            StatusCode::NOT_FOUND,
            format!("File not found: {}", filename),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Read error: {}", e),
        )),
    }
}

/// POST /api/configs/:filename - Write a config file
pub async fn write_config(
    Path(filename): Path<String>,
    Json(payload): Json<WriteConfigRequest>,
) -> Result<Json<WriteConfigResponse>, (StatusCode, String)> {
    validate_filename(&filename)?;

    let path = build_config_path(&filename);

    // Create backup before writing (if file exists)
    let backup_path = format!("{}.backup", path);
    let _ = tokio::fs::copy(&path, &backup_path).await;

    match tokio::fs::write(&path, payload.content.as_bytes()).await {
        Ok(_) => Ok(Json(WriteConfigResponse { success: true })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Write error: {}", e),
        )),
    }
}
