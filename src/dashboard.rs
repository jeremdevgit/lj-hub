use crate::{
    device, i18n,
    model::Saved,
    products::{self, Kind, Product},
    App,
};
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, Sense, Stroke, Vec2};
use std::{sync::mpsc, time::Duration};

const BG: Color32 = Color32::from_rgb(13, 19, 27);
const PANEL: Color32 = Color32::from_rgb(22, 31, 42);
const CARD: Color32 = Color32::from_rgb(29, 42, 55);
const ACCENT: Color32 = Color32::from_rgb(57, 216, 196);
const MUTED: Color32 = Color32::from_rgb(153, 170, 184);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Device,
    Profiles,
    Catalog,
    Settings,
}

impl Page {
    const ALL: [Self; 5] = [
        Self::Home,
        Self::Device,
        Self::Profiles,
        Self::Catalog,
        Self::Settings,
    ];
    fn key(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Device => "device_page",
            Self::Profiles => "profiles",
            Self::Catalog => "catalog",
            Self::Settings => "settings",
        }
    }
    fn mark(self) -> &'static str {
        match self {
            Self::Home => "🏠",
            Self::Device => "🖱",
            Self::Profiles => "📋",
            Self::Catalog => "📚",
            Self::Settings => "⚙",
        }
    }
}

#[derive(Clone, Copy)]
enum Action {
    Dpi,
    Rate,
    Rgb,
}

pub fn theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = BG;
    visuals.window_fill = PANEL;
    visuals.widgets.inactive.bg_fill = CARD;
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(43, 60, 74);
    visuals.selection.bg_fill = Color32::from_rgb(29, 100, 96);
    visuals.hyperlink_color = ACCENT;
    ctx.set_visuals(visuals);
}

pub fn render(app: &mut App, ui: &mut egui::Ui) {
    let ctx = &ui.ctx().clone();
    poll(app, ctx);
    let lang = app.data.language.clone();
    if !app.cjk_font_loaded && (lang == "zh-CN" || lang == "ja") {
        app.cjk_font_loaded = true;
        install_cjk_font(ctx);
    }
    let products = products::connected(&app.devices, &app.inventory, &app.catalog);
    egui::Panel::left("nav")
        .exact_size(190.0)
        .frame(egui::Frame::new().fill(PANEL).inner_margin(16.0))
        .show(ui, |ui| {
            ui.add_space(8.0);
            ui.colored_label(ACCENT, egui::RichText::new("LJ").size(34.0).strong());
            ui.heading("Hub");
            ui.colored_label(MUTED, "0.2 ALPHA");
            ui.add_space(24.0);
            for page in Page::ALL {
                let title = format!("{}   {}", page.mark(), i18n::t(&lang, page.key()));
                if ui
                    .add_sized(
                        [158.0, 38.0],
                        egui::Button::new(title).selected(app.page == page),
                    )
                    .clicked()
                {
                    app.page = page;
                }
                ui.add_space(5.0);
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.colored_label(MUTED, i18n::t(&lang, "local_only"));
                ui.colored_label(ACCENT, "•  rJb RGB");
            });
        });
    egui::Panel::bottom("status")
        .frame(
            egui::Frame::new()
                .fill(PANEL)
                .inner_margin(egui::Margin::symmetric(16, 8)),
        )
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(ACCENT, "•");
                ui.label(if app.status.is_empty() {
                    i18n::t(&lang, "ready")
                } else {
                    &app.status
                });
            });
        });
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(BG).inner_margin(24.0))
        .show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                match app.page {
                    Page::Home => home(app, ui, &products, &lang),
                    Page::Device => device_page(app, ui, &products, &lang),
                    Page::Profiles => profiles_page(app, ui, &lang),
                    Page::Catalog => catalog_page(app, ui, &lang),
                    Page::Settings => settings_page(app, ui, &lang),
                }
            });
        });
}

