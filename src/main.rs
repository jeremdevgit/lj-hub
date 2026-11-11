#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod catalog;
mod dashboard;
mod device;
mod i18n;
mod inventory;
mod model;
mod products;
#[cfg(target_os = "windows")]
mod windows_hidpp;

use eframe::egui;
use model::Saved;
use std::{fs, path::PathBuf, sync::mpsc};

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
    devices_rx: Option<mpsc::Receiver<Result<Vec<device::Device>, String>>>,
    action_rx: Option<mpsc::Receiver<Result<String, String>>>,
    inventory_status: String,
    catalog: Vec<catalog::CatalogDevice>,
    catalog_query: String,
    selected_profile: usize,
    selected_product: Option<String>,
    page: dashboard::Page,
    backup_text: String,
    status: String,
    settings_load_error: Option<String>,
    path: Option<PathBuf>,
    capture: Option<Capture>,
    cjk_font_loaded: bool,
}

struct Capture {
    path: PathBuf,
    frames: u32,
    requested: bool,
}

impl App {
    fn new() -> Self {
        let demo = std::env::var_os("LJHUB_DEMO").is_some();
        let path = if demo { None } else { data_path() };
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
        let catalog = catalog::entries();
        let inventory_rx = (!demo).then(start_inventory_scan);
        let status = settings_load_error.clone().unwrap_or_default();
        let mut data = data;
        if let Ok(lang) = std::env::var("LJHUB_LANG") {
            if i18n::LANGUAGES.iter().any(|(code, _)| *code == lang) {
                data.language = lang;
            }
        }
        let page = match std::env::var("LJHUB_PAGE").as_deref() {
            Ok("device") => dashboard::Page::Device,
            Ok("profiles") => dashboard::Page::Profiles,
            Ok("catalog") => dashboard::Page::Catalog,
            Ok("settings") => dashboard::Page::Settings,
            _ => dashboard::Page::Home,
        };
        let capture = std::env::var_os("LJHUB_SCREENSHOT").map(|p| Capture {
            path: PathBuf::from(p),
            frames: 0,
            requested: false,
        });
        Self {
            data,
            devices: Vec::new(),
            inventory: if demo { demo_inventory() } else { Vec::new() },
            inventory_rx,
            devices_rx: (!demo).then(start_device_scan),
            action_rx: None,
            inventory_status: String::new(),
            catalog,
            catalog_query: String::new(),
            selected_profile: 0,
            selected_product: None,
            page,
            backup_text: String::new(),
            status,
            settings_load_error,
            path,
            capture,
            cjk_font_loaded: false,
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
        self.devices_rx = Some(start_device_scan());
        self.inventory_rx = Some(start_inventory_scan());
        self.inventory_status.clear();
    }
}

// macOS hidapi pumps the CFRunLoop while enumerating, which re-enters winit on the main thread.
fn start_device_scan() -> mpsc::Receiver<Result<Vec<device::Device>, String>> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(device::discover());
    });
    rx
}

fn demo_inventory() -> Vec<inventory::InventoryDevice> {
    [
        ("G502 X PLUS Gaming Mouse", "046D:C095"),
        ("G915 TKL Wireless Keyboard", "046D:C343"),
        ("PRO X 2 Headset", "046D:0AF7"),
        ("Brio 4K Webcam", "046D:085E"),
        ("G923 Racing Wheel", "046D:C266"),
        ("Lightspeed USB Receiver", "046D:C547"),
    ]
    .into_iter()
    .map(|(name, id)| inventory::InventoryDevice {
        name: name.into(),
        id: id.into(),
        kind: "USB".into(),
    })
    .collect()
}

fn start_inventory_scan() -> mpsc::Receiver<Result<Vec<inventory::InventoryDevice>, String>> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(inventory::discover());
    });
    rx
}

impl App {
    fn capture_frame(&mut self, ctx: &egui::Context) {
        let Some(capture) = self.capture.as_mut() else {
            return;
        };
        let shot = ctx.input(|i| {
            i.raw.events.iter().find_map(|e| match e {
                egui::Event::Screenshot { image, .. } => Some(image.clone()),
                _ => None,
            })
        });
        if let Some(image) = shot {
            let [w, h] = image.size;
            let rgba: Vec<u8> = image.pixels.iter().flat_map(|c| c.to_array()).collect();
            if let Err(e) = image::save_buffer(
                &capture.path,
                &rgba,
                w as u32,
                h as u32,
                image::ExtendedColorType::Rgba8,
            ) {
                eprintln!("screenshot failed: {e}");
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }
        capture.frames += 1;
        if capture.frames > 20 && !capture.requested {
            capture.requested = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
        }
        ctx.request_repaint();
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        dashboard::render(self, ctx);
        self.capture_frame(ctx);
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1180.0, 760.0])
            .with_min_inner_size([760.0, 540.0]),
        ..Default::default()
    };
    eframe::run_native(
        "LJ Hub",
        options,
        Box::new(|cc| {
            dashboard::theme(&cc.egui_ctx);
            Ok(Box::new(App::new()))
        }),
    )
}

#[cfg(all(test, target_os = "windows"))]
mod windows_tests {
    use super::*;

    #[test]
    fn profiles_can_be_saved_more_than_once() {
        let dir = std::env::temp_dir().join(format!("lj-hub-test-{}", std::process::id()));
        let path = dir.join("profiles.json");
        let mut data = Saved::default();
        save(&path, &data).unwrap();
        data.profiles[0].dpi = 1600;
        save(&path, &data).unwrap();
        assert_eq!(load(&path).unwrap().profiles[0].dpi, 1600);
        fs::remove_dir_all(dir).unwrap();
    }
}
