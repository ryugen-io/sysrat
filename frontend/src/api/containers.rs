use super::types::{
    ContainerActionResponse, ContainerDetails, ContainerDetailsResponse, ContainerInfo,
    ContainerListResponse,
};
use gloo_net::http::Request;
use wasm_bindgen::JsValue;

pub async fn fetch_container_list() -> Result<Vec<ContainerInfo>, JsValue> {
    let response = Request::get("/api/containers")
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to fetch containers: {}", e)))?;

    if !response.ok() {
        return Err(JsValue::from_str(&format!(
            "Server returned error: {}",
            response.status()
        )));
    }

    let data: ContainerListResponse = response
        .json()
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))?;

    Ok(data.containers)
}

pub async fn fetch_container_details(container_id: &str) -> Result<ContainerDetails, JsValue> {
    let url = format!("/api/containers/{}/details", container_id);
    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to fetch container details: {}", e)))?;

    if !response.ok() {
        return Err(JsValue::from_str(&format!(
            "Server returned error: {}",
            response.status()
        )));
    }

    let data: ContainerDetailsResponse = response
        .json()
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))?;

    Ok(data.details)
}

pub async fn start_container(container_id: &str) -> Result<String, JsValue> {
    execute_container_action(container_id, "start").await
}

pub async fn stop_container(container_id: &str) -> Result<String, JsValue> {
    execute_container_action(container_id, "stop").await
}

pub async fn restart_container(container_id: &str) -> Result<String, JsValue> {
    execute_container_action(container_id, "restart").await
}

async fn execute_container_action(container_id: &str, action: &str) -> Result<String, JsValue> {
    let url = format!("/api/containers/{}/{}", container_id, action);
    let response = Request::post(&url)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to {} container: {}", action, e)))?;

    if !response.ok() {
        return Err(JsValue::from_str(&format!(
            "Server returned error: {}",
            response.status()
        )));
    }

    let data: ContainerActionResponse = response
        .json()
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))?;

    if !data.success {
        return Err(JsValue::from_str(&format!(
            "Action failed: {}",
            data.message
        )));
    }

    Ok(data.message)
}
