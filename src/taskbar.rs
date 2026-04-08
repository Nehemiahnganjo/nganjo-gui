use egui::{Color32, Pos2, Rect, Vec2, Rounding};
use chrono::Local;
use crate::theme::NeoTheme;

pub struct Taskbar {
    pub show_launcher: bool,
    pub show_settings: bool,
}

impl Default for Taskbar {
    fn default() -> Self {
        Self {
            show_launcher: false,
            show_settings: false,
        }
    }
}

impl Taskbar {
    pub fn draw(
        &mut self,
        ctx: &egui::Context,
        theme: &NeoTheme,
        screen_rect: Rect,
        open_windows: &mut Vec<String>,
    ) {
        let bar_h = 44.0;
        let bar_rect = Rect::from_min_size(
            Pos2::new(screen_rect.left(), screen_rect.bottom() - bar_h),
            Vec2::new(screen_rect.width(), bar_h),
        );

        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("taskbar")));

        // Taskbar background with blur effect simulation
        painter.rect_filled(bar_rect, 0.0, theme.panel_bg());
        painter.line_segment(
            [bar_rect.left_top(), bar_rect.right_top()],
            egui::Stroke::new(1.0, theme.border()),
        );

        egui::Area::new(egui::Id::new("taskbar_area"))
            .fixed_pos(bar_rect.min)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                ui.set_min_size(Vec2::new(screen_rect.width(), bar_h));
                ui.horizontal(|ui| {
                    ui.add_space(8.0);

                    // App launcher button
                    let btn_size = Vec2::new(36.0, 36.0);
                    let (launcher_id, launcher_rect) = ui.allocate_space(btn_size);
                    let launcher_response = ui.interact(launcher_rect, launcher_id, egui::Sense::click());

                    let btn_color = if self.show_launcher {
                        theme.accent()
                    } else if launcher_response.hovered() {
                        theme.hover()
                    } else {
                        theme.card_bg()
                    };
                    painter.rect_filled(launcher_rect, Rounding::same(8.0), btn_color);

                    // Grid icon (3x3 dots)
                    let dot_positions = [
                        (-5.0f32, -5.0f32), (0.0, -5.0), (5.0, -5.0),
                        (-5.0, 0.0), (0.0, 0.0), (5.0, 0.0),
                        (-5.0, 5.0), (0.0, 5.0), (5.0, 5.0),
                    ];
                    let center = launcher_rect.center();
                    let dot_color = if self.show_launcher { Color32::WHITE } else { theme.text() };
                    for (dx, dy) in &dot_positions {
                        painter.circle_filled(Pos2::new(center.x + dx, center.y + dy), 1.5, dot_color);
                    }

                    if launcher_response.clicked() {
                        self.show_launcher = !self.show_launcher;
                        self.show_settings = false;
                    }

                    ui.add_space(8.0);

                    // Pinned app icons
                    let pinned_apps = [
                        ("FM", "File Manager", "filemanager"),
                        ("TM", "Terminal", "terminal"),
                        ("ED", "Text Editor", "editor"),
                        ("BR", "Browser", "browser"),
                        ("ST", "Settings", "settings"),
                    ];

                    for (icon_text, label, id) in &pinned_apps {
                        let is_open = open_windows.contains(&id.to_string());
                        let (icon_id, icon_rect) = ui.allocate_space(Vec2::new(36.0, 36.0));
                        let icon_response = ui.interact(icon_rect, icon_id, egui::Sense::click());

                        let bg = if is_open {
                            theme.accent_dim()
                        } else if icon_response.hovered() {
                            theme.hover()
                        } else {
                            Color32::TRANSPARENT
                        };
                        painter.rect_filled(icon_rect, Rounding::same(8.0), bg);

                        if is_open {
                            let dot_y = icon_rect.bottom() - 3.0;
                            painter.circle_filled(
                                Pos2::new(icon_rect.center().x, dot_y),
                                2.0,
                                theme.accent(),
                            );
                        }

                        let icon_center = icon_rect.center();
                        let font = egui::FontId::monospace(10.0);
                        painter.text(icon_center, egui::Align2::CENTER_CENTER, icon_text, font, theme.text());

                        if icon_response.hovered() {
                            let tooltip_pos = Pos2::new(icon_rect.center().x - 30.0, bar_rect.top() - 28.0);
                            let tooltip_rect = Rect::from_min_size(tooltip_pos, Vec2::new(60.0, 22.0));
                            painter.rect_filled(tooltip_rect, Rounding::same(4.0), theme.card_bg());
                            painter.rect_stroke(tooltip_rect, Rounding::same(4.0), egui::Stroke::new(1.0, theme.border()));
                            painter.text(
                                tooltip_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                *label,
                                egui::FontId::proportional(11.0),
                                theme.text(),
                            );
                        }

                        if icon_response.clicked() {
                            if *id == "settings" {
                                self.show_settings = !self.show_settings;
                            } else if !is_open {
                                open_windows.push(id.to_string());
                            }
                        }

                        ui.add_space(2.0);
                    }

                    // Spacer
                    let remaining = ui.available_width() - 220.0;
                    if remaining > 0.0 {
                        ui.add_space(remaining);
                    }

                    // System tray area
                    let now = Local::now();
                    let time_str = now.format("%H:%M").to_string();
                    let date_str = now.format("%a %b %d").to_string();

                    let tray_rect = Rect::from_center_size(
                        Pos2::new(bar_rect.right() - 80.0, bar_rect.center().y),
                        Vec2::new(120.0, 36.0),
                    );
                    painter.text(
                        Pos2::new(tray_rect.center().x, tray_rect.center().y - 7.0),
                        egui::Align2::CENTER_CENTER,
                        &time_str,
                        egui::FontId::proportional(14.0),
                        theme.text(),
                    );
                    painter.text(
                        Pos2::new(tray_rect.center().x, tray_rect.center().y + 7.0),
                        egui::Align2::CENTER_CENTER,
                        &date_str,
                        egui::FontId::proportional(10.0),
                        theme.text_dim(),
                    );

                    ui.add_space(8.0);
                });
            });
    }
}