fn poll(app: &mut App, ctx: &egui::Context) {
    if let Some(rx) = &app.inventory_rx {
        match rx.try_recv() {
            Ok(Ok(items)) => {
                app.inventory = items;
                app.inventory_rx = None;
            }
            Ok(Err(error)) => {
                app.inventory_status = error;
                app.inventory_rx = None;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                app.inventory_status = "Inventory scan stopped".into();
                app.inventory_rx = None;
            }
            Err(mpsc::TryRecvError::Empty) => ctx.request_repaint_after(Duration::from_millis(250)),
        }
    }
    if let Some(rx) = &app.devices_rx {
        match rx.try_recv() {
            Ok(Ok(devices)) => {
                app.devices = devices;
                app.devices_rx = None;
            }
            Ok(Err(error)) => {
                app.status = error;
                app.devices_rx = None;
            }
            Err(mpsc::TryRecvError::Disconnected) => app.devices_rx = None,
            Err(mpsc::TryRecvError::Empty) => ctx.request_repaint_after(Duration::from_millis(250)),
        }
    }
    if let Some(rx) = &app.action_rx {
        match rx.try_recv() {
            Ok(result) => {
                app.status = match result {
                    Ok(message) => message,
                    Err(error) => format!("Error: {error}"),
                };
                app.action_rx = None;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                app.status = "Hardware action stopped".into();
                app.action_rx = None;
            }
            Err(mpsc::TryRecvError::Empty) => ctx.request_repaint_after(Duration::from_millis(100)),
        }
    }
}

fn home(app: &mut App, ui: &mut egui::Ui, products: &[Product], lang: &str) {
    ui.heading(i18n::t(lang, "home_title"));
    ui.colored_label(MUTED, i18n::t(lang, "home_subtitle"));
    ui.add_space(18.0);
    ui.horizontal_wrapped(|ui| {
        metric(ui, products.len(), i18n::t(lang, "connected"));
        metric(ui, app.data.profiles.len(), i18n::t(lang, "profiles"));
        let candidates = products
            .iter()
            .filter(|p| {
                p.hardware_index
                    .is_some_and(|i| device::supports_hardware(&app.devices[i]))
            })
            .count();
        metric(ui, candidates, i18n::t(lang, "control_candidates"));
    });
    ui.add_space(22.0);
    ui.horizontal(|ui| {
        ui.heading(i18n::t(lang, "connected"));
        if ui.button(i18n::t(lang, "refresh")).clicked() {
            refresh(app);
        }
    });
    if app.inventory_rx.is_some() || app.devices_rx.is_some() {
        ui.colored_label(MUTED, i18n::t(lang, "scanning"));
    }
    if !app.inventory_status.is_empty() {
        ui.colored_label(MUTED, &app.inventory_status);
    }
    ui.add_space(8.0);
    if products.is_empty() {
        egui::Frame::new()
            .fill(PANEL)
            .corner_radius(14)
            .inner_margin(28.0)
            .show(ui, |ui| {
                ui.label(i18n::t(lang, "no_device"));
                ui.colored_label(MUTED, i18n::t(lang, "no_connected_note"));
            });
    } else {
        let available = ui.available_width();
        let columns = if available > 850.0 {
            3
        } else if available > 560.0 {
            2
        } else {
            1
        };
        let width = ((available - (columns as f32 - 1.0) * 12.0) / columns as f32).max(180.0);
        egui::Grid::new("connected_grid")
            .num_columns(columns)
            .spacing([12.0, 12.0])
            .show(ui, |ui| {
                for (i, product) in products.iter().enumerate() {
                    let selected = app.selected_product.as_deref() == Some(product.id.as_str());
                    if product_card(ui, product, width, selected, lang).clicked() {
                        app.selected_product = Some(product.id.clone());
                        app.page = Page::Device;
                    }
                    if (i + 1) % columns == 0 {
                        ui.end_row();
                    }
                }
            });
    }
    ui.add_space(22.0);
    egui::Frame::new()
        .fill(PANEL)
        .corner_radius(14)
        .inner_margin(18.0)
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.colored_label(ACCENT, "•");
                ui.label(i18n::t(lang, "rgb_handoff"));
            });
        });
}

