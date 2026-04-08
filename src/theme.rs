use egui::{Color32, Rounding, Stroke, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum ThemeMode {
    Dark,
    Light,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NeoTheme {
    pub mode: ThemeMode,
    pub accent: [u8; 3],
    pub wallpaper_idx: usize,
    pub use_dock: bool,   // true = macOS-style dock, false = taskbar
}

impl Default for NeoTheme {
    fn default() -> Self {
        Self {
            mode: ThemeMode::Dark,
            accent: [100, 180, 255],
            wallpaper_idx: 0,
            use_dock: true,
        }
    }
}

impl NeoTheme {
    pub fn bg(&self) -> Color32 {
        match self.mode {
            ThemeMode::Dark => Color32::from_rgb(12, 14, 20),
            ThemeMode::Light => Color32::from_rgb(235, 238, 245),
        }
    }

    pub fn panel_bg(&self) -> Color32 {
        match self.mode {
            ThemeMode::Dark => Color32::from_rgba_premultiplied(22, 26, 38, 220),
            ThemeMode::Light => Color32::from_rgba_premultiplied(255, 255, 255, 210),
        }
    }

    pub fn card_bg(&self) -> Color32 {
        match self.mode {
            ThemeMode::Dark => Color32::from_rgb(28, 33, 48),
            ThemeMode::Light => Color32::from_rgb(248, 250, 255),
        }
    }

    pub fn surface(&self) -> Color32 {
        match self.mode {
            ThemeMode::Dark => Color32::from_rgb(35, 41, 60),
            ThemeMode::Light => Color32::from_rgb(255, 255, 255),
        }
    }

    pub fn text(&self) -> Color32 {
        match self.mode {
            ThemeMode::Dark => Color32::from_rgb(220, 228, 248),
            ThemeMode::Light => Color32::from_rgb(18, 20, 30),
        }
    }

    pub fn text_dim(&self) -> Color32 {
        match self.mode {
            ThemeMode::Dark => Color32::from_rgb(110, 125, 160),
            ThemeMode::Light => Color32::from_rgb(120, 130, 155),
        }
    }

    pub fn accent(&self) -> Color32 {
        Color32::from_rgb(self.accent[0], self.accent[1], self.accent[2])
    }

    pub fn accent_dim(&self) -> Color32 {
        Color32::from_rgba_premultiplied(self.accent[0], self.accent[1], self.accent[2], 40)
    }

    pub fn border(&self) -> Color32 {
        match self.mode {
            ThemeMode::Dark => Color32::from_rgb(48, 58, 85),
            ThemeMode::Light => Color32::from_rgb(200, 210, 230),
        }
    }

    pub fn hover(&self) -> Color32 {
        match self.mode {
            ThemeMode::Dark => Color32::from_rgb(42, 50, 72),
            ThemeMode::Light => Color32::from_rgb(225, 232, 248),
        }
    }

    pub fn danger(&self) -> Color32 { Color32::from_rgb(255, 80, 90) }
    pub fn success(&self) -> Color32 { Color32::from_rgb(80, 220, 140) }
    pub fn warning(&self) -> Color32 { Color32::from_rgb(255, 190, 60) }

    pub fn apply(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        let t = self;

        style.visuals.window_fill = t.panel_bg();
        style.visuals.panel_fill = t.panel_bg();
        style.visuals.faint_bg_color = t.card_bg();
        style.visuals.extreme_bg_color = t.surface();
        style.visuals.code_bg_color = t.card_bg();

        style.visuals.override_text_color = Some(t.text());
        style.visuals.hyperlink_color = t.accent();

        style.visuals.widgets.noninteractive.bg_fill = t.card_bg();
        style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, t.text_dim());
        style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, t.border());
        style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);

        style.visuals.widgets.inactive.bg_fill = t.card_bg();
        style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, t.text());
        style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, t.border());
        style.visuals.widgets.inactive.rounding = Rounding::same(8.0);

        style.visuals.widgets.hovered.bg_fill = t.hover();
        style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, t.text());
        style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, t.accent());
        style.visuals.widgets.hovered.rounding = Rounding::same(8.0);

        style.visuals.widgets.active.bg_fill = t.accent();
        style.visuals.widgets.active.fg_stroke = Stroke::new(1.5, Color32::WHITE);
        style.visuals.widgets.active.bg_stroke = Stroke::new(1.5, t.accent());
        style.visuals.widgets.active.rounding = Rounding::same(8.0);

        style.visuals.selection.bg_fill = t.accent_dim();
        style.visuals.selection.stroke = Stroke::new(1.0, t.accent());

        style.visuals.window_rounding = Rounding::same(12.0);
        style.visuals.window_shadow = egui::epaint::Shadow {
            offset: Vec2::new(0.0, 8.0),
            blur: 32.0,
            spread: 0.0,
            color: Color32::from_black_alpha(80),
        };
        style.visuals.popup_shadow = egui::epaint::Shadow {
            offset: Vec2::new(0.0, 4.0),
            blur: 16.0,
            spread: 0.0,
            color: Color32::from_black_alpha(60),
        };

        style.spacing.item_spacing = Vec2::new(8.0, 6.0);
        style.spacing.window_margin = egui::Margin::same(16.0);
        style.spacing.button_padding = Vec2::new(12.0, 7.0);
        style.spacing.indent = 16.0;
        style.spacing.interact_size = Vec2::new(40.0, 32.0);
        style.spacing.scroll = egui::style::ScrollStyle::solid();

        ctx.set_style(style);
    }

    pub fn accent_presets() -> Vec<(&'static str, [u8; 3])> {
        vec![
            ("Arctic", [100, 180, 255]),
            ("Violet", [160, 100, 255]),
            ("Emerald", [60, 210, 140]),
            ("Rose", [255, 90, 130]),
            ("Amber", [255, 185, 50]),
            ("Cyan", [50, 220, 220]),
        ]
    }
}
