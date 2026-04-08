use egui::{Color32, Pos2, Rect, Vec2, Rounding};
use std::process::Command;
use crate::theme::NeoTheme;

#[derive(Clone)]
pub struct AppEntry {
    pub name: &'static str,
    pub desc: &'static str,
    pub icon: &'static str,
    pub cmd: &'static str,
    pub category: &'static str,
}

pub fn builtin_apps() -> Vec<AppEntry> {
    vec![
        // System
        AppEntry { name: "File Manager",   desc: "Browse files",        icon: "📁", cmd: "_filemanager",  category: "System" },
        AppEntry { name: "Terminal",       desc: "Command line",        icon: "⌨", cmd: "alacritty",      category: "System" },
        AppEntry { name: "Settings",       desc: "System preferences",  icon: "⚙", cmd: "_settings",      category: "System" },
        AppEntry { name: "Task Manager",   desc: "System monitor",      icon: "📊", cmd: "alacritty -e btop", category: "System" },
        AppEntry { name: "Text Editor",    desc: "Edit text files",     icon: "📝", cmd: "alacritty -e nvim", category: "System" },
        AppEntry { name: "Disk Usage",     desc: "Disk space analyzer", icon: "💾", cmd: "alacritty -e df -h", category: "System" },
        AppEntry { name: "Logs",           desc: "System journal",      icon: "📋", cmd: "alacritty -e journalctl -f", category: "System" },
        AppEntry { name: "Nganjo Install", desc: "Install to disk",     icon: "💿", cmd: "alacritty -e nganjo-install", category: "System" },
        AppEntry { name: "Nganjo Setup",   desc: "Post-install setup",  icon: "🔧", cmd: "alacritty -e sudo nganjo-setup", category: "System" },
        // Internet
        AppEntry { name: "Firefox",        desc: "Web browser",         icon: "🌐", cmd: "firefox",        category: "Internet" },
        AppEntry { name: "Chromium",       desc: "Web browser",         icon: "🌍", cmd: "chromium",       category: "Internet" },
        AppEntry { name: "Thunderbird",    desc: "Email client",        icon: "📧", cmd: "thunderbird",    category: "Internet" },
        AppEntry { name: "SSH Client",     desc: "Remote terminal",     icon: "🔐", cmd: "alacritty -e ssh", category: "Internet" },
        AppEntry { name: "Network TUI",    desc: "WiFi manager",        icon: "📶", cmd: "alacritty -e nmtui", category: "Internet" },
        // Media
        AppEntry { name: "VLC",            desc: "Media player",        icon: "🎬", cmd: "vlc",            category: "Media" },
        AppEntry { name: "MPV",            desc: "Video player",        icon: "▶", cmd: "mpv",             category: "Media" },
        AppEntry { name: "Audacious",      desc: "Music player",        icon: "🎵", cmd: "audacious",      category: "Media" },
        AppEntry { name: "Screenshot",     desc: "Take screenshot",     icon: "📸", cmd: "scrot ~/Pictures/screenshot_%Y%m%d_%H%M%S.png", category: "Media" },
        // Development
        AppEntry { name: "VS Code",        desc: "Code editor",         icon: "💻", cmd: "code",           category: "Development" },
        AppEntry { name: "Neovim",         desc: "Terminal editor",     icon: "🖊", cmd: "alacritty -e nvim", category: "Development" },
        AppEntry { name: "Git Log",        desc: "Git history",         icon: "🌿", cmd: "alacritty -e bash -c 'git log --oneline; read'", category: "Development" },
        AppEntry { name: "Python REPL",    desc: "Python shell",        icon: "🐍", cmd: "alacritty -e python3", category: "Development" },
        AppEntry { name: "Rust Docs",      desc: "Rust documentation",  icon: "🦀", cmd: "firefox https://doc.rust-lang.org", category: "Development" },
        // Graphics
        AppEntry { name: "GIMP",           desc: "Image editor",        icon: "🎨", cmd: "gimp",           category: "Graphics" },
        AppEntry { name: "Inkscape",       desc: "Vector editor",       icon: "✏", cmd: "inkscape",        category: "Graphics" },
        // Office
        AppEntry { name: "LibreOffice",    desc: "Office suite",        icon: "📄", cmd: "libreoffice",    category: "Office" },
        AppEntry { name: "Evince",         desc: "PDF viewer",          icon: "📕", cmd: "evince",         category: "Office" },
        AppEntry { name: "Calculator",     desc: "Calculator",          icon: "🔢", cmd: "alacritty -e bash -c 'python3 -c \"import math; print(eval(input(\\\"Calc: \\\")))\"; read'", category: "Office" },
    ]
}

