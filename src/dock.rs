// dock.rs — macOS-style dock with magnification, pinned apps, custom icons
use egui::{Color32, Pos2, Rect, Rounding, Vec2};
use crate::theme::NeoTheme;
use std::process::Command;

#[derive(Clone)]
pub struct DockApp {
    pub name:    &'static str,
    pub icon:    &'static str,   // emoji
    pub cmd:     &'static str,   // "_filemanager" | "_settings" | shell cmd
    pub color:   [u8; 3],        // icon bg gradient color
    pub pinned:  bool,
}

fn default_pinned() -> Vec<DockApp> {
    vec![
        DockApp { name: "Finder",      icon: "📁", cmd: "_filemanager",  color: [50, 160, 255],  pinned: true },
        DockApp { name: "Terminal",    icon: "⌨",  cmd: "alacritty",     color: [30, 30, 40],    pinned: true },
        DockApp { name: "Firefox",     icon: "🌐", cmd: "firefox",       color: [255, 100, 40],  pinned: true },
        DockApp { name: "Settings",    icon: "⚙",  cmd: "_settings",     color: [80, 80, 100],   pinned: true },
        DockApp { name: "Files",       icon: "🗂",  cmd: "_filemanager",  color: [60, 200, 140],  pinned: true },
        DockApp { name: "Code",        icon: "💻", cmd: "code",          color: [30, 120, 220],  pinned: true },
        DockApp { name: "Music",       icon: "🎵", cmd: "audacious",     color: [220, 60, 100],  pinned: true },
        DockApp { name: "Install",     icon: "💿", cmd: "alacritty -e nganjo-install", color: [0, 180, 160], pinned: true },
    ]
}

pub struct Dock {
    pub apps:         Vec<DockApp>,
    pub show_launcher: bool,
    pub show_settings: bool,
    pub running:      Vec<String>,  // cmd strings of running apps
}

impl Default for Dock {
    fn default() -> Self {
        Self {
            apps:          default_pinned(),
            show_launcher: false,
            show_settings: false,
            running:       vec![],
        }
    }
}

impl Dock {
    pub fn draw(
        &mut self,
        ctx: &egui::Context,
        theme: &NeoTheme,
        screen_rect: Rect,
        open_windows: &mut Vec<String>,
    ) {
        let base_size = 56.0f32;
        let max_size  = 80.0f32;
        let padding   = 10.0f32;
        let dock_h    = base_size + padding * 2.0 + 20.0; // +20 for labels

        let n = self.apps.len() + 2; // +2 for launcher btn + separator
        let dock_w = (n as f32 * (base_size + padding)).min(screen_rect.width() - 40.0);

        let dock_rect = Rect::from_center_size(
            Pos2::new(screen_rect.center().x, screen_rect.bottom() - dock_h / 2.0 - 8.0),
            Vec2::new(dock_w, dock_h),
        );

        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("dock")));

        // Frosted glass background
        painter.rect_filled(
            dock_rect,
            Rounding::same(20.0),
            match theme.mode {
                crate::theme::ThemeMode::Dark  => Color32::from_rgba_premultiplied(20, 24, 36, 200),
                crate::theme::ThemeMode::Light => Color32::from_rgba_premultiplied(240, 242, 248, 210),
            },
        );
        painter.rect_stroke(dock_rect, Rounding::same(20.0),
            egui::Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 30)));

        let mouse_pos = ctx.input(|i| i.pointer.hover_pos());

        egui::Area::new(egui::Id::new("dock_area"))
            .fixed_pos(dock_rect.min)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                ui.set_min_size(dock_rect.size());
                ui.horizontal(|ui| {
                    ui.add_space(padding);

                    // ── Launcher button ───────────────────────────────────
                    let center_y = dock_rect.min.y + padding + base_size / 2.0;
                    let launcher_x = ui.cursor().min.x + base_size / 2.0;
                    let launcher_center = Pos2::new(launcher_x, center_y);

                    let mag = magnify(Pos2::new(launcher_x, center_y), mouse_pos, base_size, max_size);
                    let (id, rect) = ui.allocate_space(Vec2::new(mag, mag));
                    let resp = ui.interact(rect, id, egui::Sense::click());

                    // Draw launcher icon — grid of dots
                    let icon_rect = Rect::from_center_size(
                        Pos2::new(launcher_x, center_y),
                        Vec2::splat(mag),
                    );
                    draw_app_icon(&painter, icon_rect, "⊞", [60, 60, 80], theme, resp.hovered() || self.show_launcher);

                    // Dot indicator
                    if self.show_launcher {
                        painter.circle_filled(
                            Pos2::new(icon_rect.center().x, icon_rect.bottom() + 5.0),
                            3.0, theme.accent(),
                        );
                    }

                    if resp.hovered() {
                        draw_tooltip(&painter, icon_rect, "App Menu", theme);
                    }
                    if resp.clicked() {
                        self.show_launcher = !self.show_launcher;
                    }

                    ui.add_space(padding);

                    // Separator
                    painter.line_segment(
                        [
                            Pos2::new(ui.cursor().min.x, dock_rect.min.y + 12.0),
                            Pos2::new(ui.cursor().min.x, dock_rect.max.y - 20.0),
                        ],
                        egui::Stroke::new(1.0, Color32::from_rgba_premultiplied(255,255,255,30)),
                    );
                    ui.add_space(padding);

                    // ── Pinned apps ───────────────────────────────────────
                    for app in &self.apps {
                        let app_x = ui.cursor().min.x + base_size / 2.0;
                        let app_center = Pos2::new(app_x, center_y);
                        let mag = magnify(app_center, mouse_pos, base_size, max_size);

                        let (id, _) = ui.allocate_space(Vec2::new(mag, mag));
                        let icon_rect = Rect::from_center_size(app_center, Vec2::splat(mag));
                        let resp = ui.interact(icon_rect, id, egui::Sense::click());

                        let is_open = open_windows.iter().any(|w| app.cmd.contains(w.as_str()))
                            || (app.cmd == "_filemanager" && open_windows.contains(&"filemanager".to_string()))
                            || (app.cmd == "_settings"    && open_windows.contains(&"settings".to_string()));

                        draw_app_icon(&painter, icon_rect, app.icon, app.color, theme, resp.hovered());

                        // Running dot
                        if is_open {
                            painter.circle_filled(
                                Pos2::new(icon_rect.center().x, icon_rect.bottom() + 5.0),
                                3.0, theme.accent(),
                            );
                        }

                        if resp.hovered() {
                            draw_tooltip(&painter, icon_rect, app.name, theme);
                        }

                        if resp.clicked() {
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

                        ui.add_space(padding);
                    }
                });
            });
    }
}

