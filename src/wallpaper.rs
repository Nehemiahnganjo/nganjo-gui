use egui::{Color32, Painter, Rect, Vec2, Pos2};
use std::f32::consts::PI;

pub struct WallpaperDef {
    pub name: &'static str,
}

pub fn wallpapers() -> Vec<WallpaperDef> {
    vec![
        WallpaperDef { name: "Aurora" },
        WallpaperDef { name: "Mesh Gradient" },
        WallpaperDef { name: "Deep Space" },
        WallpaperDef { name: "Mountain Silhouette" },
        WallpaperDef { name: "Geometric Flow" },
        WallpaperDef { name: "Neon City" },
    ]
}

pub fn draw_wallpaper(painter: &Painter, rect: Rect, idx: usize, time: f32) {
    match idx % 6 {
        0 => draw_aurora(painter, rect, time),
        1 => draw_mesh_gradient(painter, rect, time),
        2 => draw_deep_space(painter, rect, time),
        3 => draw_mountain(painter, rect, time),
        4 => draw_geometric(painter, rect, time),
        5 => draw_neon_city(painter, rect, time),
        _ => draw_aurora(painter, rect, time),
    }
}

fn draw_aurora(painter: &Painter, rect: Rect, time: f32) {
    // Deep space base
    painter.rect_filled(rect, 0.0, Color32::from_rgb(5, 8, 18));

    let w = rect.width();
    let h = rect.height();

    // Aurora bands - draw multiple semi-transparent bands
    let bands = [
        (0.3_f32, Color32::from_rgba_premultiplied(20, 200, 180, 35), 0.0),
        (0.45, Color32::from_rgba_premultiplied(80, 60, 220, 30), 1.2),
        (0.55, Color32::from_rgba_premultiplied(20, 180, 120, 25), 2.4),
        (0.65, Color32::from_rgba_premultiplied(140, 40, 200, 20), 0.8),
    ];

    for (base_y, color, phase) in &bands {
        let steps = 80;
        for i in 0..steps {
            let x = rect.left() + (i as f32 / steps as f32) * w;
            let wave = ((i as f32 / steps as f32) * PI * 3.0 + time * 0.3 + phase).sin() * 0.06;
            let cy = rect.top() + (base_y + wave) * h;
            let band_h = h * 0.12;

            // Gaussian fade
            for dy in 0..20_i32 {
                let t = dy as f32 / 20.0;
                let alpha = ((-((t - 0.5) * (t - 0.5)) / 0.1).exp() * color.a() as f32) as u8;
                let c = Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), alpha);
                painter.line_segment(
                    [Pos2::new(x, cy + t * band_h - band_h * 0.5), Pos2::new(x + w / steps as f32 + 1.0, cy + t * band_h - band_h * 0.5)],
                    egui::Stroke::new(1.0, c),
                );
            }
        }
    }

    // Stars
    let star_data: [(f32, f32, f32); 30] = [
        (0.05, 0.08, 1.8),(0.15, 0.18, 1.2),(0.25, 0.05, 2.0),(0.35, 0.12, 1.5),
        (0.45, 0.22, 1.0),(0.55, 0.07, 1.7),(0.65, 0.15, 1.3),(0.75, 0.10, 2.1),
        (0.85, 0.20, 1.4),(0.92, 0.08, 1.6),(0.08, 0.35, 1.1),(0.18, 0.28, 1.9),
        (0.30, 0.42, 1.3),(0.42, 0.30, 1.7),(0.52, 0.38, 1.2),(0.62, 0.25, 2.0),
        (0.72, 0.32, 1.5),(0.82, 0.18, 1.8),(0.90, 0.28, 1.4),(0.96, 0.40, 1.1),
        (0.12, 0.55, 2.2),(0.28, 0.62, 1.6),(0.48, 0.58, 1.3),(0.68, 0.48, 1.9),
        (0.78, 0.65, 1.4),(0.88, 0.52, 1.7),(0.02, 0.70, 1.2),(0.22, 0.75, 2.0),
        (0.58, 0.72, 1.5),(0.95, 0.68, 1.8),
    ];
    for (rx, ry, size) in &star_data {
        let twinkle = ((time * 1.5 + rx * 10.0).sin() * 0.3 + 0.7).max(0.3);
        let alpha = (200.0 * twinkle) as u8;
        painter.circle_filled(
            Pos2::new(rect.left() + rx * w, rect.top() + ry * h),
            *size,
            Color32::from_rgba_premultiplied(200, 220, 255, alpha),
        );
    }
}

