use egui::{Color32, Rounding, Vec2};
use std::path::{Path, PathBuf};
use std::fs;
use crate::theme::NeoTheme;

#[derive(Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub ext: String,
}

pub struct FileManager {
    pub current_path: PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected: Option<usize>,
    pub search: String,
    pub view_grid: bool,
    pub show_hidden: bool,
    pub bookmarks: Vec<PathBuf>,
    pub history: Vec<PathBuf>,
    pub history_pos: usize,
    pub sort_by: SortBy,
    pub sort_asc: bool,
    pub status: String,
    pub clipboard: Option<(PathBuf, bool)>, // path, is_cut
}

#[derive(PartialEq, Clone)]
pub enum SortBy { Name, Size, Type }

impl Default for FileManager {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        let mut fm = Self {
            current_path: home.clone(),
            entries: Vec::new(),
            selected: None,
            search: String::new(),
            view_grid: false,
            show_hidden: false,
            bookmarks: vec![
                dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")),
                PathBuf::from("/"),
                dirs::download_dir().unwrap_or_else(|| PathBuf::from("~/Downloads")),
                dirs::document_dir().unwrap_or_else(|| PathBuf::from("~/Documents")),
                dirs::picture_dir().unwrap_or_else(|| PathBuf::from("~/Pictures")),
                dirs::desktop_dir().unwrap_or_else(|| PathBuf::from("~/Desktop")),
            ],
            history: vec![home],
            history_pos: 0,
            sort_by: SortBy::Name,
            sort_asc: true,
            status: String::new(),
            clipboard: None,
        };
        fm.refresh();
        fm
    }
}

