use std::collections::{HashMap, HashSet};
#[cfg(target_os = "windows")]
use std::ffi::CString;
#[cfg(target_os = "linux")]
use std::process::Command;

#[derive(Clone)]
pub struct Device {
    pub label: String,
    pub id: String,
    #[cfg(target_os = "linux")]
    pub ratbag_id: Option<String>,
    pub catalog_driver: Option<String>,
    #[cfg(target_os = "windows")]
    pub hid_path: Option<CString>,
}

pub fn discover() -> Result<Vec<Device>, String> {
    let api = hidapi::HidApi::new().map_err(|e| e.to_string())?;
    let catalog = crate::catalog::entries();
    let mut devices: Vec<Device> = Vec::new();
    let mut seen = HashSet::new();
    #[cfg(target_os = "windows")]
    let mut seen_indices = HashMap::<String, usize>::new();
    let mut counts = HashMap::<String, usize>::new();
    for d in api.device_list().filter(|d| d.vendor_id() == 0x046d) {
        let product = format!("{:04X}:{:04X}", d.vendor_id(), d.product_id());
        let identity = if let Some(serial) = d.serial_number().filter(|s| !s.is_empty()) {
            format!("{product}:serial:{serial}")
        } else {
            format!("{product}:path:{}", d.path().to_string_lossy())
        };
        #[cfg(target_os = "windows")]
        if let Some(&index) = seen_indices.get(&identity) {
            if devices[index].hid_path.is_none()
                && (d.usage_page() == 0xff00 || d.usage_page() == 0xff43)
                && d.usage() != 0
            {
                devices[index].hid_path = Some(d.path().to_owned());
            }
            continue;
        }
        if !seen.insert(identity.clone()) {
            continue;
        }
        #[cfg(target_os = "windows")]
        seen_indices.insert(identity, devices.len());
        let count = counts
            .entry(product.clone())
            .and_modify(|n| *n += 1)
            .or_insert(1);
        let id = format!("{product} #{count}");
        let known = crate::catalog::usb_entry(&catalog, d.product_id());
        devices.push(Device {
            label: d
                .product_string()
                .or_else(|| known.map(|entry| entry.name.as_str()))
                .unwrap_or("Logitech device")
                .to_owned(),
            id,
            #[cfg(target_os = "linux")]
            ratbag_id: None,
            catalog_driver: known.map(|entry| entry.driver.clone()),
            #[cfg(target_os = "windows")]
            hid_path: ((d.usage_page() == 0xff00 || d.usage_page() == 0xff43) && d.usage() != 0)
                .then(|| d.path().to_owned()),
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
                        catalog_driver: None,
                    });
                }
            }
        }
    }
    Ok(devices)
}

pub fn supports_hardware(device: &Device) -> bool {
    #[cfg(target_os = "linux")]
    return device.ratbag_id.is_some();
    #[cfg(target_os = "windows")]
    return device.hid_path.is_some();
    #[cfg(target_os = "macos")]
    {
        let _ = device;
        false
    }
}

#[cfg(target_os = "linux")]
fn ratbag(device: &Device, args: &[&str]) -> Result<String, String> {
    let id = device.ratbag_id.as_ref().ok_or("Choose a ratbagd device")?;
    if !valid_ratbag_id(id) {
        return Err("Invalid device identifier".into());
    }
    let out = Command::new("ratbagctl")
        .arg(id)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        let error = String::from_utf8_lossy(&out.stderr).trim().to_owned();
        return Err(if error.is_empty() {
            format!("ratbagctl exited with {}", out.status)
        } else {
            error
        });
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

#[cfg(target_os = "linux")]
fn valid_ratbag_id(id: &str) -> bool {
    !id.is_empty() && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

#[cfg(target_os = "linux")]
pub fn apply_dpi(device: &Device, profile: &crate::model::Profile) -> Result<String, String> {
    profile.validate_dpi()?;
    ratbag(device, &["dpi", "set", &profile.dpi.to_string()])?;
    Ok(format!("DPI {} sent to ratbagd", profile.dpi))
}

#[cfg(target_os = "linux")]
pub fn apply_rate(device: &Device, profile: &crate::model::Profile) -> Result<String, String> {
    profile.validate_rate()?;
    ratbag(
        device,
        &["rate", "set", &profile.report_rate_ms.to_string()],
    )?;
    Ok(format!(
        "Report interval {} ms sent to ratbagd",
        profile.report_rate_ms
    ))
}

#[cfg(target_os = "linux")]
pub fn apply_color(
    device: &Device,
    profile: &crate::model::Profile,
    rgb_enabled: bool,
) -> Result<String, String> {
    if !rgb_enabled {
        return Err("RGB control is disabled for rJb".into());
    }
    profile.validate_color()?;
    ratbag(device, &["led", "0", "get"])?;
    ratbag(device, &["led", "0", "color", &profile.color])?;
    Ok(format!("RGB {} sent to LED 0 via ratbagd", profile.color))
}

#[cfg(target_os = "windows")]
pub fn apply_dpi(device: &Device, profile: &crate::model::Profile) -> Result<String, String> {
    profile.validate_dpi()?;
    crate::windows_hidpp::apply_dpi(device, profile.dpi)
}

#[cfg(target_os = "windows")]
pub fn apply_rate(device: &Device, profile: &crate::model::Profile) -> Result<String, String> {
    profile.validate_rate()?;
    crate::windows_hidpp::apply_rate(device, profile.report_rate_ms)
}

#[cfg(target_os = "macos")]
pub fn apply_dpi(_device: &Device, _profile: &crate::model::Profile) -> Result<String, String> {
    Err("Hardware configuration is not yet implemented on this platform".into())
}

#[cfg(target_os = "macos")]
pub fn apply_rate(_device: &Device, _profile: &crate::model::Profile) -> Result<String, String> {
    Err("Hardware configuration is not yet implemented on this platform".into())
}

#[cfg(not(target_os = "linux"))]
pub fn apply_color(
    _device: &Device,
    _profile: &crate::model::Profile,
    _rgb_enabled: bool,
) -> Result<String, String> {
    Err("Hardware configuration is not yet implemented on this platform".into())
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn disabled_rgb_never_reaches_ratbagctl() {
        let device = Device {
            label: "test".into(),
            id: "test".into(),
            ratbag_id: Some("test-device".into()),
            catalog_driver: None,
        };
        assert_eq!(
            apply_color(&device, &crate::model::Profile::default(), false).unwrap_err(),
            "RGB control is disabled for rJb"
        );
    }
}
