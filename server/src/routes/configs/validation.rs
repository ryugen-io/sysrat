use axum::http::StatusCode;

/// Validates a filename for security and allowed extensions
pub(super) fn validate_filename(filename: &str) -> Result<(), (StatusCode, String)> {
    // Security: No path traversal
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return Err((StatusCode::BAD_REQUEST, "Invalid filename".into()));
    }

    // Only allow .conf and .toml files
    if !filename.ends_with(".conf") && !filename.ends_with(".toml") {
        return Err((
            StatusCode::BAD_REQUEST,
            "Only .conf and .toml files allowed".into(),
        ));
    }

    Ok(())
}

/// Builds the full path for a config file
pub(super) fn build_config_path(filename: &str) -> String {
    const CONFIG_DIR: &str = "/tmp/config-manager-configs";
    format!("{}/{}", CONFIG_DIR, filename)
}

/// Returns the config directory path
pub(super) fn config_dir() -> &'static str {
    "/tmp/config-manager-configs"
}
