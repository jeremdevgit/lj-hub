mod device;
mod i18n;
mod model;

use eframe::egui;
use model::{Profile, Saved};
use std::{fs, path::PathBuf};

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
    selected_profile: usize,
    selected_device: usize,
    status: String,
    path: Option<PathBuf>,
}

impl App {
    fn new() -> Self {
        let path = data_path();
        let (data, status) = match path.as_ref() {
            Some(p) if p.exists() => match load(p) {
                Ok(data) => (data, String::new()),
                Err(e) => (Saved::default(), format!("Could not load settings: {e}")),
            },
            _ => (Saved::default(), String::new()),
        };
        let devices = device::discover().unwrap_or_default();
        Self {
            data,
            devices,
            selected_profile: 0,
            selected_device: 0,
            status,
            path,
        }
    }
    fn persist(&mut self) {
        self.status = match &self.path {
            Some(p) => match save(p, &self.data) {
                Ok(()) => "Saved locally".into(),
                Err(e) => e,
            },
            None => "Cannot locate configuration directory".into(),
        };
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                        }
                        Err(e) => self.status = e,
                    }
                }
                ui.separator();
                if self.devices.is_empty() {
                    ui.label(i18n::t(&lang, "no_device"));
                }
                for (i, d) in self.devices.iter().enumerate() {
                    ui.selectable_value(
                        &mut self.selected_device,
                        i,
                        format!("{} ({})", d.label, d.id),
                    );
                }
                ui.separator();
                ui.small(if cfg!(target_os = "linux") {
                    i18n::t(&lang, "linux_note")
                } else {
                    i18n::t(&lang, "desktop_note")
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
                if ui
                    .add_enabled(
                        self.devices
                            .get(self.selected_device)
                            .is_some_and(|d| d.ratbag_id.is_some()),
                        egui::Button::new(i18n::t(&lang, "apply")),
                    )
                    .clicked()
                {
                    self.status = match device::apply(
                        &self.devices[self.selected_device],
                        &self.data.profiles[self.selected_profile],
                    ) {
                        Ok(s) | Err(s) => s,
                    };
                }
            });
            ui.separator();
            ui.label(format!("{}: {}", i18n::t(&lang, "status"), self.status));
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "LJ Hub",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(App::new()))),
    )
}
