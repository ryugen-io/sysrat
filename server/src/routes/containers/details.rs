use super::super::types::ContainerDetailsResponse;
use super::parser;
use axum::{Json, extract::Path, http::StatusCode};
use serde_json::Value;
use tokio::process::Command;

/// GET /api/containers/:id/details - Get detailed information about a container
pub async fn get_container_details(
    Path(id): Path<String>,
) -> Result<Json<ContainerDetailsResponse>, (StatusCode, String)> {
    let inspect_output = fetch_container_inspect(&id).await?;
    let container = parse_inspect_json(&inspect_output)?;
    let details = parser::build_details(&container)?;

    Ok(Json(ContainerDetailsResponse { details }))
}

async fn fetch_container_inspect(id: &str) -> Result<String, (StatusCode, String)> {
    let output = Command::new("docker")
        .args(["inspect", id])
        .output()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to execute docker inspect: {}", e),
            )
        })?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err((
            StatusCode::NOT_FOUND,
            format!("Container not found: {}", error),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn parse_inspect_json(json_str: &str) -> Result<Value, (StatusCode, String)> {
    let json: Vec<Value> = serde_json::from_str(json_str).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to parse docker inspect output: {}", e),
        )
    })?;

    if json.is_empty() {
        return Err((StatusCode::NOT_FOUND, "Container not found".to_string()));
    }

    Ok(json[0].clone())
}
