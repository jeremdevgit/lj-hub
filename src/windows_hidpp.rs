//! Conservative HID++ 2.0 controls for directly connected Windows devices.
//! Only feature-advertised commands are sent, and writes are read back.

use crate::device::Device;
use hidapi::HidDevice;

const INDEX: u8 = 0xff; // Corded device. Receiver children need separate pairing discovery.
const SOFTWARE_ID: u8 = 0x08;

struct Hidpp {
    hid: HidDevice,
}

impl Hidpp {
    fn open(device: &Device) -> Result<Self, String> {
        let path = device
            .hid_path
            .as_ref()
            .ok_or("No HID++ interface for this device")?;
        let api = hidapi::HidApi::new().map_err(|e| e.to_string())?;
        let hid = api
            .open_path(path)
            .map_err(|e| format!("Open HID++: {e}"))?;
        Ok(Self { hid })
    }

    fn command(&self, feature: u8, function: u8, params: &[u8]) -> Result<Vec<u8>, String> {
        if params.len() > 16 || function > 0x0f {
            return Err("Invalid HID++ request".into());
        }
        let mut last_error = String::from("HID++ device did not respond");
        // Some devices only accept long requests; try both report types.
        for report_id in [0x10, 0x11] {
            if report_id == 0x10 && params.len() > 3 {
                continue;
            }
            let len = if report_id == 0x10 { 7 } else { 20 };
            let mut request = [0u8; 20];
            request[..4].copy_from_slice(&[
                report_id,
                INDEX,
                feature,
                (function << 4) | SOFTWARE_ID,
            ]);
            request[4..4 + params.len()].copy_from_slice(params);
            match self.hid.write(&request[..len]) {
                Ok(n) if n == len => {}
                Ok(_) => {
                    last_error = "HID++ write was incomplete".into();
                    continue;
                }
                Err(e) => {
                    last_error = format!("HID++ write: {e}");
                    continue;
                }
            }
            for _ in 0..4 {
                let mut response = [0u8; 20];
                let size = self
                    .hid
                    .read_timeout(&mut response, 250)
                    .map_err(|e| format!("HID++ read: {e}"))?;
                if size < 7 || !matches!(response[0], 0x10 | 0x11) || response[1] != INDEX {
                    continue;
                }
                if matches!(response[2], 0x8f | 0xff)
                    && response[3] == feature
                    && response[4] == request[3]
                {
                    return Err(format!("HID++ device error 0x{:02X}", response[5]));
                }
                if response[2] == feature && response[3] == request[3] {
                    return Ok(response[4..size].to_vec());
                }
            }
        }
        Err(format!("{last_error}; this interface may be unsupported"))
    }

    fn feature(&self, id: u16) -> Result<u8, String> {
        let response = self.command(0, 0, &id.to_be_bytes())?;
        let index = *response.first().ok_or("Empty HID++ feature response")?;
        if index == 0 {
            return Err(format!(
                "Device does not advertise HID++ feature 0x{id:04X}"
            ));
        }
        Ok(index)
    }
}

pub fn apply_dpi(device: &Device, dpi: u32) -> Result<String, String> {
    let dpi = u16::try_from(dpi).map_err(|_| "DPI is out of range")?;
    let hidpp = Hidpp::open(device)?;
    let feature = hidpp.feature(0x2201)?;
    let count = *hidpp
        .command(feature, 0, &[])?
        .first()
        .ok_or("Missing sensor count")?;
    if count == 0 {
        return Err("Device has no adjustable DPI sensor".into());
    }
    let sensor = 0u8;
    let list = hidpp.command(feature, 1, &[sensor])?;
    let supported = supported_dpi(&list, dpi);
    if !supported {
        return Err(format!("DPI {dpi} is not advertised by this sensor"));
    }
    hidpp.command(feature, 3, &[sensor, (dpi >> 8) as u8, dpi as u8])?;
    let readback = hidpp.command(feature, 2, &[sensor])?;
    if readback.len() < 3 || u16::from_be_bytes([readback[1], readback[2]]) != dpi {
        return Err("DPI write could not be verified".into());
    }
    Ok(format!("DPI {dpi} applied and verified"))
}

fn supported_dpi(response: &[u8], dpi: u16) -> bool {
    if response.len() < 3 {
        return false;
    }
    let mut values = Vec::new();
    let mut step = None;
    for pair in response[1..].chunks_exact(2) {
        let value = u16::from_be_bytes([pair[0], pair[1]]);
        if value == 0 {
            break;
        }
        if value > 0xe000 {
            step = Some(value - 0xe000);
        } else {
            values.push(value);
        }
    }
    if values.contains(&dpi) {
        return true;
    }
    let (Some(step), Some(min), Some(max)) = (step, values.iter().min(), values.iter().max())
    else {
        return false;
    };
    step != 0 && dpi >= *min && dpi <= *max && (dpi - *min) % step == 0
}

pub fn apply_rate(device: &Device, ms: u32) -> Result<String, String> {
    let ms = u8::try_from(ms).map_err(|_| "Report interval is out of range")?;
    let hidpp = Hidpp::open(device)?;
    let feature = hidpp.feature(0x8060)?;
    let flags = *hidpp
        .command(feature, 0, &[])?
        .first()
        .ok_or("Missing report rate list")?;
    if !(1..=8).contains(&ms) || flags & (1 << (ms - 1)) == 0 {
        return Err(format!("Report interval {ms} ms is not advertised"));
    }
    hidpp.command(feature, 2, &[ms])?;
    let readback = hidpp.command(feature, 1, &[0])?;
    if readback.first() != Some(&ms) {
        return Err("Report interval write could not be verified".into());
    }
    Ok(format!("Report interval {ms} ms applied and verified"))
}

#[cfg(test)]
mod tests {
    use super::supported_dpi;

    #[test]
    fn explicit_and_step_dpi_require_advertised_values() {
        assert!(supported_dpi(&[0, 0x03, 0x20, 0x06, 0x40, 0, 0], 800));
        assert!(!supported_dpi(&[0, 0x03, 0x20, 0x06, 0x40, 0, 0], 900));
        assert!(supported_dpi(&[0, 0, 100, 0x06, 0x40, 0xe0, 50, 0, 0], 800));
        assert!(!supported_dpi(
            &[0, 0, 100, 0x06, 0x40, 0xe0, 50, 0, 0],
            825
        ));
    }
}
