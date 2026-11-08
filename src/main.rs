#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod catalog;
mod device;
mod i18n;
mod inventory;
mod model;

use eframe::egui;
use model::{Profile, Saved};
use std::{fs, path::PathBuf, sync::mpsc, time::Duration};

fn data_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "jeremdevgit", "LJ Hub")
        .map(|d| d.config_dir().join("profiles.json"))
}

fn load(path: &PathBuf) -> Result<Saved, String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    if bytes.len() > 1_000_000 {
        return Err("Profile file exceeds 1 MB".into());
    }
    let data: Saved = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    data.validate()?;
    Ok(data)
}

fn save(path: &PathBuf, data: &Saved) -> Result<(), String> {
    data.validate()?;
    let parent = path.parent().ok_or("Invalid settings path")?;
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let tmp = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(data).map_err(|e| e.to_string())?;
    fs::write(&tmp, bytes).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| e.to_string())?;
    Ok(())
}

struct App {
    data: Saved,
    devices: Vec<device::Device>,
    inventory: Vec<inventory::InventoryDevice>,
    inventory_rx: Option<mpsc::Receiver<Result<Vec<inventory::InventoryDevice>, String>>>,
    inventory_status: String,
    catalog: Vec<catalog::CatalogDevice>,
    catalog_query: String,
    selected_profile: usize,
    selected_device: usize,
    status: String,
    settings_load_error: Option<String>,
    path: Option<PathBuf>,
}

impl App {
    fn new() -> Self {
        let path = data_path();
        let (data, settings_load_error) = match path.as_ref() {
            Some(p) if p.exists() => match load(p) {
                Ok(data) => (data, None),
                Err(e) => (
                    Saved::default(),
                    Some(format!("Could not load settings: {e}")),
                ),
            },
            _ => (Saved::default(), None),
        };
        let devices = device::discover().unwrap_or_default();
        let catalog = catalog::entries();
        let inventory_rx = Some(start_inventory_scan());
        Self {
            data,
            devices,
            inventory: Vec::new(),
            inventory_rx,
            inventory_status: String::new(),
            catalog,
            catalog_query: String::new(),
            selected_profile: 0,
            selected_device: 0,
            status: String::new(),
            settings_load_error,
            path,
        }
    }
    fn persist(&mut self) {
        if let Some(error) = &self.settings_load_error {
            self.status =
                format!("{error}. Existing file was preserved; correct it before saving.");
            return;
        }
        self.status = match &self.path {
            Some(p) => match save(p, &self.data) {
                Ok(()) => "Saved locally".into(),
                Err(e) => e,
            },
            None => "Cannot locate configuration directory".into(),
        };
    }

    fn refresh_inventory(&mut self) {
        self.inventory_rx = Some(start_inventory_scan());
        self.inventory_status.clear();
    }
}

