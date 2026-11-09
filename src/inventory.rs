#[cfg(any(target_os = "macos", target_os = "windows", test))]
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventoryDevice {
    pub name: String,
    pub id: String,
    pub kind: String,
}

#[cfg(target_os = "linux")]
pub fn discover() -> Result<Vec<InventoryDevice>, String> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir("/sys/bus/usb/devices").map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        let vendor = std::fs::read_to_string(path.join("idVendor")).unwrap_or_default();
        if !vendor.trim().eq_ignore_ascii_case("046d") {
            continue;
        }
        let product_id = std::fs::read_to_string(path.join("idProduct")).unwrap_or_default();
        let name = std::fs::read_to_string(path.join("product")).unwrap_or_default();
        let id = product_id.trim().to_uppercase();
        if id.len() != 4 || !id.bytes().all(|b| b.is_ascii_hexdigit()) {
            continue;
        }
        found.push(InventoryDevice {
            name: fallback_name(name.trim()),
            id: format!("046D:{id}"),
            kind: "USB".into(),
        });
    }
    found.sort_by(|a, b| (&a.name, &a.id).cmp(&(&b.name, &b.id)));
    Ok(found)
}

#[cfg(target_os = "macos")]
pub fn discover() -> Result<Vec<InventoryDevice>, String> {
    let out = std::process::Command::new("system_profiler")
        .args(["SPUSBDataType", "-json"])
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err("system_profiler failed".into());
    }
    parse_macos(&out.stdout)
}

#[cfg(target_os = "windows")]
pub fn discover() -> Result<Vec<InventoryDevice>, String> {
    const SCRIPT: &str = "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; Get-PnpDevice -PresentOnly | Where-Object { $_.InstanceId -match 'VID_046D' } | Select-Object FriendlyName,InstanceId,Class | ConvertTo-Json -Compress";
    let out = std::process::Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", SCRIPT])
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err("Get-PnpDevice failed".into());
    }
    parse_windows(&out.stdout)
}

fn fallback_name(name: &str) -> String {
    if name.is_empty() {
        "Logitech device".into()
    } else {
        name.to_owned()
    }
}

#[cfg(any(target_os = "macos", test))]
fn parse_macos(bytes: &[u8]) -> Result<Vec<InventoryDevice>, String> {
    let root: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    let mut found = Vec::new();
    fn visit(value: &Value, found: &mut Vec<InventoryDevice>) {
        match value {
            Value::Array(items) => {
                for item in items {
                    visit(item, found);
                }
            }
            Value::Object(map) => {
                let vendor = map
                    .get("vendor_id")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_ascii_lowercase();
                if vendor.starts_with("0x046d") {
                    let product = map
                        .get("product_id")
                        .and_then(Value::as_str)
                        .unwrap_or_default();
                    let id = product
                        .split_whitespace()
                        .next()
                        .unwrap_or_default()
                        .trim_start_matches("0x")
                        .to_uppercase();
                    if id.len() == 4 && id.bytes().all(|b| b.is_ascii_hexdigit()) {
                        let name = map.get("_name").and_then(Value::as_str).unwrap_or_default();
                        found.push(InventoryDevice {
                            name: fallback_name(name),
                            id: format!("046D:{id}"),
                            kind: "USB".into(),
                        });
                    }
                }
                for child in map.values() {
                    visit(child, found);
                }
            }
            _ => {}
        }
    }
    visit(&root, &mut found);
    found.sort_by(|a, b| (&a.name, &a.id).cmp(&(&b.name, &b.id)));
    Ok(found)
}

#[cfg(any(target_os = "windows", test))]
fn parse_windows(bytes: &[u8]) -> Result<Vec<InventoryDevice>, String> {
    if bytes.iter().all(u8::is_ascii_whitespace) {
        return Ok(Vec::new());
    }
    let root: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    let items: Vec<&Value> = match &root {
        Value::Array(items) => items.iter().collect(),
        Value::Object(_) => vec![&root],
        _ => Vec::new(),
    };
    let mut found = Vec::new();
    for item in items {
        let instance = item
            .get("InstanceId")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_ascii_uppercase();
        let Some(start) = instance.find("VID_046D&PID_") else {
            continue;
        };
        let id = instance.get(start + 13..start + 17).unwrap_or_default();
        if id.len() != 4 || !id.bytes().all(|b| b.is_ascii_hexdigit()) {
            continue;
        }
        let name = item
            .get("FriendlyName")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let kind = item.get("Class").and_then(Value::as_str).unwrap_or("PnP");
        found.push(InventoryDevice {
            name: fallback_name(name),
            id: format!("046D:{id}"),
            kind: kind.to_owned(),
        });
    }
    found.sort_by(|a, b| (&a.name, &a.id).cmp(&(&b.name, &b.id)));
    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn macos_usb_tree_finds_logitech_camera() {
        let json = br#"{"SPUSBDataType":[{"_items":[{"_name":"HD Pro Webcam","vendor_id":"0x046d (Logitech)","product_id":"0x082d"}]}]}"#;
        assert_eq!(parse_macos(json).unwrap()[0].id, "046D:082D");
    }
    #[test]
    fn windows_pnp_finds_logitech_wheel() {
        let json = br#"{"FriendlyName":"G923 Racing Wheel","InstanceId":"USB\\VID_046D&PID_C26E\\123","Class":"HIDClass"}"#;
        assert_eq!(parse_windows(json).unwrap()[0].name, "G923 Racing Wheel");
    }
    #[test]
    fn windows_pnp_empty_inventory_is_not_an_error() {
        assert!(parse_windows(b"\r\n").unwrap().is_empty());
    }
}
