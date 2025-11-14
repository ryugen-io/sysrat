mod basic;
mod config;
mod network;
mod storage;

use super::super::types::ContainerDetails;
use axum::http::StatusCode;
use serde_json::Value;

pub(super) fn build_details(container: &Value) -> Result<ContainerDetails, (StatusCode, String)> {
    Ok(ContainerDetails {
        id: basic::extract_id(container),
        name: basic::extract_name(container),
        image: basic::extract_image(container),
        state: basic::extract_state(container),
        status: basic::extract_status(container),
        created: basic::extract_created(container),
        started: basic::extract_started(container),
        ports: network::extract_ports(container),
        volumes: storage::extract_volumes(container),
        networks: network::extract_networks(container),
        environment: config::extract_environment(container),
        restart_policy: config::extract_restart_policy(container),
        health: config::extract_health(container),
    })
}