pub struct Launcher {
    pub search: String,
    pub selected_category: Option<String>,
    pub selected_idx: usize,
    pub recent: Vec<&'static str>,  // cmd strings of recently launched apps
}

impl Default for Launcher {
    fn default() -> Self {
        Self {
            search: String::new(),
            selected_category: None,
            selected_idx: 0,
            recent: vec![],
        }
    }
}

impl Launcher {
    pub fn draw(
        &mut self,
        ctx: &egui::Context,
        theme: &NeoTheme,
        screen_rect: Rect,
        show: &mut bool,
        open_windows: &mut Vec<String>,
    ) {
        if !*show { return; }

        let apps = builtin_apps();

        // Build filtered list
        let filtered: Vec<&AppEntry> = apps.iter().filter(|a| {
            let cat_ok = match &self.selected_category {
                None => true,
                Some(c) => a.category == c.as_str(),
            };
            let q = self.search.to_lowercase();
            let search_ok = q.is_empty()
                || a.name.to_lowercase().contains(&q)
                || a.desc.to_lowercase().contains(&q)
                || a.category.to_lowercase().contains(&q);
            cat_ok && search_ok
        }).collect();

        // Clamp selection
        if !filtered.is_empty() {
            self.selected_idx = self.selected_idx.min(filtered.len() - 1);
        }

        // Keyboard navigation
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Escape) { *show = false; }
            if i.key_pressed(egui::Key::ArrowRight) && self.selected_idx + 1 < filtered.len() { self.selected_idx += 1; }
            if i.key_pressed(egui::Key::ArrowLeft)  && self.selected_idx > 0 { self.selected_idx -= 1; }
        });

        // Dimmed backdrop
        ctx.layer_painter(egui::LayerId::new(egui::Order::PanelResizeLine, egui::Id::new("launcher_overlay")))
            .rect_filled(screen_rect, 0.0, Color32::from_rgba_premultiplied(0, 0, 0, 160));

        let panel_w = 720.0_f32.min(screen_rect.width() - 60.0);
        let panel_h = 560.0_f32.min(screen_rect.height() - 100.0);
        let panel_pos = Pos2::new(
            screen_rect.center().x - panel_w / 2.0,
            screen_rect.center().y - panel_h / 2.0 - 20.0,
        );

        egui::Area::new(egui::Id::new("launcher_area"))
            .fixed_pos(panel_pos)
            .order(egui::Order::Tooltip)
            .show(ctx, |ui| {
                ui.set_min_size(Vec2::new(panel_w, panel_h));
                egui::Frame::none()
                    .fill(theme.panel_bg())
                    .rounding(Rounding::same(20.0))
                    .stroke(egui::Stroke::new(1.5, theme.accent_dim()))
                    .shadow(egui::epaint::Shadow {
                        offset: Vec2::new(0.0, 24.0),
                        blur: 64.0,
                        spread: 0.0,
                        color: Color32::from_black_alpha(180),
                    })
                    .show(ui, |ui| {
                        ui.set_min_size(Vec2::new(panel_w, panel_h));
                        ui.vertical(|ui| {
                            ui.add_space(20.0);

                            // ── Search bar ────────────────────────────────
                            ui.horizontal(|ui| {
                                ui.add_space(20.0);
                                // Search box with accent border
                                egui::Frame::none()
                                    .fill(theme.card_bg())
                                    .rounding(Rounding::same(12.0))
                                    .stroke(egui::Stroke::new(1.5, theme.accent()))
                                    .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new("⌕ ").size(18.0).color(theme.accent()));
                                            let resp = ui.add(
                                                egui::TextEdit::singleline(&mut self.search)
                                                    .hint_text("Search applications...")
                                                    .frame(false)
                                                    .desired_width(panel_w - 120.0)
                                                    .font(egui::FontId::proportional(16.0))
                                            );
                                            resp.request_focus();
                                            // Reset selection on search change
                                            if resp.changed() { self.selected_idx = 0; }
                                            // Clear button
                                            if !self.search.is_empty() {
                                                if ui.add(egui::Button::new(
                                                    egui::RichText::new("✕").size(13.0).color(theme.text_dim())
                                                ).frame(false)).clicked() {
                                                    self.search.clear();
                                                    self.selected_idx = 0;
                                                }
                                            }
                                        });
                                    });
                                ui.add_space(20.0);
                            });

                            ui.add_space(12.0);

                            // ── Category pills ────────────────────────────
                            ui.horizontal(|ui| {
                                ui.add_space(20.0);
                                for cat in &["All", "System", "Internet", "Media", "Development", "Graphics", "Office"] {
                                    let active = if *cat == "All" { self.selected_category.is_none() }
                                                 else { self.selected_category.as_deref() == Some(*cat) };
                                    // Count badge
                                    let count = if *cat == "All" { apps.len() }
                                                else { apps.iter().filter(|a| a.category == *cat).count() };
                                    let label = format!("{}  {}", cat, count);
                                    let btn = egui::Button::new(
                                        egui::RichText::new(label).size(11.5)
                                            .color(if active { Color32::WHITE } else { theme.text_dim() })
                                    )
                                    .fill(if active { theme.accent() } else { theme.card_bg() })
                                    .rounding(Rounding::same(20.0))
                                    .stroke(egui::Stroke::new(1.0, if active { theme.accent() } else { theme.border() }));
                                    if ui.add(btn).clicked() {
                                        self.selected_category = if *cat == "All" { None } else { Some(cat.to_string()) };
                                        self.selected_idx = 0;
                                    }
                                }
                            });

                            ui.add_space(12.0);
                            ui.add(egui::Separator::default().spacing(0.0));
                            ui.add_space(8.0);

                            // ── App grid ──────────────────────────────────
                            if filtered.is_empty() {
                                ui.add_space(60.0);
                                ui.vertical_centered(|ui| {
                                    ui.label(egui::RichText::new("🔍").size(40.0));
                                    ui.add_space(8.0);
                                    ui.label(egui::RichText::new(format!("No apps found for \"{}\"", self.search))
                                        .size(14.0).color(theme.text_dim()));
                                });
                            } else {
                                let cols = ((panel_w - 40.0) / 130.0) as usize;
                                let cols = cols.max(3).min(6);
                                let cell_w = (panel_w - 40.0) / cols as f32;
                                let cell_h = 100.0;

                                egui::ScrollArea::vertical()
                                    .id_source("launcher_scroll")
                                    .max_height(panel_h - 200.0)
                                    .show(ui, |ui| {
                                        ui.add_space(4.0);
                                        egui::Grid::new("app_grid")
                                            .num_columns(cols)
                                            .spacing(Vec2::new(4.0, 4.0))
                                            .min_col_width(cell_w)
                                            .max_col_width(cell_w)
                                            .show(ui, |ui| {
                                                for (i, app) in filtered.iter().enumerate() {
                                                    if i > 0 && i % cols == 0 { ui.end_row(); }

                                                    let is_selected = i == self.selected_idx;
                                                    let (id, rect) = ui.allocate_space(Vec2::new(cell_w, cell_h));
                                                    let resp = ui.interact(rect, id, egui::Sense::click());

                                                    if resp.hovered() { self.selected_idx = i; }

                                                    // Background
                                                    let bg = if is_selected || resp.hovered() {
                                                        theme.hover()
                                                    } else { Color32::TRANSPARENT };
                                                    ui.painter().rect_filled(rect, Rounding::same(12.0), bg);

                                                    // Accent border on selected
                                                    if is_selected {
                                                        ui.painter().rect_stroke(rect, Rounding::same(12.0),
                                                            egui::Stroke::new(1.5, theme.accent()));
                                                        // Subtle glow
                                                        ui.painter().rect_stroke(
                                                            rect.expand(2.0), Rounding::same(14.0),
                                                            egui::Stroke::new(1.0, Color32::from_rgba_premultiplied(
                                                                theme.accent().r(), theme.accent().g(), theme.accent().b(), 40
                                                            )));
                                                    }

                                                    // Icon
                                                    ui.painter().text(
                                                        Pos2::new(rect.center().x, rect.top() + 32.0),
                                                        egui::Align2::CENTER_CENTER,
                                                        app.icon,
                                                        egui::FontId::proportional(32.0),
                                                        theme.text(),
                                                    );

                                                    // Name
                                                    ui.painter().text(
                                                        Pos2::new(rect.center().x, rect.top() + 62.0),
                                                        egui::Align2::CENTER_CENTER,
                                                        app.name,
                                                        egui::FontId::proportional(11.5),
                                                        if is_selected { theme.accent() } else { theme.text() },
                                                    );

                                                    // Desc
                                                    ui.painter().text(
                                                        Pos2::new(rect.center().x, rect.top() + 78.0),
                                                        egui::Align2::CENTER_CENTER,
                                                        app.desc,
                                                        egui::FontId::proportional(9.5),
                                                        theme.text_dim(),
                                                    );

                                                    // Recently used dot
                                                    if self.recent.contains(&app.cmd) {
                                                        ui.painter().circle_filled(
                                                            Pos2::new(rect.right() - 8.0, rect.top() + 8.0),
                                                            3.5, theme.accent(),
                                                        );
                                                    }

                                                    if resp.clicked() || (is_selected && ctx.input(|i| i.key_pressed(egui::Key::Enter))) {
                                                        // Track recent
                                                        self.recent.retain(|&r| r != app.cmd);
                                                        self.recent.insert(0, app.cmd);
                                                        self.recent.truncate(8);

                                                        *show = false;
                                                        self.search.clear();
                                                        self.selected_idx = 0;

                                                        match app.cmd {
                                                            "_filemanager" => {
                                                                if !open_windows.contains(&"filemanager".to_string()) {
                                                                    open_windows.push("filemanager".to_string());
                                                                }
                                                            }
                                                            "_settings" => {
                                                                if !open_windows.contains(&"settings".to_string()) {
                                                                    open_windows.push("settings".to_string());
                                                                }
                                                            }
                                                            cmd => { let _ = Command::new("sh").arg("-c").arg(cmd).spawn(); }
                                                        }
                                                    }
                                                }
                                            });
                                        ui.add_space(8.0);
                                    });
                            }

                            // ── Footer ────────────────────────────────────
                            ui.add_space(4.0);
                            ui.add(egui::Separator::default().spacing(0.0));
                            ui.horizontal(|ui| {
                                ui.add_space(20.0);
                                for (key, action) in &[("↵", "Open"), ("←→", "Navigate"), ("Esc", "Close")] {
                                    ui.label(egui::RichText::new(*key).size(11.0).color(theme.accent()).strong());
                                    ui.label(egui::RichText::new(format!(" {}   ", action)).size(11.0).color(theme.text_dim()));
                                }
                                let total = filtered.len();
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.add_space(20.0);
                                    ui.label(egui::RichText::new(format!("{} apps", total)).size(11.0).color(theme.text_dim()));
                                });
                            });
                            ui.add_space(8.0);
                        });
                    });
            });
    }
}