fn draw_mesh_gradient(painter: &Painter, rect: Rect, time: f32) {
    let w = rect.width();
    let h = rect.height();

    // Animated mesh points
    let nodes: [(f32, f32, [u8;3]); 6] = [
        (0.1, 0.2, [30, 60, 180]),
        (0.8, 0.1, [100, 30, 200]),
        (0.95, 0.7, [20, 180, 220]),
        (0.15, 0.85, [60, 180, 100]),
        (0.5, 0.5, [150, 50, 220]),
        (0.7, 0.4, [40, 140, 255]),
    ];

    for ny in 0..30_u32 {
        for nx in 0..40_u32 {
            let px = nx as f32 / 40.0;
            let py = ny as f32 / 30.0;

            let mut r = 0.0f32;
            let mut g = 0.0f32;
            let mut b = 0.0f32;
            let mut total_w = 0.0f32;

            for (mx, my, col) in &nodes {
                let anim_mx = mx + (time * 0.2 + mx * 5.0).sin() * 0.08;
                let anim_my = my + (time * 0.15 + my * 5.0).cos() * 0.06;
                let dx = px - anim_mx;
                let dy = py - anim_my;
                let dist = (dx*dx + dy*dy).sqrt().max(0.001);
                let w_val = 1.0 / (dist * dist);
                r += col[0] as f32 * w_val;
                g += col[1] as f32 * w_val;
                b += col[2] as f32 * w_val;
                total_w += w_val;
            }

            let color = Color32::from_rgb(
                (r / total_w).min(255.0) as u8,
                (g / total_w).min(255.0) as u8,
                (b / total_w).min(255.0) as u8,
            );

            let cell_w = w / 40.0 + 1.0;
            let cell_h = h / 30.0 + 1.0;
            painter.rect_filled(
                Rect::from_min_size(
                    Pos2::new(rect.left() + px * w, rect.top() + py * h),
                    Vec2::new(cell_w, cell_h),
                ),
                0.0,
                color,
            );
        }
    }
}

fn draw_deep_space(painter: &Painter, rect: Rect, time: f32) {
    painter.rect_filled(rect, 0.0, Color32::from_rgb(3, 4, 10));
    let w = rect.width();
    let h = rect.height();

    // Nebula cloud
    for i in 0..200_u32 {
        let seed = i as f32;
        let nx = ((seed * 0.371 + 0.1) % 1.0) * w;
        let ny = ((seed * 0.618 + 0.2) % 1.0) * h;
        let nr = ((seed * 0.239) % 1.0) * w * 0.15 + 20.0;
        let pulse = (time * 0.1 + seed * 0.01).sin() * 0.2 + 0.8;
        let colors: [[u8;3]; 4] = [[80,20,140],[20,80,160],[140,40,80],[20,120,100]];
        let ci = (i % 4) as usize;
        let c = &colors[ci];
        let alpha = ((nr * 0.3 * pulse) as u8).min(15).max(3);
        painter.circle_filled(
            Pos2::new(rect.left() + nx, rect.top() + ny),
            nr,
            Color32::from_rgba_premultiplied(c[0], c[1], c[2], alpha),
        );
    }

    // Star field
    for i in 0..150_u32 {
        let seed = i as f32 * 1.618;
        let sx = ((seed * 0.419) % 1.0) * w;
        let sy = ((seed * 0.731) % 1.0) * h;
        let twinkle = ((time * (0.5 + (i as f32 % 10.0) * 0.1) + seed).sin() * 0.4 + 0.6).max(0.1);
        let size = ((seed * 0.251) % 1.0) * 1.5 + 0.5;
        let alpha = (220.0 * twinkle) as u8;
        painter.circle_filled(
            Pos2::new(rect.left() + sx, rect.top() + sy),
            size,
            Color32::from_rgba_premultiplied(200, 215, 255, alpha),
        );
    }
}

