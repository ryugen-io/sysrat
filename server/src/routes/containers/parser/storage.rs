use crate::routes::types::VolumeMount;
use serde_json::Value;

pub(super) fn extract_volumes(c: &Value) -> Vec<VolumeMount> {
    c.get("Mounts")
        .and_then(|m| m.as_array())
        .map(|mounts| {
            mounts
                .iter()
                .filter_map(|mount| {
                    if let (Some(src), Some(dst), Some(mode)) = (
                        mount.get("Source").and_then(|s| s.as_str()),
                        mount.get("Destination").and_then(|d| d.as_str()),
                        mount.get("Mode").and_then(|m| m.as_str()),
                    ) {
                        Some(VolumeMount {
                            source: src.to_string(),
                            destination: dst.to_string(),
                            mode: mode.to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}
