use super::super::types::ContainerActionResponse;
use axum::{Json, http::StatusCode};
use tokio::process::Command;

/// Execute a docker action (start/stop/restart) on a container
pub(super) async fn execute_container_action(
    container_id: &str,
    action: &str,
) -> Result<Json<ContainerActionResponse>, (StatusCode, String)> {
    let output = Command::new("docker")
        .args([action, container_id])
        .output()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to execute docker {}: {}", action, e),
            )
        })?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Docker {} failed: {}", action, error),
        ));
    }

    Ok(Json(ContainerActionResponse {
        success: true,
        message: format!("Container {} successful", action),
    }))
}