fn refresh(app: &mut App) {
    app.refresh_inventory();
}

fn device_page(app: &mut App, ui: &mut egui::Ui, products: &[Product], lang: &str) {
    let product = app
        .selected_product
        .as_ref()
        .and_then(|id| products.iter().find(|p| &p.id == id))
        .or_else(|| products.first())
        .cloned();
    let Some(product) = product else {
        ui.heading(i18n::t(lang, "device_page"));
        ui.label(i18n::t(lang, "no_device"));
        return;
    };
    let candidate = product
        .hardware_index
        .is_some_and(|i| device::supports_hardware(&app.devices[i]));
    ui.horizontal(|ui| {
        if ui.button("⬅").clicked() {
            app.page = Page::Home;
        }
        ui.heading(i18n::t(lang, "device_page"));
    });
    ui.add_space(12.0);
    egui::Frame::new()
        .fill(PANEL)
        .corner_radius(18)
        .inner_margin(22.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::new(180.0, 150.0), Sense::hover());
                ui.painter().rect_filled(rect, 16.0, CARD);
                paint_product(ui.painter(), rect.shrink(28.0), product.kind, ACCENT);
                ui.vertical(|ui| {
                    ui.add_space(12.0);
                    ui.heading(&product.name);
                    ui.colored_label(
                        MUTED,
                        format!(
                            "{} · {}",
                            i18n::t(lang, product.kind.key()),
                            product.connection
                        ),
                    );
                    ui.colored_label(MUTED, &product.id);
                    if let Some(driver) = product
                        .hardware_index
                        .and_then(|i| app.devices[i].catalog_driver.as_ref())
                    {
                        ui.colored_label(MUTED, format!("libratbag: {driver}"));
                    }
                    ui.add_space(14.0);
                    ui.colored_label(
                        if candidate { ACCENT } else { MUTED },
                        if candidate {
                            i18n::t(lang, "experimental")
                        } else {
                            i18n::t(lang, "detection_only")
                        },
                    );
                });
            });
        });
    ui.add_space(20.0);
    ui.heading(i18n::t(lang, "hardware_features"));
    let mouse_control = matches!(product.kind, Kind::Mouse | Kind::Other) && candidate;
    capability(ui, "DPI", mouse_control, lang);
    capability(ui, i18n::t(lang, "rate"), mouse_control, lang);
    capability(
        ui,
        "LIGHTSYNC / RGB",
        cfg!(target_os = "linux") && candidate && app.data.rgb_control_enabled,
        lang,
    );
    capability(ui, i18n::t(lang, "assignments"), false, lang);
    if matches!(product.kind, Kind::Headset | Kind::Camera | Kind::Wheel) {
        capability(ui, i18n::t(lang, "advanced_controls"), false, lang);
    }
    ui.add_space(18.0);
    if mouse_control {
        ui.heading(i18n::t(lang, "sensitivity"));
        ui.horizontal(|ui| {
            ui.label(i18n::t(lang, "profiles"));
            egui::ComboBox::from_id_salt("selected_profile")
                .selected_text(&app.data.profiles[app.selected_profile].name)
                .show_ui(ui, |ui| {
                    for (i, p) in app.data.profiles.iter().enumerate() {
                        ui.selectable_value(&mut app.selected_profile, i, &p.name);
                    }
                });
        });
        let profile = &mut app.data.profiles[app.selected_profile];
        ui.horizontal(|ui| {
            ui.label("DPI");
            ui.add(egui::DragValue::new(&mut profile.dpi).range(100..=32000));
            ui.label(i18n::t(lang, "rate"));
            egui::ComboBox::from_id_salt("device_rate")
                .selected_text(format!("{} ms", profile.report_rate_ms))
                .show_ui(ui, |ui| {
                    for ms in [1, 2, 4, 8] {
                        ui.selectable_value(&mut profile.report_rate_ms, ms, format!("{ms} ms"));
                    }
                });
        });
        ui.horizontal(|ui| {
            if ui.button(i18n::t(lang, "save")).clicked() {
                app.persist();
            }
            if ui
                .add_enabled(
                    app.action_rx.is_none(),
                    egui::Button::new(i18n::t(lang, "apply_dpi")),
                )
                .clicked()
            {
                start_action(app, &product, Action::Dpi);
            }
            if ui
                .add_enabled(
                    app.action_rx.is_none(),
                    egui::Button::new(i18n::t(lang, "apply_rate")),
                )
                .clicked()
            {
                start_action(app, &product, Action::Rate);
            }
        });
    } else {
        ui.colored_label(MUTED, i18n::t(lang, "detection_explanation"));
    }
    if cfg!(target_os = "linux") && candidate && app.data.rgb_control_enabled {
        ui.add_space(10.0);
        if ui
            .add_enabled(
                app.action_rx.is_none(),
                egui::Button::new(i18n::t(lang, "apply_rgb")),
            )
            .clicked()
        {
            start_action(app, &product, Action::Rgb);
        }
    }
}

