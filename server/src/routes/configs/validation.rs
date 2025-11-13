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
