use serde_json::Value;

pub(super) fn extract_environment(c: &Value) -> Vec<String> {
    c.get("Config")
        .and_then(|cfg| cfg.get("Env"))
        .and_then(|e| e.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

pub(super) fn extract_restart_policy(c: &Value) -> String {
    c.get("HostConfig")
        .and_then(|h| h.get("RestartPolicy"))
        .and_then(|r| r.get("Name"))
        .and_then(|n| n.as_str())
        .unwrap_or("no")
        .to_string()
}

pub(super) fn extract_health(c: &Value) -> Option<String> {
    c.get("State")
        .and_then(|s| s.get("Health"))
        .and_then(|h| h.get("Status"))
        .and_then(|s| s.as_str())
        .map(|s| s.to_string())
}
