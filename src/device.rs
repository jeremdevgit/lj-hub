#[cfg(target_os = "linux")]
use std::process::Command;

#[derive(Clone)]
pub struct Device {
    pub label: String,
    pub id: String,
    pub ratbag_id: Option<String>,
}

pub fn discover() -> Result<Vec<Device>, String> {
    let api = hidapi::HidApi::new().map_err(|e| e.to_string())?;
    let mut devices = Vec::new();
    for d in api.device_list().filter(|d| d.vendor_id() == 0x046d) {
        let id = format!("{:04X}:{:04X}", d.vendor_id(), d.product_id());
        if devices.iter().any(|x: &Device| x.id == id) {
            continue;
        }
        devices.push(Device {
            label: d.product_string().unwrap_or("Logitech device").to_owned(),
            id,
            ratbag_id: None,
        });
    }
    #[cfg(target_os = "linux")]
    if let Ok(out) = Command::new("ratbagctl").arg("list").output() {
        if out.status.success() {
            for line in String::from_utf8_lossy(&out.stdout).lines() {
                if let Some((id, label)) = line.split_once(':') {
                    let id = id.trim();
                    if id.is_empty() || !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
                        continue;
                    }
                    devices.push(Device {
                        label: label.trim().to_owned(),
                        id: format!("ratbag:{id}"),
                        ratbag_id: Some(id.to_owned()),
                    });
                }
            }
        }
    }
    Ok(devices)
}

#[cfg(target_os = "linux")]
pub fn apply(device: &Device, profile: &crate::model::Profile) -> Result<String, String> {
    profile.validate()?;
    let id = device
        .ratbag_id
        .as_ref()
        .ok_or("Choose a ratbagd device to apply settings")?;
    if !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err("Invalid device identifier".into());
    }
    // Each operation is passed as argv. No shell is used and no untrusted text becomes a command.
    for (kind, value) in [("dpi", profile.dpi), ("rate", profile.report_rate_ms)] {
        let out = Command::new("ratbagctl")
            .arg(id)
            .args([kind, "set", &value.to_string()])
            .output()
            .map_err(|e| e.to_string())?;
        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).trim().to_owned());
        }
    }
    Ok("DPI and report interval sent to ratbagd".into())
}

#[cfg(not(target_os = "linux"))]
pub fn apply(_device: &Device, _profile: &crate::model::Profile) -> Result<String, String> {
    Err("Hardware configuration is not yet implemented on this platform".into())
}
