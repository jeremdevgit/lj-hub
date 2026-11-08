use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct CatalogDevice {
    pub name: String,
    pub transport: String,
    pub product_id: String,
    pub kind: String,
    pub driver: String,
}

#[derive(Deserialize)]
struct CatalogFile {
    devices: Vec<CatalogDevice>,
}

pub fn entries() -> Vec<CatalogDevice> {
    serde_json::from_str::<CatalogFile>(include_str!("../data/libratbag-logitech.json"))
        .map(|file| file.devices)
        .unwrap_or_default()
}

pub fn usb_entry(entries: &[CatalogDevice], product_id: u16) -> Option<&CatalogDevice> {
    let id = format!("{product_id:04X}");
    entries
        .iter()
        .find(|entry| entry.transport == "usb" && entry.product_id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_is_unique_and_contains_logitech_g502_hero() {
        let entries = entries();
        assert!(entries.len() >= 100);
        assert!(usb_entry(&entries, 0xC08B).is_some_and(|d| d.name.contains("G502")));
        let mut ids: Vec<_> = entries
            .iter()
            .map(|d| (&d.transport, &d.product_id))
            .collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), entries.len());
    }
}