impl FileManager {
    pub fn refresh(&mut self) {
        self.entries.clear();
        if let Ok(read) = fs::read_dir(&self.current_path) {
            for entry in read.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if !self.show_hidden && name.starts_with('.') { continue; }
                if !self.search.is_empty() && !name.to_lowercase().contains(&self.search.to_lowercase()) { continue; }
                let is_dir = path.is_dir();
                let size = if is_dir { 0 } else { fs::metadata(&path).map(|m| m.len()).unwrap_or(0) };
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                self.entries.push(FileEntry { name, path, is_dir, size, ext });
            }
        }
        self.sort();
    }

    fn sort(&mut self) {
        self.entries.sort_by(|a, b| {
            // Dirs first
            if a.is_dir != b.is_dir {
                return if a.is_dir { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater };
            }
            let ord = match self.sort_by {
                SortBy::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortBy::Size => a.size.cmp(&b.size),
                SortBy::Type => a.ext.cmp(&b.ext),
            };
            if self.sort_asc { ord } else { ord.reverse() }
        });
    }

    pub fn navigate_to(&mut self, path: PathBuf) {
        // Truncate forward history
        self.history.truncate(self.history_pos + 1);
        self.history.push(path.clone());
        self.history_pos = self.history.len() - 1;
        self.current_path = path;
        self.selected = None;
        self.refresh();
    }

    pub fn go_back(&mut self) {
        if self.history_pos > 0 {
            self.history_pos -= 1;
            self.current_path = self.history[self.history_pos].clone();
            self.selected = None;
            self.refresh();
        }
    }

    pub fn go_forward(&mut self) {
        if self.history_pos + 1 < self.history.len() {
            self.history_pos += 1;
            self.current_path = self.history[self.history_pos].clone();
            self.selected = None;
            self.refresh();
        }
    }

    pub fn go_up(&mut self) {
        if let Some(parent) = self.current_path.parent().map(|p| p.to_path_buf()) {
            self.navigate_to(parent);
        }
    }

    pub fn open_selected(&mut self) {
        if let Some(idx) = self.selected {
            if let Some(entry) = self.entries.get(idx).cloned() {
                if entry.is_dir {
                    self.navigate_to(entry.path);
                } else {
                    let _ = open::that(&entry.path);
                    self.status = format!("Opening {}", entry.name);
                }
            }
        }
    }

    pub fn file_icon(entry: &FileEntry) -> &'static str {
        if entry.is_dir { return "📁"; }
        match entry.ext.as_str() {
            "rs" => "🦀", "py" => "🐍", "js" | "ts" => "📜", "html" | "htm" => "🌐",
            "css" => "🎨", "json" => "📋", "toml" | "yaml" | "yml" => "⚙",
            "md" | "txt" => "📄", "pdf" => "📕",
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg" => "🖼",
            "mp4" | "mkv" | "avi" | "mov" | "webm" => "🎬",
            "mp3" | "flac" | "ogg" | "wav" | "aac" => "🎵",
            "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" => "📦",
            "sh" | "bash" | "zsh" => "⌨",
            "exe" | "bin" | "elf" => "⚡",
            "iso" | "img" => "💿",
            "deb" | "rpm" | "pkg" => "📦",
            _ => "📄",
        }
    }

    pub fn format_size(bytes: u64) -> String {
        if bytes == 0 { return "-".to_string(); }
        const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
        let mut val = bytes as f64;
        let mut unit = 0;
        while val >= 1024.0 && unit < 4 {
            val /= 1024.0;
            unit += 1;
        }
        if unit == 0 { format!("{} B", bytes) } else { format!("{:.1} {}", val, UNITS[unit]) }
    }

    pub fn draw_window(
        &mut self,
        ctx: &egui::Context,
        theme: &NeoTheme,
        open: &mut bool,
    ) {
        let mut window_open = *open;
        egui::Window::new("File Manager")
            .id(egui::Id::new("filemanager_window"))
            .open(&mut window_open)
            .min_size(Vec2::new(700.0, 450.0))
            .default_size(Vec2::new(900.0, 560.0))
            .resizable(true)
            .show(ctx, |ui| {
                self.draw_inner(ui, theme);
            });
        *open = window_open;
    }

    fn draw_inner(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        ui.vertical(|ui| {
            // Toolbar
            ui.horizontal(|ui| {
                // Nav buttons
                let back_enabled = self.history_pos > 0;
                let fwd_enabled = self.history_pos + 1 < self.history.len();

                if ui.add_enabled(back_enabled, egui::Button::new("◀").rounding(Rounding::same(6.0))).clicked() {
                    self.go_back();
                }
                if ui.add_enabled(fwd_enabled, egui::Button::new("▶").rounding(Rounding::same(6.0))).clicked() {
                    self.go_forward();
                }
                if ui.button("↑").on_hover_text("Parent").clicked() {
                    self.go_up();
                }
                if ui.button("↺").on_hover_text("Refresh").clicked() {
                    self.refresh();
                }

                ui.separator();

                // Path bar
                let path_str = self.current_path.to_string_lossy().to_string();
                let path_label = egui::Label::new(
                    egui::RichText::new(&path_str)
                        .size(13.0)
                        .color(theme.text_dim())
                );
                ui.add(path_label);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // View toggle
                    let view_icon = if self.view_grid { "☰" } else { "⊞" };
                    if ui.button(view_icon).on_hover_text("Toggle view").clicked() {
                        self.view_grid = !self.view_grid;
                    }

                    // Hidden files
                    let hidden_btn = egui::Button::new("👁")
                        .fill(if self.show_hidden { theme.accent_dim() } else { Color32::TRANSPARENT });
                    if ui.add(hidden_btn).on_hover_text("Show hidden").clicked() {
                        self.show_hidden = !self.show_hidden;
                        self.refresh();
                    }

                    // Search
                    let search = egui::TextEdit::singleline(&mut self.search)
                        .hint_text("Search...")
                        .desired_width(150.0);
                    if ui.add(search).changed() {
                        self.refresh();
                    }
                });
            });

            ui.separator();

            // Main content - sidebar + file list
            ui.horizontal(|ui| {
                // Sidebar
                let sidebar_w = 160.0;
                ui.vertical(|ui| {
                    ui.set_min_width(sidebar_w);
                    ui.set_max_width(sidebar_w);

                    let bookmarks = self.bookmarks.clone();
                    let bookmark_names = ["Home", "Root", "Downloads", "Documents", "Pictures", "Desktop"];
                    let bookmark_icons = ["🏠", "💻", "⬇", "📄", "🖼", "🖥"];

                    for (i, (bm, (name, icon))) in bookmarks.iter().zip(bookmark_names.iter().zip(bookmark_icons.iter())).enumerate() {
                        let active = &self.current_path == bm;
                        let bg = if active { theme.accent_dim() } else { Color32::TRANSPARENT };
                        let text_color = if active { theme.accent() } else { theme.text() };

                        let btn = egui::Button::new(
                            egui::RichText::new(format!("{} {}", icon, name)).color(text_color).size(13.0)
                        )
                        .fill(bg)
                        .rounding(Rounding::same(6.0))
                        .min_size(Vec2::new(sidebar_w - 8.0, 28.0));

                        if ui.add(btn).clicked() {
                            self.navigate_to(bm.clone());
                        }
                    }
                });

                ui.separator();

                // File list / grid
                let available = ui.available_size();
                egui::ScrollArea::vertical()
                    .id_source("fm_scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.view_grid {
                            self.draw_grid_view(ui, theme);
                        } else {
                            self.draw_list_view(ui, theme, available.x);
                        }
                    });
            });

            ui.separator();

            // Status bar
            let count = self.entries.len();
            let selected_info = self.selected.and_then(|i| self.entries.get(i)).map(|e| {
                if e.is_dir { format!(" — {} (directory)", e.name) }
                else { format!(" — {} ({})", e.name, Self::format_size(e.size)) }
            }).unwrap_or_default();

            ui.label(
                egui::RichText::new(format!("{} items{}", count, selected_info))
                    .size(11.0)
                    .color(theme.text_dim())
            );
        });
    }

    fn draw_list_view(&mut self, ui: &mut egui::Ui, theme: &NeoTheme, avail_w: f32) {
        // Header
        ui.horizontal(|ui| {
            ui.set_min_width(avail_w);
            let sort_btn = |ui: &mut egui::Ui, label: &str, sort: SortBy, current: &mut SortBy, asc: &mut bool| {
                let active = *current == sort;
                let arrow = if active { if *asc { " ↑" } else { " ↓" } } else { "" };
                let btn = egui::Button::new(
                    egui::RichText::new(format!("{}{}", label, arrow)).size(12.0)
                        .color(if active { theme.accent() } else { theme.text_dim() })
                )
                .fill(Color32::TRANSPARENT);
                if ui.add(btn).clicked() {
                    if *current == sort { *asc = !*asc; } else { *current = sort; *asc = true; }
                    true
                } else { false }
            };

            let mut refresh = false;
            if sort_btn(ui, "Name", SortBy::Name, &mut self.sort_by, &mut self.sort_asc) { refresh = true; }
            ui.add_space(200.0);
            if sort_btn(ui, "Size", SortBy::Size, &mut self.sort_by, &mut self.sort_asc) { refresh = true; }
            ui.add_space(40.0);
            if sort_btn(ui, "Type", SortBy::Type, &mut self.sort_by, &mut self.sort_asc) { refresh = true; }
            if refresh { self.sort(); }
        });

        ui.separator();

        let entries = self.entries.clone();
        let mut navigate_to: Option<PathBuf> = None;
        let mut open_file: Option<PathBuf> = None;

        for (i, entry) in entries.iter().enumerate() {
            let selected = self.selected == Some(i);
            let bg = if selected { theme.accent_dim() } else { Color32::TRANSPARENT };

            let (row_id, row_rect) = ui.allocate_space(Vec2::new(avail_w - 16.0, 28.0));
            let row_resp = ui.interact(row_rect, row_id, egui::Sense::click());

            let hover_bg = if row_resp.hovered() { theme.hover() } else { bg };
            ui.painter().rect_filled(row_rect, Rounding::same(4.0), hover_bg);

            if selected {
                ui.painter().rect_stroke(row_rect, Rounding::same(4.0), egui::Stroke::new(1.0, theme.accent()));
            }

            // Icon + name
            let icon = Self::file_icon(entry);
            let text_x = row_rect.left() + 8.0;
            let text_y = row_rect.center().y;
            ui.painter().text(
                egui::Pos2::new(text_x, text_y),
                egui::Align2::LEFT_CENTER,
                icon,
                egui::FontId::proportional(14.0),
                theme.text(),
            );
            ui.painter().text(
                egui::Pos2::new(text_x + 24.0, text_y),
                egui::Align2::LEFT_CENTER,
                &entry.name,
                egui::FontId::proportional(13.0),
                if entry.is_dir { theme.accent() } else { theme.text() },
            );

            // Size
            if !entry.is_dir {
                ui.painter().text(
                    egui::Pos2::new(row_rect.right() - 100.0, text_y),
                    egui::Align2::LEFT_CENTER,
                    Self::format_size(entry.size),
                    egui::FontId::proportional(11.0),
                    theme.text_dim(),
                );
            }

            // Type
            let type_label = if entry.is_dir { "DIR".to_string() } else { entry.ext.to_uppercase() };
            ui.painter().text(
                egui::Pos2::new(row_rect.right() - 20.0, text_y),
                egui::Align2::RIGHT_CENTER,
                &type_label,
                egui::FontId::proportional(10.0),
                theme.text_dim(),
            );

            if row_resp.clicked() {
                self.selected = Some(i);
            }
            if row_resp.double_clicked() {
                if entry.is_dir {
                    navigate_to = Some(entry.path.clone());
                } else {
                    open_file = Some(entry.path.clone());
                }
            }
        }

        if let Some(path) = navigate_to {
            self.navigate_to(path);
        }
        if let Some(path) = open_file {
            let _ = open::that(&path);
        }
    }

    fn draw_grid_view(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        let icon_size = Vec2::new(90.0, 90.0);
        let cols = ((ui.available_width() + 8.0) / (icon_size.x + 8.0)) as usize;
        let cols = cols.max(2);

        let entries = self.entries.clone();
        let mut navigate_to: Option<PathBuf> = None;
        let mut open_file: Option<PathBuf> = None;

        egui::Grid::new("fm_grid")
            .spacing(Vec2::new(8.0, 8.0))
            .num_columns(cols)
            .show(ui, |ui| {
                for (i, entry) in entries.iter().enumerate() {
                    if i > 0 && i % cols == 0 {
                        ui.end_row();
                    }

                    let selected = self.selected == Some(i);
                    let (icon_id, icon_rect) = ui.allocate_space(icon_size);
                    let resp = ui.interact(icon_rect, icon_id, egui::Sense::click());

                    let bg = if selected { theme.accent_dim() }
                        else if resp.hovered() { theme.hover() }
                        else { Color32::TRANSPARENT };
                    ui.painter().rect_filled(icon_rect, Rounding::same(8.0), bg);
                    if selected {
                        ui.painter().rect_stroke(icon_rect, Rounding::same(8.0), egui::Stroke::new(1.0, theme.accent()));
                    }

                    let icon = Self::file_icon(entry);
                    ui.painter().text(
                        egui::Pos2::new(icon_rect.center().x, icon_rect.top() + 28.0),
                        egui::Align2::CENTER_CENTER,
                        icon,
                        egui::FontId::proportional(28.0),
                        theme.text(),
                    );

                    let name_short = if entry.name.len() > 12 {
                        format!("{}…", &entry.name[..10])
                    } else {
                        entry.name.clone()
                    };
                    ui.painter().text(
                        egui::Pos2::new(icon_rect.center().x, icon_rect.top() + 58.0),
                        egui::Align2::CENTER_CENTER,
                        &name_short,
                        egui::FontId::proportional(10.0),
                        if entry.is_dir { theme.accent() } else { theme.text() },
                    );

                    if resp.clicked() { self.selected = Some(i); }
                    if resp.double_clicked() {
                        if entry.is_dir { navigate_to = Some(entry.path.clone()); }
                        else { open_file = Some(entry.path.clone()); }
                    }
                }
            });

        if let Some(path) = navigate_to { self.navigate_to(path); }
        if let Some(path) = open_file { let _ = open::that(&path); }
    }
}