fn start_action(app: &mut App, product: &Product, action: Action) {
    let Some(index) = product.hardware_index else {
        return;
    };
    let Some(device) = app.devices.get(index).cloned() else {
        return;
    };
    let profile = app.data.profiles[app.selected_profile].clone();
    let rgb = app.data.rgb_control_enabled;
    let (tx, rx) = mpsc::channel();
    app.action_rx = Some(rx);
    app.status = "Applying…".into();
    std::thread::spawn(move || {
        let result = match action {
            Action::Dpi => device::apply_dpi(&device, &profile),
            Action::Rate => device::apply_rate(&device, &profile),
            Action::Rgb => device::apply_color(&device, &profile, rgb),
        };
        let _ = tx.send(result);
    });
}

fn profiles_page(app: &mut App, ui: &mut egui::Ui, lang: &str) {
    ui.heading(i18n::t(lang, "profiles"));
    ui.colored_label(MUTED, i18n::t(lang, "profile_subtitle"));
    ui.add_space(14.0);
    ui.horizontal(|ui| {
        if ui.button(format!("➕ {}", i18n::t(lang, "add"))).clicked()
            && app.data.profiles.len() < 100
        {
            app.data.profiles.push(crate::model::Profile {
                name: format!("Profile {}", app.data.profiles.len() + 1),
                ..Default::default()
            });
            app.selected_profile = app.data.profiles.len() - 1;
            app.persist();
        }
        if ui.button(i18n::t(lang, "duplicate")).clicked() && app.data.profiles.len() < 100 {
            let mut copy = app.data.profiles[app.selected_profile].clone();
            copy.name = format!("{} (copy)", copy.name.chars().take(73).collect::<String>());
            app.data.profiles.push(copy);
            app.selected_profile = app.data.profiles.len() - 1;
            app.persist();
        }
        if ui
            .add_enabled(
                app.data.profiles.len() > 1,
                egui::Button::new(i18n::t(lang, "delete")),
            )
            .clicked()
        {
            app.data.profiles.remove(app.selected_profile);
            app.selected_profile = app.selected_profile.min(app.data.profiles.len() - 1);
            app.persist();
        }
    });
    ui.add_space(12.0);
    ui.horizontal_wrapped(|ui| {
        for (i, p) in app.data.profiles.iter().enumerate() {
            ui.selectable_value(&mut app.selected_profile, i, &p.name);
        }
    });
    ui.add_space(14.0);
    egui::Frame::new()
        .fill(PANEL)
        .corner_radius(14)
        .inner_margin(20.0)
        .show(ui, |ui| {
            let p = &mut app.data.profiles[app.selected_profile];
            egui::Grid::new("profile_editor")
                .num_columns(2)
                .spacing([18.0, 12.0])
                .show(ui, |ui| {
                    ui.label(i18n::t(lang, "name"));
                    ui.text_edit_singleline(&mut p.name);
                    ui.end_row();
                    ui.label("DPI");
                    ui.add(egui::DragValue::new(&mut p.dpi).range(100..=32000));
                    ui.end_row();
                    ui.label(i18n::t(lang, "rate"));
                    egui::ComboBox::from_id_salt("profile_rate")
                        .selected_text(format!("{} ms", p.report_rate_ms))
                        .show_ui(ui, |ui| {
                            for ms in [1, 2, 4, 8] {
                                ui.selectable_value(&mut p.report_rate_ms, ms, format!("{ms} ms"));
                            }
                        });
                    ui.end_row();
                    ui.label(i18n::t(lang, "color"));
                    ui.text_edit_singleline(&mut p.color);
                    ui.end_row();
                });
            ui.add_space(12.0);
            ui.label(i18n::t(lang, "notes"));
            ui.add_sized(
                [ui.available_width().min(600.0), 100.0],
                egui::TextEdit::multiline(&mut p.notes),
            );
            ui.add_space(10.0);
            if ui.button(i18n::t(lang, "save")).clicked() {
                app.persist();
            }
        });
}