fn start_inventory_scan() -> mpsc::Receiver<Result<Vec<inventory::InventoryDevice>, String>> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(inventory::discover());
    });
    rx
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(rx) = &self.inventory_rx {
            match rx.try_recv() {
                Ok(Ok(items)) => {
                    self.inventory = items;
                    self.inventory_rx = None;
                }
                Ok(Err(error)) => {
                    self.inventory_status = error;
                    self.inventory_rx = None;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.inventory_status = "Inventory scan stopped".into();
                    self.inventory_rx = None;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    ctx.request_repaint_after(Duration::from_millis(250))
                }
            }
        }
        let lang = self.data.language.clone();
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("LJ Hub");
                ui.separator();
                egui::ComboBox::from_id_salt("language")
                    .selected_text(
                        i18n::LANGUAGES
                            .iter()
                            .find(|(c, _)| *c == lang)
                            .map(|(_, n)| *n)
                            .unwrap_or("Français"),
                    )
                    .show_ui(ui, |ui| {
                        for (code, name) in i18n::LANGUAGES {
                            ui.selectable_value(&mut self.data.language, code.into(), name);
                        }
                    });
                if self.data.language != lang {
                    self.persist();
                }
            });
        });
        egui::SidePanel::left("devices")
            .min_width(240.0)
            .show(ctx, |ui| {
                ui.heading(i18n::t(&lang, "devices"));
                if ui.button(i18n::t(&lang, "refresh")).clicked() {
                    match device::discover() {
                        Ok(devices) => {
                            self.devices = devices;
                            self.selected_device = 0;
                            self.refresh_inventory();
                        }
                        Err(e) => self.status = e,
                    }
                }
                ui.separator();
                if self.devices.is_empty() {
                    ui.label(i18n::t(&lang, "no_device"));
                }
                for (i, d) in self.devices.iter().enumerate() {
                    let response = ui.selectable_value(
                        &mut self.selected_device,
                        i,
                        format!("{} ({})", d.label, d.id),
                    );
                    if let Some(driver) = &d.catalog_driver {
                        response.on_hover_text(format!("libratbag: {driver}"));
                    }
                }
                ui.separator();
                ui.heading(i18n::t(&lang, "system_inventory"));
                ui.small(i18n::t(&lang, "inventory_note"));
                if self.inventory_rx.is_some() {
                    ui.label(i18n::t(&lang, "scanning"));
                }
                if !self.inventory_status.is_empty() {
                    ui.small(&self.inventory_status);
                }
                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .show(ui, |ui| {
                        for d in &self.inventory {
                            ui.label(format!("{} · {} · {}", d.name, d.id, d.kind));
                        }
                    });
                ui.separator();
                ui.small(if cfg!(target_os = "linux") {
                    i18n::t(&lang, "linux_note")
                } else {
                    i18n::t(&lang, "desktop_note")
                });
                ui.separator();
                egui::CollapsingHeader::new(i18n::t(&lang, "catalog")).show(ui, |ui| {
                    ui.small(i18n::t(&lang, "catalog_note"));
                    ui.text_edit_singleline(&mut self.catalog_query);
                    let query = self.catalog_query.to_lowercase();
                    egui::ScrollArea::vertical()
                        .max_height(240.0)
                        .show(ui, |ui| {
                            for entry in self.catalog.iter().filter(|e| {
                                query.is_empty()
                                    || e.name.to_lowercase().contains(&query)
                                    || e.product_id.to_lowercase().contains(&query)
                            }) {
                                ui.label(format!(
                                    "{} · {} · {} · {}",
                                    entry.name, entry.transport, entry.product_id, entry.kind
                                ));
                            }
                        });
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(i18n::t(&lang, "profiles"));
            ui.horizontal(|ui| {
                if ui.button(i18n::t(&lang, "add")).clicked() && self.data.profiles.len() < 100 {
                    let p = Profile {
                        name: format!("Profile {}", self.data.profiles.len() + 1),
                        ..Default::default()
                    };
                    self.data.profiles.push(p);
                    self.selected_profile = self.data.profiles.len() - 1;
                    self.persist();
                }
                if ui.button(i18n::t(&lang, "delete")).clicked() && self.data.profiles.len() > 1 {
                    self.data.profiles.remove(self.selected_profile);
                    self.selected_profile = 0;
                    self.persist();
                }
            });
            ui.horizontal_wrapped(|ui| {
                for (i, p) in self.data.profiles.iter().enumerate() {
                    ui.selectable_value(&mut self.selected_profile, i, &p.name);
                }
            });
            ui.separator();
            let p = &mut self.data.profiles[self.selected_profile];
            egui::Grid::new("profile_grid")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label(i18n::t(&lang, "name"));
                    ui.text_edit_singleline(&mut p.name);
                    ui.end_row();
                    ui.label("DPI");
                    ui.add(egui::DragValue::new(&mut p.dpi).range(100..=32000));
                    ui.end_row();
                    ui.label(i18n::t(&lang, "rate"));
                    egui::ComboBox::from_id_salt("rate")
                        .selected_text(format!("{} ms", p.report_rate_ms))
                        .show_ui(ui, |ui| {
                            for ms in [1, 2, 4, 8] {
                                ui.selectable_value(
                                    &mut p.report_rate_ms,
                                    ms,
                                    format!("{} ms", ms),
                                );
                            }
                        });
                    ui.end_row();
                    ui.label(i18n::t(&lang, "color"));
                    ui.text_edit_singleline(&mut p.color);
                    ui.end_row();
                });
            ui.label(i18n::t(&lang, "notes"));
            ui.text_edit_multiline(&mut p.notes);
            ui.horizontal(|ui| {
                if ui.button(i18n::t(&lang, "save")).clicked() {
                    self.persist();
                }
            });
            let can_apply = self
                .devices
                .get(self.selected_device)
                .is_some_and(|d| d.ratbag_id.is_some());
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add_enabled(can_apply, egui::Button::new(i18n::t(&lang, "apply_dpi")))
                    .clicked()
                {
                    self.status = format_result(device::apply_dpi(
                        &self.devices[self.selected_device],
                        &self.data.profiles[self.selected_profile],
                    ));
                }
                if ui
                    .add_enabled(can_apply, egui::Button::new(i18n::t(&lang, "apply_rate")))
                    .clicked()
                {
                    self.status = format_result(device::apply_rate(
                        &self.devices[self.selected_device],
                        &self.data.profiles[self.selected_profile],
                    ));
                }
            });
            if ui
                .checkbox(
                    &mut self.data.rgb_control_enabled,
                    i18n::t(&lang, "rgb_control"),
                )
                .changed()
            {
                self.persist();
            }
            ui.small(i18n::t(&lang, "rgb_note"));
            if ui
                .add_enabled(
                    can_apply && self.data.rgb_control_enabled,
                    egui::Button::new(i18n::t(&lang, "apply_rgb")),
                )
                .clicked()
            {
                self.status = format_result(device::apply_color(
                    &self.devices[self.selected_device],
                    &self.data.profiles[self.selected_profile],
                    self.data.rgb_control_enabled,
                ));
            }
            ui.separator();
            ui.label(format!("{}: {}", i18n::t(&lang, "status"), self.status));
        });
    }
}

fn format_result(result: Result<String, String>) -> String {
    match result {
        Ok(message) => message,
        Err(error) => format!("Error: {error}"),
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "LJ Hub",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(App::new()))),
    )
}
