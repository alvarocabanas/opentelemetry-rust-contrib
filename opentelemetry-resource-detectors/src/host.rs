//! HOST resource detector
//!
//! Detect the unique host ID.
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::time::Duration;
use opentelemetry::KeyValue;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::resource::ResourceDetector;

/// Detect the unique host ID.
///
/// This detector looks up the host id using the sources defined
/// in the OpenTelemetry semantic conventions [`host.id from non-containerized systems`].
///
/// [`host.id from non-containerized systems`]: https://opentelemetry.io/docs/specs/semconv/resource/host/#collecting-hostid-from-non-containerized-systems
pub struct HostResourceDetector {
    host_id_detect: fn() -> Option<String>
}

impl ResourceDetector for HostResourceDetector {
    fn detect(&self, _timeout: Duration) -> Resource {
        (self.host_id_detect)().map(|host_id| {
            Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::HOST_ID,
                host_id,
            )])
        }).unwrap_or(Resource::new(vec![]))
    }
}

#[cfg(target_os = "linux")]
fn host_id_detect() -> Option<String> {
    let machine_id_path  = Path::new("/etc/machine-id");
    let dbus_machine_id_path = Path::new("/var/lib/dbus/machine-id");
    read_to_string(machine_id_path).or_else(|_|{read_to_string(dbus_machine_id_path)}).ok()
}

#[cfg(not(target_os = "linux"))]
fn host_id_detect() -> Option<String> {
    None
}

impl Default for HostResourceDetector {
    fn default() -> Self {
        Self {
            host_id_detect
        }
    }
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod tests {
    use std::time::Duration;
    use opentelemetry::{Key, Value};
    use opentelemetry_sdk::resource::ResourceDetector;
    use super::HostResourceDetector;

    #[test]
    fn test_host_resource_detector() {
        let resource =  HostResourceDetector::default().detect(Duration::from_secs(0));
        assert_eq!(resource.len(), 1);
        assert!(
            resource.get(Key::from_static_str(
                opentelemetry_semantic_conventions::resource::HOST_ID
            )).is_some()
        )
    }
}