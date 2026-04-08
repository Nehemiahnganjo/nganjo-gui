use egui::{Color32, Pos2, Rect, Vec2, Rounding};
use std::time::{Duration, Instant};
use crate::theme::NeoTheme;

#[derive(Clone)]
pub struct Notification {
    pub title: String,
    pub body: String,
    pub kind: NotifKind,
    pub created: Instant,
    pub duration: Duration,
    pub dismissed: bool,
}

#[derive(Clone, PartialEq)]
pub enum NotifKind {
    Info,
    Success,
    Warning,
    Error,
}

impl Notification {
    pub fn info(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self { title: title.into(), body: body.into(), kind: NotifKind::Info, created: Instant::now(), duration: Duration::from_secs(5), dismissed: false }
    }
    pub fn success(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self { title: title.into(), body: body.into(), kind: NotifKind::Success, created: Instant::now(), duration: Duration::from_secs(4), dismissed: false }
    }
    pub fn warning(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self { title: title.into(), body: body.into(), kind: NotifKind::Warning, created: Instant::now(), duration: Duration::from_secs(6), dismissed: false }
    }
}

#[derive(Default)]
pub struct NotificationManager {
    pub notifications: Vec<Notification>,
}

impl NotificationManager {
    pub fn push(&mut self, notif: Notification) {
        if self.notifications.len() >= 5 {
            self.notifications.remove(0);
        }
        self.notifications.push(notif);
    }

    pub fn draw(&mut self, ctx: &egui::Context, theme: &NeoTheme, screen_rect: Rect) {
        let now = Instant::now();
        self.notifications.retain(|n| !n.dismissed && now.duration_since(n.created) < n.duration);

        let notif_w = 320.0_f32;
        let notif_h = 64.0_f32;
        let gap = 8.0_f32;
        let margin = 16.0_f32;

        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("notifications")));

        for (i, notif) in self.notifications.iter_mut().enumerate() {
            let elapsed = now.duration_since(notif.created).as_secs_f32();
            let remaining = notif.duration.as_secs_f32() - elapsed;

            // Slide in / fade out
            let slide_in = (elapsed / 0.3).min(1.0);
            let fade_out = if remaining < 0.5 { remaining / 0.5 } else { 1.0 };
            let alpha = (slide_in * fade_out * 255.0) as u8;

            let x = screen_rect.right() - notif_w - margin - (1.0 - slide_in) * 50.0;
            let y = screen_rect.top() + margin + i as f32 * (notif_h + gap);

            let rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(notif_w, notif_h));

            let bg = match notif.kind {
                NotifKind::Info => theme.panel_bg(),
                NotifKind::Success => Color32::from_rgba_premultiplied(20, 60, 40, 230),
                NotifKind::Warning => Color32::from_rgba_premultiplied(60, 40, 10, 230),
                NotifKind::Error => Color32::from_rgba_premultiplied(60, 15, 15, 230),
            };

            let accent = match notif.kind {
                NotifKind::Info => theme.accent(),
                NotifKind::Success => theme.success(),
                NotifKind::Warning => theme.warning(),
                NotifKind::Error => theme.danger(),
            };

            // Shadow
            painter.rect_filled(
                rect.translate(Vec2::new(0.0, 4.0)),
                Rounding::same(10.0),
                Color32::from_black_alpha((alpha / 4).into()),
            );

            painter.rect_filled(rect, Rounding::same(10.0), bg);
            painter.rect_stroke(rect, Rounding::same(10.0), egui::Stroke::new(1.0, theme.border()));

            // Accent bar
            painter.rect_filled(
                Rect::from_min_size(rect.min, Vec2::new(3.0, notif_h)),
                Rounding { nw: 10.0, sw: 10.0, ne: 0.0, se: 0.0 },
                accent,
            );

            // Text
            painter.text(
                Pos2::new(rect.left() + 16.0, rect.top() + 18.0),
                egui::Align2::LEFT_CENTER,
                &notif.title,
                egui::FontId::proportional(13.0),
                Color32::from_rgba_premultiplied(220, 228, 248, alpha),
            );
            painter.text(
                Pos2::new(rect.left() + 16.0, rect.top() + 38.0),
                egui::Align2::LEFT_CENTER,
                &notif.body,
                egui::FontId::proportional(11.0),
                Color32::from_rgba_premultiplied(140, 155, 190, alpha),
            );

            // Progress bar
            let progress = remaining / notif.duration.as_secs_f32();
            painter.rect_filled(
                Rect::from_min_size(
                    Pos2::new(rect.left() + 8.0, rect.bottom() - 3.0),
                    Vec2::new((notif_w - 16.0) * progress, 2.0),
                ),
                Rounding::same(1.0),
                Color32::from_rgba_premultiplied(accent.r(), accent.g(), accent.b(), alpha / 2),
            );

            // Dismiss check via Area
            egui::Area::new(egui::Id::new(format!("notif_{}", i)))
                .fixed_pos(rect.min)
                .order(egui::Order::Tooltip)
                .show(ctx, |ui| {
                    ui.set_min_size(Vec2::new(notif_w, notif_h));
                    let (_, resp_rect) = ui.allocate_space(Vec2::new(notif_w, notif_h));
                    let resp = ui.interact(resp_rect, egui::Id::new(format!("notif_click_{}", i)), egui::Sense::click());
                    if resp.clicked() {
                        notif.dismissed = true;
                    }
                });
        }
    }
}