fn draw_mountain(painter: &Painter, rect: Rect, _time: f32) {
    let w = rect.width();
    let h = rect.height();

    // Sky gradient
    let sky_steps = 40;
    for i in 0..sky_steps {
        let t = i as f32 / sky_steps as f32;
        let r = (15.0 + t * 80.0) as u8;
        let g = (20.0 + t * 100.0) as u8;
        let b = (50.0 + t * 130.0) as u8;
        let y = rect.top() + t * h * 0.7;
        painter.rect_filled(
            Rect::from_min_size(Pos2::new(rect.left(), y), Vec2::new(w, h / sky_steps as f32 + 1.0)),
            0.0,
            Color32::from_rgb(r, g, b),
        );
    }

    // Mountain layers
    let layers = [
        (0.75_f32, Color32::from_rgb(18, 28, 55), 5),
        (0.65, Color32::from_rgb(28, 42, 75), 7),
        (0.55, Color32::from_rgb(40, 58, 95), 9),
    ];

    for (base_y, color, peaks) in &layers {
        let points_per_peak = 20;
        let total = peaks * points_per_peak;
        let mut pts = vec![Pos2::new(rect.left(), rect.bottom())];

        for i in 0..=total {
            let t = i as f32 / total as f32;
            let x = rect.left() + t * w;
            let peak_t = (t * *peaks as f32 * PI).sin().abs();
            let y = rect.top() + (base_y + (1.0 - peak_t) * 0.2) * h;
            pts.push(Pos2::new(x, y));
        }
        pts.push(Pos2::new(rect.right(), rect.bottom()));

        painter.add(egui::Shape::convex_polygon(pts, *color, egui::Stroke::NONE));
    }

    // Ground
    painter.rect_filled(
        Rect::from_min_size(Pos2::new(rect.left(), rect.top() + h * 0.78), Vec2::new(w, h * 0.22)),
        0.0,
        Color32::from_rgb(12, 20, 38),
    );

    // Moon
    painter.circle_filled(
        Pos2::new(rect.left() + w * 0.75, rect.top() + h * 0.18),
        h * 0.06,
        Color32::from_rgb(240, 235, 200),
    );
    painter.circle_filled(
        Pos2::new(rect.left() + w * 0.77, rect.top() + h * 0.17),
        h * 0.055,
        Color32::from_rgb(18, 26, 55),
    );
}

fn draw_geometric(painter: &Painter, rect: Rect, time: f32) {
    painter.rect_filled(rect, 0.0, Color32::from_rgb(8, 10, 18));
    let w = rect.width();
    let h = rect.height();
    let cx = rect.center();

    // Concentric hexagons
    for i in (1..=12_u32).rev() {
        let t = i as f32 / 12.0;
        let size = t * w.min(h) * 0.6;
        let rotation = time * 0.05 * if i % 2 == 0 { 1.0 } else { -1.0 };
        let alpha = (30.0 * (1.0 - t) + 10.0) as u8;

        let mut pts = Vec::new();
        for k in 0..6 {
            let angle = rotation + k as f32 * PI / 3.0;
            pts.push(Pos2::new(cx.x + angle.cos() * size, cx.y + angle.sin() * size));
        }

        let hue = (i as f32 / 12.0 * 180.0 + time * 20.0) as u32 % 360;
        let color = hsl_to_rgb(hue as f32, 0.7, 0.6);
        painter.add(egui::Shape::closed_line(
            pts,
            egui::Stroke::new(1.0, Color32::from_rgba_premultiplied(color.0, color.1, color.2, alpha)),
        ));
    }

    // Center glow
    for r in (1..=8_u32).rev() {
        let radius = r as f32 * 15.0;
        let alpha = (40 / r).max(2);
        painter.circle_filled(cx, radius, Color32::from_rgba_premultiplied(100, 160, 255, alpha as u8));
    }
}