fn catalog_page(app: &mut App, ui: &mut egui::Ui, lang: &str) {
    ui.heading(i18n::t(lang, "catalog"));
    ui.colored_label(MUTED, i18n::t(lang, "catalog_note"));
    ui.add_space(12.0);
    ui.add_sized(
        [ui.available_width().min(500.0), 32.0],
        egui::TextEdit::singleline(&mut app.catalog_query).hint_text(i18n::t(lang, "search")),
    );
    ui.add_space(12.0);
    let query = app.catalog_query.to_lowercase();
    for entry in app
        .catalog
        .iter()
        .filter(|e| {
            query.is_empty()
                || e.name.to_lowercase().contains(&query)
                || e.product_id.to_lowercase().contains(&query)
        })
        .take(200)
    {
        egui::Frame::new()
            .fill(PANEL)
            .corner_radius(8)
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(&entry.name);
                    ui.colored_label(
                        MUTED,
                        format!(
                            "{} · {} · {}",
                            entry.transport, entry.product_id, entry.kind
                        ),
                    );
                });
            });
        ui.add_space(5.0);
    }
}

fn settings_page(app: &mut App, ui: &mut egui::Ui, lang: &str) {
    ui.heading(i18n::t(lang, "settings"));
    ui.add_space(14.0);
    egui::Frame::new()
        .fill(PANEL)
        .corner_radius(14)
        .inner_margin(20.0)
        .show(ui, |ui| {
            ui.heading(i18n::t(lang, "interface_language"));
            let previous = app.data.language.clone();
            egui::ComboBox::from_id_salt("language")
                .selected_text(
                    i18n::LANGUAGES
                        .iter()
                        .find(|(code, _)| *code == app.data.language)
                        .map(|(_, name)| *name)
                        .unwrap_or("Français"),
                )
                .show_ui(ui, |ui| {
                    for (code, name) in i18n::LANGUAGES {
                        ui.selectable_value(&mut app.data.language, code.into(), name);
                    }
                });
            if app.data.language != previous {
                app.persist();
            }
            ui.separator();
            ui.heading("RGB / rJb");
            if ui
                .add_enabled(
                    app.settings_load_error.is_none(),
                    egui::Checkbox::new(
                        &mut app.data.rgb_control_enabled,
                        i18n::t(lang, "rgb_control"),
                    ),
                )
                .changed()
            {
                app.persist();
            }
            ui.colored_label(MUTED, i18n::t(lang, "rgb_note"));
        });
    ui.add_space(16.0);
    egui::Frame::new()
        .fill(PANEL)
        .corner_radius(14)
        .inner_margin(20.0)
        .show(ui, |ui| {
            ui.heading(i18n::t(lang, "backup"));
            ui.colored_label(MUTED, i18n::t(lang, "backup_note"));
            ui.horizontal(|ui| {
                if ui.button(i18n::t(lang, "export_backup")).clicked() {
                    match serde_json::to_string_pretty(&app.data) {
                        Ok(text) => {
                            ui.ctx().copy_text(text.clone());
                            app.backup_text = text;
                            app.status = "Backup copied to clipboard".into();
                        }
                        Err(error) => app.status = error.to_string(),
                    }
                }
                if ui.button(i18n::t(lang, "import_backup")).clicked() {
                    match serde_json::from_str::<Saved>(&app.backup_text)
                        .map_err(|e| e.to_string())
                        .and_then(|data| {
                            data.validate()?;
                            Ok(data)
                        }) {
                        Ok(data) => {
                            app.data = data;
                            app.selected_profile = 0;
                            app.settings_load_error = None;
                            app.persist();
                        }
                        Err(error) => app.status = format!("Invalid backup: {error}"),
                    }
                }
            });
            ui.add_sized(
                [ui.available_width().min(700.0), 130.0],
                egui::TextEdit::multiline(&mut app.backup_text).hint_text("JSON"),
            );
            if let Some(path) = &app.path {
                ui.colored_label(MUTED, path.display().to_string());
            }
        });
}

