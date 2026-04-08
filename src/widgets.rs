use egui::{Color32, Pos2, Rect, Vec2, Rounding};
use crate::theme::NeoTheme;

pub struct ContextMenu {
    pub items: Vec<ContextItem>,
    pub pos: Pos2,
    pub visible: bool,
}

pub struct ContextItem {
    pub label: &'static str,
    pub icon: &'static str,
    pub action: &'static str,
    pub separator_after: bool,
}

impl ContextMenu {
    pub fn desktop_menu() -> Self {
        Self {
            pos: Pos2::ZERO,
            visible: false,
            items: vec![
                ContextItem { label: "Open Terminal", icon: "⌨", action: "terminal", separator_after: false },
                ContextItem { label: "Open File Manager", icon: "📁", action: "filemanager", separator_after: false },
                ContextItem { label: "Open Settings", icon: "⚙", action: "settings", separator_after: true },
                ContextItem { label: "Refresh Desktop", icon: "↺", action: "refresh", separator_after: false },
                ContextItem { label: "Change Wallpaper", icon: "🖼", action: "wallpaper", separator_after: true },
                ContextItem { label: "About NganjoGUI", icon: "ℹ", action: "about", separator_after: false },
            ],
        }
    }

    pub fn draw(
        &mut self,
        ctx: &egui::Context,
        theme: &NeoTheme,
        open_windows: &mut Vec<String>,
    ) -> Option<&'static str> {
        if !self.visible { return None; }

        let item_h = 32.0_f32;
        let menu_w = 200.0_f32;
        let sep_h = 8.0_f32;
        let padding = 8.0_f32;

        let sep_count = self.items.iter().filter(|i| i.separator_after).count();
        let total_h = self.items.len() as f32 * item_h + sep_count as f32 * sep_h + padding * 2.0;

        // Adjust position to stay on screen
        let screen = ctx.screen_rect();
        let x = (self.pos.x).min(screen.right() - menu_w - 4.0);
        let y = (self.pos.y).min(screen.bottom() - total_h - 50.0);

        let menu_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(menu_w, total_h));

        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("ctx_menu")));

        // Shadow
        painter.rect_filled(
            menu_rect.translate(Vec2::new(4.0, 4.0)),
            Rounding::same(10.0),
            Color32::from_black_alpha(40),
        );
        painter.rect_filled(menu_rect, Rounding::same(10.0), theme.panel_bg());
        painter.rect_stroke(menu_rect, Rounding::same(10.0), egui::Stroke::new(1.0, theme.border()));

        let mut action = None;

        egui::Area::new(egui::Id::new("ctx_menu_area"))
            .fixed_pos(menu_rect.min)
            .order(egui::Order::Tooltip)
            .show(ctx, |ui| {
                ui.set_min_size(Vec2::new(menu_w, total_h));
                ui.add_space(padding);
                for item in &self.items {
                    let (item_id, item_rect) = ui.allocate_space(Vec2::new(menu_w - 2.0, item_h));
                    let resp = ui.interact(item_rect, item_id, egui::Sense::click());

                    let bg = if resp.hovered() { theme.hover() } else { Color32::TRANSPARENT };
                    ui.painter().rect_filled(
                        item_rect.shrink2(Vec2::new(4.0, 1.0)),
                        Rounding::same(6.0),
                        bg,
                    );

                    ui.painter().text(
                        Pos2::new(item_rect.left() + 14.0, item_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        item.icon,
                        egui::FontId::proportional(14.0),
                        theme.text_dim(),
                    );
                    ui.painter().text(
                        Pos2::new(item_rect.left() + 36.0, item_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        item.label,
                        egui::FontId::proportional(13.0),
                        theme.text(),
                    );

                    if resp.clicked() {
                        action = Some(item.action);
                        if item.action == "filemanager" || item.action == "settings" {
                            if !open_windows.contains(&item.action.to_string()) {
                                open_windows.push(item.action.to_string());
                            }
                        }
                        if item.action == "terminal" {
                            let _ = std::process::Command::new("sh").arg("-c").arg("alacritty || xterm || kitty").spawn();
                        }
                        self.visible = false;
                    }

                    if item.separator_after {
                        ui.add_space(2.0);
                        ui.add(egui::Separator::default().spacing(4.0));
                        ui.add_space(2.0);
                    }
                }
                ui.add_space(padding);
            });

        // Close on click outside or Escape
        let clicked_outside = ctx.input(|i| {
            i.pointer.any_click() && !menu_rect.contains(i.pointer.hover_pos().unwrap_or(Pos2::ZERO))
        });
        if clicked_outside || ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.visible = false;
        }

        action
    }
}

pub struct DesktopIcon {
    pub label: &'static str,
    pub icon: &'static str,
    pub action: &'static str,
    pub pos: Vec2,
}

pub fn default_desktop_icons() -> Vec<DesktopIcon> {
    vec![
        DesktopIcon { label: "Home", icon: "🏠", action: "filemanager", pos: Vec2::new(20.0, 20.0) },
        DesktopIcon { label: "Terminal", icon: "⌨", action: "terminal", pos: Vec2::new(20.0, 110.0) },
        DesktopIcon { label: "Settings", icon: "⚙", action: "settings", pos: Vec2::new(20.0, 200.0) },
    ]
}

pub fn draw_desktop_icons(
    ctx: &egui::Context,
    theme: &NeoTheme,
    icons: &[DesktopIcon],
    screen_rect: Rect,
    open_windows: &mut Vec<String>,
) {
    let icon_size = Vec2::new(72.0, 72.0);
    let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("desktop_icons")));

    for icon in icons {
        let pos = Pos2::new(screen_rect.left() + icon.pos.x, screen_rect.top() + icon.pos.y);
        let rect = Rect::from_min_size(pos, icon_size);

        egui::Area::new(egui::Id::new(format!("dicon_{}", icon.label)))
            .fixed_pos(rect.min)
            .order(egui::Order::Background)
            .show(ctx, |ui| {
                let (id, r) = ui.allocate_space(icon_size);
                let resp = ui.interact(r, id, egui::Sense::click());

                let bg = if resp.hovered() { theme.hover() } else { Color32::TRANSPARENT };
                ui.painter().rect_filled(r, Rounding::same(8.0), bg);

                ui.painter().text(
                    Pos2::new(r.center().x, r.top() + 26.0),
                    egui::Align2::CENTER_CENTER,
                    icon.icon,
                    egui::FontId::proportional(28.0),
                    theme.text(),
                );
                ui.painter().text(
                    Pos2::new(r.center().x, r.top() + 54.0),
                    egui::Align2::CENTER_CENTER,
                    icon.label,
                    egui::FontId::proportional(11.0),
                    Color32::WHITE,
                );

                if resp.double_clicked() {
                    match icon.action {
                        "filemanager" | "settings" => {
                            if !open_windows.contains(&icon.action.to_string()) {
                                open_windows.push(icon.action.to_string());
                            }
                        }
                        "terminal" => {
                            let _ = std::process::Command::new("sh").arg("-c").arg("alacritty || xterm || kitty").spawn();
                        }
                        _ => {}
                    }
                }
            });
    }
}