/// Magnify icon size based on mouse proximity (macOS-style)
fn magnify(icon_center: Pos2, mouse: Option<Pos2>, base: f32, max: f32) -> f32 {
    match mouse {
        None => base,
        Some(m) => {
            let dist = (m.x - icon_center.x).abs();
            let range = 80.0;
            if dist > range { base }
            else {
                let t = 1.0 - (dist / range);
                base + (max - base) * t * t
            }
        }
    }
}

/// Draw a rounded-square app icon with gradient-style fill + emoji
fn draw_app_icon(
    painter: &egui::Painter,
    rect: Rect,
    icon: &str,
    color: [u8; 3],
    theme: &NeoTheme,
    hovered: bool,
) {
    let r = Rounding::same(rect.width() * 0.22); // ~22% radius like macOS

    // Base color
    let base = Color32::from_rgb(color[0], color[1], color[2]);
    // Lighter top-left for gradient feel
    let light = Color32::from_rgb(
        (color[0] as u16 + 60).min(255) as u8,
        (color[1] as u16 + 60).min(255) as u8,
        (color[2] as u16 + 60).min(255) as u8,
    );

    painter.rect_filled(rect, r, base);

    // Simulate gradient: lighter top half
    let top_half = Rect::from_min_max(rect.min, Pos2::new(rect.max.x, rect.center().y));
    painter.rect_filled(top_half, Rounding { nw: r.nw, ne: r.ne, sw: 0.0, se: 0.0 },
        Color32::from_rgba_premultiplied(light.r(), light.g(), light.b(), 80));

    // Hover highlight
    if hovered {
        painter.rect_stroke(rect, r, egui::Stroke::new(2.0, Color32::from_rgba_premultiplied(255,255,255,120)));
    }

    // Subtle inner shadow at bottom
    let bottom_strip = Rect::from_min_max(
        Pos2::new(rect.min.x, rect.max.y - rect.height() * 0.25),
        rect.max,
    );
    painter.rect_filled(bottom_strip, Rounding { nw: 0.0, ne: 0.0, sw: r.sw, se: r.se },
        Color32::from_rgba_premultiplied(0, 0, 0, 40));

    // Icon emoji
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        icon,
        egui::FontId::proportional(rect.width() * 0.48),
        Color32::WHITE,
    );
}

fn draw_tooltip(painter: &egui::Painter, icon_rect: Rect, name: &str, theme: &NeoTheme) {
    let tip_w = (name.len() as f32 * 7.5 + 16.0).max(50.0);
    let tip_rect = Rect::from_center_size(
        Pos2::new(icon_rect.center().x, icon_rect.top() - 18.0),
        Vec2::new(tip_w, 22.0),
    );
    painter.rect_filled(tip_rect, Rounding::same(6.0), theme.card_bg());
    painter.rect_stroke(tip_rect, Rounding::same(6.0), egui::Stroke::new(1.0, theme.border()));
    painter.text(tip_rect.center(), egui::Align2::CENTER_CENTER, name,
        egui::FontId::proportional(11.5), theme.text());
}