fn draw_neon_city(painter: &Painter, rect: Rect, time: f32) {
    painter.rect_filled(rect, 0.0, Color32::from_rgb(5, 3, 15));
    let w = rect.width();
    let h = rect.height();

    // Grid
    let grid_alpha = 20u8;
    let cols = 20;
    let rows = 15;
    for c in 0..=cols {
        let x = rect.left() + c as f32 / cols as f32 * w;
        // Perspective
        let convergence = rect.left() + w * 0.5;
        let x_top = convergence + (x - convergence) * 0.3;
        painter.line_segment(
            [Pos2::new(x_top, rect.top() + h * 0.4), Pos2::new(x, rect.bottom())],
            egui::Stroke::new(0.5, Color32::from_rgba_premultiplied(0, 200, 255, grid_alpha)),
        );
    }
    for r in 0..=rows {
        let t = r as f32 / rows as f32;
        let y = rect.top() + h * 0.4 + t * h * 0.6;
        let x_spread = w * 0.5 * t;
        painter.line_segment(
            [Pos2::new(rect.left() + w * 0.5 - x_spread, y), Pos2::new(rect.left() + w * 0.5 + x_spread, y)],
            egui::Stroke::new(0.5, Color32::from_rgba_premultiplied(0, 200, 255, grid_alpha)),
        );
    }

    // Buildings
    let buildings: [(f32, f32, f32); 12] = [
        (0.02, 0.35, 0.06),(0.09, 0.28, 0.05),(0.16, 0.32, 0.07),
        (0.24, 0.22, 0.05),(0.30, 0.30, 0.06),(0.38, 0.18, 0.07),
        (0.46, 0.25, 0.05),(0.54, 0.20, 0.06),(0.62, 0.28, 0.05),
        (0.70, 0.24, 0.07),(0.78, 0.32, 0.06),(0.88, 0.28, 0.08),
    ];

    for (bx, by, bw) in &buildings {
        let building_rect = Rect::from_min_size(
            Pos2::new(rect.left() + bx * w, rect.top() + by * h),
            Vec2::new(bw * w, (0.4 - by) * h),
        );
        painter.rect_filled(building_rect, 0.0, Color32::from_rgb(8, 10, 22));
        painter.rect_stroke(building_rect, 0.0, egui::Stroke::new(0.5, Color32::from_rgba_premultiplied(0, 150, 255, 60)));

        // Windows
        let win_rows = 6;
        let win_cols = 3;
        for wr in 0..win_rows {
            for wc in 0..win_cols {
                let lit = ((bx * 100.0 + wr as f32 * 7.3 + wc as f32 * 3.1 + time * 0.5).sin() > 0.1) as u8;
                if lit > 0 {
                    let wx = building_rect.left() + (wc as f32 + 0.5) / win_cols as f32 * building_rect.width() - 2.0;
                    let wy = building_rect.top() + (wr as f32 + 0.5) / win_rows as f32 * building_rect.height() - 2.0;
                    let colors = [Color32::from_rgb(255, 220, 100), Color32::from_rgb(100, 200, 255)];
                    let ci = (wr + wc) % 2;
                    painter.rect_filled(Rect::from_min_size(Pos2::new(wx, wy), Vec2::new(4.0, 3.0)), 0.0, colors[ci]);
                }
            }
        }
    }

    // Neon reflections on ground
    painter.rect_filled(
        Rect::from_min_size(Pos2::new(rect.left(), rect.top() + h * 0.39), Vec2::new(w, h * 0.02)),
        0.0,
        Color32::from_rgba_premultiplied(20, 100, 255, 40),
    );
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r1, g1, b1) = if h < 60.0 { (c, x, 0.0) }
        else if h < 120.0 { (x, c, 0.0) }
        else if h < 180.0 { (0.0, c, x) }
        else if h < 240.0 { (0.0, x, c) }
        else if h < 300.0 { (x, 0.0, c) }
        else { (c, 0.0, x) };
    (((r1 + m) * 255.0) as u8, ((g1 + m) * 255.0) as u8, ((b1 + m) * 255.0) as u8)
}