fn metric(ui: &mut egui::Ui, value: usize, label: &str) {
    egui::Frame::new()
        .fill(PANEL)
        .corner_radius(12)
        .inner_margin(16.0)
        .show(ui, |ui| {
            ui.set_min_width(145.0);
            ui.heading(value.to_string());
            ui.colored_label(MUTED, label);
        });
}

fn capability(ui: &mut egui::Ui, name: &str, enabled: bool, lang: &str) {
    egui::Frame::new()
        .fill(PANEL)
        .corner_radius(8)
        .inner_margin(10.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(name);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.colored_label(
                        if enabled { ACCENT } else { MUTED },
                        if enabled {
                            i18n::t(lang, "experimental")
                        } else {
                            i18n::t(lang, "not_available")
                        },
                    );
                });
            });
        });
    ui.add_space(5.0);
}

fn product_card(
    ui: &mut egui::Ui,
    product: &Product,
    width: f32,
    selected: bool,
    lang: &str,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, 206.0), Sense::click());
    let painter = ui.painter_at(rect);
    painter.rect_filled(
        rect,
        14.0,
        if response.hovered() || selected {
            CARD
        } else {
            PANEL
        },
    );
    if selected {
        painter.rect_filled(
            Rect::from_min_size(rect.min, Vec2::new(4.0, rect.height())),
            2.0,
            ACCENT,
        );
    }
    let preview = Rect::from_min_size(rect.min + Vec2::new(18.0, 16.0), Vec2::new(92.0, 100.0));
    painter.rect_filled(preview, 12.0, BG);
    paint_product(&painter, preview.shrink(19.0), product.kind, ACCENT);
    painter.text(
        Pos2::new(rect.left() + 18.0, rect.top() + 130.0),
        Align2::LEFT_TOP,
        shorten(&product.name, 27),
        FontId::proportional(16.0),
        Color32::WHITE,
    );
    painter.text(
        Pos2::new(rect.left() + 18.0, rect.top() + 159.0),
        Align2::LEFT_TOP,
        i18n::t(lang, product.kind.key()),
        FontId::proportional(12.0),
        MUTED,
    );
    painter.text(
        Pos2::new(rect.left() + 18.0, rect.top() + 181.0),
        Align2::LEFT_TOP,
        &product.id,
        FontId::monospace(11.0),
        MUTED,
    );
    response.on_hover_text(&product.name)
}

fn shorten(text: &str, max: usize) -> String {
    let mut chars = text.chars();
    let prefix: String = chars.by_ref().take(max).collect();
    if chars.next().is_some() {
        format!("{prefix}…")
    } else {
        prefix
    }
}

