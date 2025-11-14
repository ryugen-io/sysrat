use crate::routes::types::PortMapping;
use serde_json::Value;

pub(super) fn extract_ports(c: &Value) -> Vec<PortMapping> {
    let mut ports = Vec::new();
    if let Some(port_map) = c
        .get("NetworkSettings")
        .and_then(|n| n.get("Ports"))
        .and_then(|p| p.as_object())
    {
        for (container_port, bindings) in port_map {
            if let Some(bind_arr) = bindings.as_array() {
                for bind in bind_arr {
                    if let Some(host_port) = bind.get("HostPort").and_then(|p| p.as_str()) {
                        let parts: Vec<&str> = container_port.split('/').collect();
                        ports.push(PortMapping {
                            container_port: parts[0].to_string(),
                            host_port: host_port.to_string(),
                            protocol: parts.get(1).unwrap_or(&"tcp").to_string(),
                        });
                    }
                }
            }
        }
    }
    ports
}

pub(super) fn extract_networks(c: &Value) -> Vec<String> {
    c.get("NetworkSettings")
        .and_then(|n| n.get("Networks"))
        .and_then(|nets| nets.as_object())
        .map(|obj| obj.keys().cloned().collect())
        .unwrap_or_default()
}
