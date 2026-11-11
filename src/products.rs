use crate::{catalog, device, inventory};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Mouse,
    Keyboard,
    Headset,
    Camera,
    Wheel,
    Receiver,
    Other,
}

impl Kind {
    pub fn key(self) -> &'static str {
        match self {
            Self::Mouse => "kind_mouse",
            Self::Keyboard => "kind_keyboard",
            Self::Headset => "kind_headset",
            Self::Camera => "kind_camera",
            Self::Wheel => "kind_wheel",
            Self::Receiver => "kind_receiver",
            Self::Other => "kind_other",
        }
    }
}

#[derive(Clone)]
pub struct Product {
    pub name: String,
    pub id: String,
    pub kind: Kind,
    pub connection: String,
    pub hardware_index: Option<usize>,
}

pub fn classify(name: &str, class: &str, catalog_kind: Option<&str>) -> Kind {
    let name = name.to_ascii_lowercase();
    let class = class.to_ascii_lowercase();
    if name.contains("webcam") || name.contains("camera") || class == "camera" || class == "image" {
        Kind::Camera
    } else if name.contains("wheel") || name.contains("volant") || name.contains("pedal") {
        Kind::Wheel
    } else if name.contains("headset")
        || name.contains("microphone")
        || name.contains("speaker")
        || class == "audioendpoint"
        || class == "media"
    {
        Kind::Headset
    } else if name.contains("keyboard") || class == "keyboard" {
        Kind::Keyboard
    } else if name.contains("receiver") || name.contains("lightspeed") && name.contains("usb") {
        Kind::Receiver
    } else if name.contains("mouse") || name.contains("g502") || catalog_kind == Some("mouse") {
        Kind::Mouse
    } else {
        Kind::Other
    }
}

pub fn connected(
    devices: &[device::Device],
    inventory: &[inventory::InventoryDevice],
    catalog: &[catalog::CatalogDevice],
) -> Vec<Product> {
    let mut result = Vec::new();
    let mut hid_ids = HashSet::new();
    for (index, d) in devices.iter().enumerate() {
        let id = d.id.split(' ').next().unwrap_or(&d.id).to_owned();
        hid_ids.insert(id.clone());
        let kind = id
            .split(':')
            .nth(1)
            .and_then(|part| u16::from_str_radix(part, 16).ok())
            .and_then(|id| catalog::usb_entry(catalog, id))
            .map(|entry| entry.kind.as_str());
        result.push(Product {
            name: d.label.clone(),
            id: d.id.clone(),
            kind: classify(&d.label, "", kind),
            connection: "HID / USB".into(),
            hardware_index: Some(index),
        });
    }
    let mut inventory_ids = HashSet::new();
    for d in inventory {
        if hid_ids.contains(&d.id) || !inventory_ids.insert(d.id.clone()) {
            continue;
        }
        result.push(Product {
            name: d.name.clone(),
            id: d.id.clone(),
            kind: classify(&d.name, &d.kind, None),
            connection: d.kind.clone(),
            hardware_index: None,
        });
    }
    result.sort_by(|a, b| (&a.name, &a.id).cmp(&(&b.name, &b.id)));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categories_cover_non_hid_products() {
        assert_eq!(classify("Brio", "Camera", None), Kind::Camera);
        assert_eq!(classify("G923 Racing Wheel", "HIDClass", None), Kind::Wheel);
        assert_eq!(classify("PRO X Headset", "MEDIA", None), Kind::Headset);
        assert_eq!(classify("G502 HERO", "", None), Kind::Mouse);
    }

    #[test]
    fn duplicate_pnp_interfaces_become_one_card() {
        let items = vec![
            inventory::InventoryDevice {
                name: "G923 Racing Wheel".into(),
                id: "046D:C26E".into(),
                kind: "HIDClass".into(),
            },
            inventory::InventoryDevice {
                name: "G923 Racing Wheel".into(),
                id: "046D:C26E".into(),
                kind: "USB".into(),
            },
        ];
        let cards = connected(&[], &items, &[]);
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].kind, Kind::Wheel);
    }
}
