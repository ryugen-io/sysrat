use serde_json::Value;

pub(super) fn extract_id(c: &Value) -> String {
    c.get("Id")
        .and_then(|i| i.as_str())
        .unwrap_or("")
        .chars()
        .take(12)
        .collect()
}

pub(super) fn extract_name(c: &Value) -> String {
    c.get("Name")
        .and_then(|n| n.as_str())
        .unwrap_or("")
        .trim_start_matches('/')
        .to_string()
}

pub(super) fn extract_image(c: &Value) -> String {
    c.get("Config")
        .and_then(|cfg| cfg.get("Image"))
        .and_then(|i| i.as_str())
        .unwrap_or("")
        .to_string()
}

pub(super) fn extract_state(c: &Value) -> String {
    c.get("State")
        .and_then(|s| s.get("Status"))
        .and_then(|s| s.as_str())
        .unwrap_or("")
        .to_string()
}

pub(super) fn extract_status(c: &Value) -> String {
    extract_state(c)
}

pub(super) fn extract_created(c: &Value) -> String {
    c.get("Created")
        .and_then(|cr| cr.as_str())
        .unwrap_or("")
        .to_string()
}

pub(super) fn extract_started(c: &Value) -> String {
    c.get("State")
        .and_then(|s| s.get("StartedAt"))
        .and_then(|s| s.as_str())
        .unwrap_or("")
        .to_string()
}