fn paint_product(p: &egui::Painter, r: Rect, kind: Kind, color: Color32) {
    let c = r.center();
    let stroke = Stroke::new(3.0_f32, color);
    match kind {
        Kind::Mouse => {
            p.rect_filled(
                Rect::from_center_size(c, Vec2::new(37.0, 66.0)),
                18.0,
                color,
            );
            p.line_segment(
                [c + Vec2::new(0.0, -30.0), c + Vec2::new(0.0, -8.0)],
                Stroke::new(3.0_f32, BG),
            );
            p.circle_filled(c + Vec2::new(0.0, -15.0), 3.0, BG);
        }
        Kind::Keyboard => {
            p.rect_filled(Rect::from_center_size(c, Vec2::new(75.0, 40.0)), 6.0, color);
            for row in 0..3 {
                for col in 0..6 {
                    let x = c.x - 32.0 + col as f32 * 12.5;
                    let y = c.y - 13.0 + row as f32 * 12.0;
                    p.rect_filled(
                        Rect::from_min_size(Pos2::new(x, y), Vec2::new(8.0, 6.0)),
                        1.0,
                        BG,
                    );
                }
            }
        }
        Kind::Headset => {
            p.circle_stroke(c + Vec2::new(0.0, -3.0), 27.0, stroke);
            p.rect_filled(
                Rect::from_center_size(c + Vec2::new(-26.0, 6.0), Vec2::new(11.0, 26.0)),
                4.0,
                color,
            );
            p.rect_filled(
                Rect::from_center_size(c + Vec2::new(26.0, 6.0), Vec2::new(11.0, 26.0)),
                4.0,
                color,
            );
        }
        Kind::Camera => {
            p.rect_filled(
                Rect::from_center_size(c + Vec2::new(0.0, -5.0), Vec2::new(62.0, 38.0)),
                8.0,
                color,
            );
            p.circle_filled(c + Vec2::new(0.0, -5.0), 14.0, BG);
            p.circle_filled(c + Vec2::new(0.0, -5.0), 6.0, color);
            p.line_segment([c + Vec2::new(0.0, 15.0), c + Vec2::new(0.0, 29.0)], stroke);
            p.line_segment(
                [c + Vec2::new(-18.0, 29.0), c + Vec2::new(18.0, 29.0)],
                stroke,
            );
        }
        Kind::Wheel => {
            p.circle_stroke(c, 32.0, Stroke::new(7.0_f32, color));
            p.circle_filled(c, 11.0, color);
            for offset in [
                Vec2::new(0.0, -27.0),
                Vec2::new(-25.0, 16.0),
                Vec2::new(25.0, 16.0),
            ] {
                p.line_segment([c, c + offset], stroke);
            }
        }
        Kind::Receiver | Kind::Other => {
            p.rect_filled(Rect::from_center_size(c, Vec2::new(48.0, 54.0)), 9.0, color);
            p.circle_filled(c, 12.0, BG);
            p.circle_filled(c, 5.0, color);
        }
    }
}

const CJK_FONTS: &[&str] = &[
    "/System/Library/Fonts/Hiragino Sans GB.ttc",
    "/System/Library/Fonts/STHeiti Light.ttc",
    "C:\\Windows\\Fonts\\msyh.ttc",
    "C:\\Windows\\Fonts\\YuGothR.ttc",
    "C:\\Windows\\Fonts\\msgothic.ttc",
    "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/google-noto-cjk/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
];

// Loaded only when a CJK language is selected, to keep the default footprint small.
fn install_cjk_font(ctx: &egui::Context) {
    let Some(bytes) = CJK_FONTS.iter().find_map(|path| std::fs::read(path).ok()) else {
        return;
    };
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "cjk".into(),
        std::sync::Arc::new(egui::FontData::from_owned(bytes)),
    );
    for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        fonts.families.entry(family).or_default().push("cjk".into());
    }
    ctx.set_fonts(fonts);
}
