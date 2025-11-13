use axum::http::StatusCode;

/// Validates a filename for security and allowed extensions
pub(super) fn validate_filename(filename: &str) -> Result<(), (StatusCode, String)> {
    // Security: No path traversal or Windows paths
    // Forward slashes (/) are allowed for directory-scanned files
    if filename.contains("..") || filename.contains('\\') {
        return Err((StatusCode::BAD_REQUEST, "Invalid filename".into()));
    }

    // Extract actual filename from path (handle directory-scanned files)
    let actual_filename = filename.rsplit('/').next().unwrap_or(filename);

    // Only allow .conf and .toml files (check the actual filename, not the path)
    if !actual_filename.ends_with(".conf")
        && !actual_filename.ends_with(".toml")
        && !actual_filename.ends_with(".rs")
        && !actual_filename.ends_with(".txt")
        && !actual_filename.ends_with(".sh")
        && !actual_filename.ends_with(".fish")
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "Only .conf, .toml, .rs, .txt, .sh, .fish files allowed".into(),
        ));
    }

    Ok(())
}
