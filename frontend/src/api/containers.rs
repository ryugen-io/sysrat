use super::types::{ContainerInfo, ContainerListResponse};
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
